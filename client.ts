#!ts-node

import net from "net";

export class TcpClient<I, O> {
  private _socket = new net.Socket();
  private _msgCnt = 1;

  constructor(host: string, port: number, onDataCallback: (id: number, result: O) => void) {
    this._socket.connect(port, host, function () {
      console.log("Connection established");
    });

    this._socket.on("close", function () {
      console.log("Connection closed");
    });

    this._socket.on("data", function (data) {
      // console.log("...batch");
      const new_line_seperated = data
        .toString()
        .split("\n")
        .filter((x) => x);
      for (var d of new_line_seperated) {
        const resp: { id: number; result: O } = JSON.parse(d);
        // console.log(`Received response id: ${resp.id} => ${resp.result}`);
        onDataCallback(resp.id, resp.result);
      }
      // client.destroy(); // kill client after server's response
    });
  }

  send(req: I) {
    let reqEnvelope = JSON.stringify({
      id: this._msgCnt,
      payload: req,
    });
    // console.log(`Sending envelope ${reqEnvelope}`);
    this._socket.write(`${reqEnvelope}\n`);
    return this._msgCnt++;
  }
}

export interface SumRequest {
  x: number;
  y: number;
}

const client = new TcpClient<SumRequest, any>("127.0.0.1", 3333, function (id: number, result) {
  console.log(`Response for reqId: ${id}: ${JSON.stringify(result)}`);
});

let payload: any = { x: 1, y: 2 };
console.log(`Sending ${JSON.stringify(payload)}, reqId: ${client.send(payload)}`);

payload = "bogus payload";
console.log(`Sending ${payload}, reqId: ${client.send(payload)}`);

payload = { x: 5, y: 10 };
console.log(`Sending ${JSON.stringify(payload)}, reqId: ${client.send(payload)}`);

console.log("Done sending!");
