import { readFileSync } from 'node:fs';
import { createServer } from 'node:http';
import { fileURLToPath } from 'node:url';

const page = readFileSync(new URL('prototype.html', import.meta.url));

export function startServer(port = 3000) {
  return new Promise(resolve => {
    const server = createServer((request, response) => {
      if (request.url?.startsWith('/prototype.html')) {
        response.writeHead(200, { 'content-type': 'text/html' }).end(page);
      } else {
        response.writeHead(404).end('Not found');
      }
    }).listen(port, '127.0.0.1', () => resolve(server));
  });
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await startServer();
  console.log('Prototype: http://localhost:3000/prototype.html');
}
