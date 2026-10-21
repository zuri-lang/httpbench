// Node.js HTTP/1.1 and HTTP/2 Server Implementation for httpbench

import http from 'node:http';
import http2 from 'node:http2';

const port = parseInt(process.argv[2] || '9000', 10);
const proto = process.argv[3] || 'http1';

function handleHttp1(req, res) {
  const url = req.url;
  if (url === '/health') {
    res.writeHead(200, { 'content-type': 'text/plain' });
    res.end('ok');
  } else if (url === '/plaintext') {
    res.writeHead(200, { 'content-type': 'text/plain' });
    res.end('Hello, World!');
  } else if (url === '/json') {
    res.writeHead(200, { 'content-type': 'application/json' });
    res.end(JSON.stringify({ message: 'Hello, World!' }));
  } else if (url === '/echo' && req.method === 'POST') {
    res.writeHead(200, { 'content-type': 'application/octet-stream' });
    req.pipe(res);
  } else {
    res.writeHead(404, { 'content-type': 'text/plain' });
    res.end('Not Found');
  }
}

function handleHttp2(stream, headers) {
  const path = headers[':path'];
  const method = headers[':method'];

  if (path === '/health') {
    stream.respond({
      ':status': 200,
      'content-type': 'text/plain'
    });
    stream.end('ok');
  } else if (path === '/plaintext') {
    stream.respond({
      ':status': 200,
      'content-type': 'text/plain'
    });
    stream.end('Hello, World!');
  } else if (path === '/json') {
    stream.respond({
      ':status': 200,
      'content-type': 'application/json'
    });
    stream.end(JSON.stringify({ message: 'Hello, World!' }));
  } else if (path === '/echo' && method === 'POST') {
    stream.respond({
      ':status': 200,
      'content-type': 'application/octet-stream'
    });
    stream.pipe(stream);
  } else {
    stream.respond({
      ':status': 404,
      'content-type': 'text/plain'
    });
    stream.end('Not Found');
  }
}

if (proto === 'http2') {
  const server = http2.createServer();
  server.on('stream', handleHttp2);
  server.listen(port, () => {
    console.log(`Node HTTP/2 server listening on :${port}`);
  });
} else {
  const server = http.createServer(handleHttp1);
  server.listen(port, () => {
    console.log(`Node HTTP/1.1 server listening on :${port}`);
  });
}

