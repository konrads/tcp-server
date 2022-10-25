#!ts-node

import net from "net";

var client = new net.Socket();
client.connect(3333, "127.0.0.1", function () {
  console.log("Connection established");
});

client.on("data", function (data) {
  console.log("...batch");
  for (var d of data
    .toString()
    .split("\n")
    .filter((x) => x)) {
    console.log(`Received: ${d}`);
  }
  // client.destroy(); // kill client after server's response
});

client.on("close", function () {
  console.log("Connection closed");
});

let reqMsq = JSON.stringify({
  id: 1,
  payload: {
    x: 1,
    y: 2,
  },
});
console.log(`Sending ${reqMsq}`);
client.write(`${reqMsq}\n`);

reqMsq = "bogus non json message";
console.log(`Sending: ${reqMsq}`);
client.write(`${reqMsq}\n`);

reqMsq = JSON.stringify({
  id: 2,
  payload: {
    x: 5,
    y: 10,
  },
});
console.log(`Sending ${reqMsq}`);
client.write(`${reqMsq}\n`);

console.log("Done sending!");
