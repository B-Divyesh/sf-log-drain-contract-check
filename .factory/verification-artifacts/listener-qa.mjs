import { spawn } from 'node:child_process';
import { mkdtempSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { createServer, connect } from 'node:net';
import { request } from 'node:http';

const root = readFileSync('/work/repo/.factory/verification-artifacts/consumer-root.txt', 'utf8').trim();
const binary = join(root, 'bin', 'drain-check');

async function freePort() {
  const server = createServer();
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  const port = server.address().port;
  await new Promise((resolve) => server.close(resolve));
  return port;
}

function post(port, body) {
  return new Promise((resolve, reject) => {
    const req = request({ host: '127.0.0.1', port, method: 'POST', path: '/', headers: { 'content-length': Buffer.byteLength(body) } }, (res) => {
      let responseBody = '';
      res.setEncoding('utf8');
      res.on('data', (chunk) => responseBody += chunk);
      res.on('end', () => resolve({ status: res.statusCode, retryAfter: res.headers['retry-after'] ?? null, body: responseBody }));
    });
    req.on('error', reject);
    req.end(body);
  });
}

function shortBody(port) {
  return new Promise((resolve, reject) => {
    const socket = connect(port, '127.0.0.1');
    let response = '';
    socket.setEncoding('utf8');
    socket.on('connect', () => socket.end('POST / HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: 100\r\n\r\n{"short":true}'));
    socket.on('data', (chunk) => response += chunk);
    socket.on('end', () => resolve({ status: Number(response.match(/^HTTP\/1\.1 (\d+)/)?.[1]), raw: response }));
    socket.on('error', reject);
  });
}

async function runListener(extraArgs, action) {
  const port = await freePort();
  const directory = mkdtempSync(join(tmpdir(), 'drain-check-independent-'));
  const output = join(directory, 'report.json');
  const resolvedArgs = extraArgs.map((argument) => argument.replace('$DIR', directory));
  const child = spawn(binary, ['listen', '--duration', '4', '--port', String(port), '--output', output, ...resolvedArgs], { stdio: ['ignore', 'pipe', 'pipe'] });
  let stdout = '';
  let stderr = '';
  child.stdout.on('data', (chunk) => stdout += chunk);
  child.stderr.on('data', (chunk) => stderr += chunk);
  const deadline = Date.now() + 3000;
  while (!stderr.includes('listens only on') && Date.now() < deadline) await new Promise((resolve) => setTimeout(resolve, 20));
  if (!stderr.includes('listens only on')) throw new Error(`listener did not start: ${stderr}`);
  const portHex = port.toString(16).toUpperCase().padStart(4, '0');
  const bound = readFileSync('/proc/net/tcp', 'utf8').split('\n').filter((line) => line.includes(`0100007F:${portHex}`));
  const result = await action({ port, directory });
  child.kill('SIGINT');
  const exit = await new Promise((resolve) => child.on('exit', (code, signal) => resolve({ code, signal })));
  return { port, directory, output, bound, result, exit, stdout, stderr, report: JSON.parse(readFileSync(output, 'utf8')) };
}

const rateLimit = await runListener([], async ({ port }) => {
  const responses = await Promise.all(Array.from({ length: 21 }, (_, index) => post(port, JSON.stringify({ index }))));
  return {
    statuses: responses.reduce((counts, response) => ({ ...counts, [response.status]: (counts[response.status] ?? 0) + 1 }), {}),
    limited: responses.filter((response) => response.status === 429),
  };
});

const recovery = await runListener(['--rate-limit', '100'], async ({ port }) => ({
  first: await post(port, '{"accepted":1}\n'),
  malformed: await post(port, '{not-json}\n'),
  short: await shortBody(port),
  second: await post(port, '{"accepted":2}\n'),
}));

const saved = await runListener(['--rate-limit', '100', '--save-sample', '$DIR/accepted.ndjson'], async ({ port, directory }) => ({
  malformed: await post(port, '{not-json}\n'),
  accepted: await post(port, '{"saved":true}\n'),
  samplePath: join(directory, 'accepted.ndjson'),
}));
saved.savedBody = readFileSync(join(saved.directory, 'accepted.ndjson'), 'utf8');

console.log(JSON.stringify({ rateLimit, recovery, saved }, null, 2));
