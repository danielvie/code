import { spawn } from 'node:child_process';
import { mkdtemp, rm, mkdir, writeFile, readdir, readFile } from 'node:fs/promises';
import { createServer } from 'node:http';
import { tmpdir } from 'node:os';
import { resolve, join, basename, dirname } from 'node:path';
import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';

// A disposable Chromium profile, synthetic lesson and local download server. No login.
const browserRoot = join(process.env.LOCALAPPDATA || '', 'ms-playwright');
const installs = existsSync(browserRoot) ? (await readdir(browserRoot)).filter(name => /^chromium-\d+$/.test(name)).sort().reverse() : [];
const chromium = process.env.CHROMIUM_PATH || installs.map(name => join(browserRoot, name, 'chrome-win64', 'chrome.exe')).find(existsSync);
if (!chromium) throw new Error('Set CHROMIUM_PATH to an extension-capable Chromium executable.');
const profile = await mkdtemp(join(tmpdir(), 'lesson-saver-smoke-'));
const downloadPath = join(profile, 'downloads');
await mkdir(join(profile, 'Default'), { recursive: true });
await mkdir(downloadPath, { recursive: true });
await writeFile(join(profile, 'Default', 'Preferences'), JSON.stringify({ download: { default_directory: downloadPath, prompt_for_download: false } }));
const browser = spawn(chromium, [
  '--headless=new', '--no-first-run', '--no-default-browser-check', '--disable-background-networking',
  `--user-data-dir=${profile}`, '--remote-debugging-port=0',
  `--disable-extensions-except=${resolve('extension')}`, `--load-extension=${resolve('extension')}`, 'about:blank'
], { stdio: ['ignore', 'ignore', 'pipe'] });
const server = createServer((request, response) => {
  response.writeHead(200, { 'Content-Type': 'video/mp4', 'Content-Length': '8' });
  response.end('fixture!');
});
await new Promise(resolveListen => server.listen(0, '127.0.0.1', resolveListen));
let ws;
const errors = [];
const report = {};
try {
  const endpoint = await new Promise((resolveEndpoint, reject) => {
    let text = '';
    const timeout = setTimeout(() => reject(new Error(`Chromium did not start: ${text}`)), 20000);
    browser.once('error', reject);
    browser.stderr.on('data', chunk => {
      text += chunk;
      const match = text.match(/DevTools listening on (ws:\/\/[^\s]+)/);
      if (match) { clearTimeout(timeout); resolveEndpoint(match[1]); }
    });
  });
  ws = new WebSocket(endpoint);
  await new Promise((resolveOpen, reject) => { ws.onopen = resolveOpen; ws.onerror = reject; });
  let sequence = 0;
  const pending = new Map();
  const listeners = [];
  ws.onmessage = event => {
    const message = JSON.parse(event.data);
    if (message.id) {
      const request = pending.get(message.id);
      if (!request) return;
      pending.delete(message.id);
      clearTimeout(request.timeout);
      message.error ? request.reject(new Error(message.error.message)) : request.resolve(message.result);
    } else {
      for (const listener of listeners) listener(message);
      if (message.method === 'Runtime.exceptionThrown') errors.push(message.params.exceptionDetails.text);
    }
  };
  function call(method, params = {}, sessionId) {
    const id = ++sequence;
    return new Promise((resolveCall, reject) => {
      const timeout = setTimeout(() => { pending.delete(id); reject(new Error(`CDP timeout: ${method}`)); }, 15000);
      pending.set(id, { resolve: resolveCall, reject, timeout });
      ws.send(JSON.stringify({ id, method, params, ...(sessionId ? { sessionId } : {}) }));
    });
  }
  async function evaluate(session, expression) {
    const value = await call('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true, replMode: true }, session);
    if (value.exceptionDetails) throw new Error(value.exceptionDetails.exception?.description || value.exceptionDetails.text);
    return value.result.value;
  }
  async function attach(url) {
    const { targetId } = await call('Target.createTarget', { url });
    const { sessionId } = await call('Target.attachToTarget', { targetId, flatten: true });
    await call('Runtime.enable', {}, sessionId);
    return sessionId;
  }
  async function ready(session, expression) {
    for (let attempt = 0; attempt < 60; attempt++) {
      if (await evaluate(session, expression)) return;
      await new Promise(resolveWait => setTimeout(resolveWait, 100));
    }
    const diagnostic = await evaluate(session, `({text:document.body?.innerText,downloads:await chrome.downloads.search({}),busy:[...document.querySelectorAll('#videos button')].map(button=>({disabled:button.disabled,busy:button.getAttribute('aria-busy')}))})`);
    throw new Error(`Page not ready: ${expression}\n${JSON.stringify(diagnostic)}\nBrowser exceptions: ${JSON.stringify(errors)}`);
  }
  let worker;
  for (let attempt = 0; attempt < 60 && !worker; attempt++) {
    const { targetInfos } = await call('Target.getTargets');
    for (const target of targetInfos.filter(target => target.type === 'service_worker' && target.url.endsWith('/background.js'))) {
      const { sessionId } = await call('Target.attachToTarget', { targetId: target.targetId, flatten: true });
      await call('Runtime.enable', {}, sessionId);
      await call('Runtime.runIfWaitingForDebugger', {}, sessionId);
      if (await evaluate(sessionId, `globalThis.chrome?.runtime?.getManifest?.().name`) === 'Lesson video saver') { worker = target; break; }
    }
    if (!worker) await new Promise(resolveWait => setTimeout(resolveWait, 100));
  }
  assert.ok(worker, 'Extension service worker started');
  const origin = worker.url.slice(0, worker.url.lastIndexOf('/'));
  const ui = await attach(`${origin}/popup.html`);
  await ready(ui, `document.querySelector('#scan')?.textContent === 'Scan videos'`);
  assert.equal(await evaluate(ui, `document.querySelectorAll('input,select').length`), 0);
  report.noFolderControls = true;

  const lesson = await attach('about:blank');
  await call('Page.enable', {}, lesson);
  await call('Fetch.enable', { patterns: [{ urlPattern: '*' }] }, lesson);
  const signed = 'https://test.vimeocdn.com/m02s14.mp4?signature=a%2Fb&expires=123&x=1&x=2';
  const localVideo = `http://127.0.0.1:${server.address().port}/m02s14.mp4?signature=preserved`;
  const fixture = `<!doctype html><title>Lesson title must not become the filename</title><video preload="none" src="${signed}"></video><video preload="none" src="${localVideo}"></video><iframe srcdoc="&lt;video preload='none' src='https://test.vimeocdn.com/27c0bc14-5f5f7fc9.mp4'&gt;&lt;/video&gt;"></iframe><script>window.playerConfig={request:{files:{progressive:[{url:'https://test.vimeocdn.com/opaque?token=signed',mime:'video/mp4'}]}}};</script>`;
  listeners.push(message => {
    if (message.method !== 'Fetch.requestPaused' || message.sessionId !== lesson) return;
    const documentRequest = message.params.resourceType === 'Document';
    call('Fetch.fulfillRequest', {
      requestId: message.params.requestId, responseCode: 200,
      responseHeaders: [{ name: 'Content-Type', value: documentRequest ? 'text/html' : message.params.request.url.startsWith('https://test.vimeocdn.com/') ? 'video/mp4' : 'image/x-icon' }, { name: 'Access-Control-Allow-Origin', value: '*' }],
      body: Buffer.from(documentRequest ? fixture : '').toString('base64')
    }, lesson).catch(error => errors.push(error.message));
  });
  await call('Page.navigate', { url: 'https://learn.delligattiassociates.com/extension-test' }, lesson);
  await ready(lesson, `document.querySelector('iframe')?.contentDocument?.querySelector('video') != null`);
  const tabId = await evaluate(ui, `(await chrome.tabs.query({url:'https://learn.delligattiassociates.com/extension-test'}))[0].id`);
  await evaluate(lesson, `fetch('https://test.vimeocdn.com/network-only?signature=network').catch(()=>{});true`);
  await ready(ui, `(await chrome.storage.session.get('media:${tabId}'))['media:${tabId}']?.videos.some(video=>video.url.includes('/network-only?'))`);
  const scan = await evaluate(ui, `await chrome.runtime.sendMessage({type:'scan',tabId:${tabId}})`);
  assert.equal(scan.ok, true);
  for (const expected of [signed, localVideo, 'https://test.vimeocdn.com/27c0bc14-5f5f7fc9.mp4', 'https://test.vimeocdn.com/opaque?token=signed']) {
    assert.ok(scan.videos.some(video => video.url === expected), expected);
  }
  report.networkAndFrameDetection = true;

  // An upgrade must ignore the old custom-folder preference rather than blocking downloads.
  await evaluate(ui, `await chrome.storage.local.set({destination:{mode:'folder',name:'Old folder',subfolder:'Old/Courses'}})`);
  const uiTab = await evaluate(ui, `(await chrome.tabs.query({url:'${origin}/popup.html'}))[0].id`);
  assert.equal((await evaluate(ui, `await chrome.runtime.sendMessage({type:'scan',tabId:${uiTab}})`)).ok, false);
  await evaluate(ui, `await chrome.tabs.update(${tabId},{active:true});document.querySelector('#scan').click();`);
  await ready(ui, `document.querySelectorAll('#videos button').length >= 4`);
  const titles = await evaluate(ui, `[...document.querySelectorAll('.video-title')].map(item=>item.textContent)`);
  assert.ok(titles.includes('m02s14.mp4'));
  assert.ok(titles.includes('27c0bc14-5f5f7fc9.mp4'));
  assert.ok(!titles.some(title => title.includes('Lesson title')));
  report.originalFilenamesDisplayed = true;
  for (let index = 0; index < 2; index++) {
    await evaluate(ui, `[...document.querySelectorAll('#videos li')].find(row=>row.querySelector('.video-detail').textContent==='127.0.0.1').querySelector('button').click()`);
    await ready(ui, `(await chrome.downloads.search({url:${JSON.stringify(localVideo)},state:'complete'})).length === ${index + 1}`);
    await ready(ui, `document.querySelector('#status').textContent.startsWith('Download started: m02s14.mp4')`);
  }
  const downloaded = await evaluate(ui, `await chrome.downloads.search({url:${JSON.stringify(localVideo)},state:'complete'})`);
  assert.deepEqual(downloaded.map(file=>basename(file.filename)).sort(), ['m02s14 (1).mp4','m02s14.mp4']);
  for (const file of downloaded) {
    assert.equal(dirname(file.filename), downloadPath);
    assert.equal(await readFile(file.filename, 'utf8'), 'fixture!');
  }
  report.defaultFolderAndOriginalNames = true;
  report.duplicatesDoNotOverwrite = true;
  await call('Emulation.setDeviceMetricsOverride', { width: 560, height: 600, deviceScaleFactor: 1, mobile: false }, ui);
  const dimensions = await evaluate(ui, `({width:document.body.offsetWidth,height:document.body.offsetHeight,overflow:document.documentElement.scrollWidth>innerWidth})`);
  assert.equal(dimensions.width, 560);
  assert.ok(dimensions.height < 400, JSON.stringify(dimensions));
  assert.equal(dimensions.overflow, false);
  report.popupDimensions = dimensions;
  await call('Emulation.setDeviceMetricsOverride', { width: 560, height: dimensions.height, deviceScaleFactor: 1, mobile: false }, ui);
  await mkdir('artifacts', { recursive: true });
  const screenshot = await call('Page.captureScreenshot', { format: 'png', captureBeyondViewport: true }, ui);
  await writeFile('artifacts/popup.png', Buffer.from(screenshot.data, 'base64'));
  assert.deepEqual(errors, []);
  report.browserExceptions = errors;
  await writeFile('artifacts/smoke.json', JSON.stringify(report, null, 2));
  console.log(JSON.stringify(report, null, 2));
  console.log('Browser checks passed. Screenshot: artifacts/popup.png');
} finally {
  server.close();
  ws?.close();
  browser.kill();
  await new Promise(resolveExit => browser.exitCode !== null ? resolveExit() : browser.once('exit', resolveExit));
  await rm(profile, { recursive: true, force: true, maxRetries: 10, retryDelay: 200 });
}
