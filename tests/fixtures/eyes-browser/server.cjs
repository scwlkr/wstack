const http = require('node:http');
const fs = require('node:fs/promises');
const path = require('node:path');

function stateDir() {
  const evidence = process.env.WSTACK_EVIDENCE_DIR;
  if (!evidence) throw new Error('WSTACK_EVIDENCE_DIR must name this run');
  return path.join('.state', path.basename(path.dirname(evidence)));
}

async function start() {
  const file = path.join(stateDir(), 'draft.json');
  const server = http.createServer(async (request, response) => {
    try {
      if (request.url === '/' && request.method === 'GET') {
        response.setHeader('Content-Type', 'text/html');
        response.end(await fs.readFile(path.join(__dirname, 'index.html')));
      } else if (request.url === '/draft' && request.method === 'GET') {
        response.setHeader('Content-Type', 'application/json');
        response.end(await fs.readFile(file));
      } else if (request.url === '/draft' && request.method === 'POST') {
        let body = '';
        for await (const chunk of request) body += chunk;
        const title = JSON.parse(body).title.trim();
        if (!title) {
          response.writeHead(400, {'Content-Type': 'application/json'});
          response.end(JSON.stringify({error: 'title required'}));
        } else {
          if (!process.env.EYES_FIXTURE_BREAK_SAVE) {
            await fs.writeFile(file, JSON.stringify({title}));
          }
          response.setHeader('Content-Type', 'application/json');
          response.end(JSON.stringify({title}));
        }
      } else {
        response.writeHead(404);
        response.end();
      }
    } catch (error) {
      response.writeHead(500);
      response.end(String(error));
    }
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  return {server, url: `http://127.0.0.1:${server.address().port}`};
}

module.exports = {stateDir, start};
