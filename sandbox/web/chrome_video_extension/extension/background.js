import { mediaKind, normalizeCandidates } from './media.js';
import { inspectFrame } from './scanner.js';
import { downloadOptions } from './downloads.js';

const api = globalThis.browser ?? globalThis.chrome;
const lessonHost = 'learn.delligattiassociates.com';
const cacheKey = tabId => `media:${tabId}`;
let pending = Promise.resolve();

function isLesson(url) {
  try { return new URL(url).origin === `https://${lessonHost}`; }
  catch { return false; }
}

function updateCache(action) {
  pending = pending.then(action, action).catch(() => {});
  return pending;
}

// Observe metadata only. Do not read cookies, request bodies, or video contents.
// https://developer.chrome.com/docs/extensions/reference/api/webRequest
api.webRequest.onHeadersReceived.addListener(details => {
  if (details.tabId < 0 || details.statusCode < 200 || details.statusCode >= 300) return;
  const mime = details.responseHeaders?.find(h => h.name.toLowerCase() === 'content-type')?.value || '';
  if (!['file', 'stream'].includes(mediaKind(details.url, mime))) return;
  updateCache(async () => {
    const tab = await api.tabs.get(details.tabId);
    if (!isLesson(tab.url)) return;
    const key = cacheKey(tab.id);
    const previous = (await api.storage.session.get(key))[key];
    const videos = previous?.pageUrl === tab.url ? previous.videos : [];
    const combined = normalizeCandidates([...videos, { url: details.url, mime, title: tab.title }]);
    await api.storage.session.set({ [key]: { pageUrl: tab.url, videos: combined.slice(-150) } });
  });
}, { urls: ['<all_urls>'] }, ['responseHeaders']);

api.tabs.onUpdated.addListener((tabId, change) => {
  if (change.status === 'loading' || change.url) updateCache(() => api.storage.session.remove(cacheKey(tabId)));
});
api.tabs.onRemoved.addListener(tabId => updateCache(() => api.storage.session.remove(cacheKey(tabId))));

async function scan(tabId) {
  const tab = await api.tabs.get(tabId);
  if (!isLesson(tab.url)) throw new Error('Open a lesson at learn.delligattiassociates.com, then open this extension.');
  const frames = await api.webNavigation.getAllFrames({ tabId });
  let skippedFrames = 0;
  const results = await Promise.all((frames || [{ frameId: 0 }]).map(async frame => {
    try {
      const injection = await api.scripting.executeScript({
        target: { tabId, frameIds: [frame.frameId] },
        world: 'MAIN',
        func: inspectFrame
      });
      return injection.flatMap(result => result.result || []);
    } catch {
      skippedFrames += 1;
      return [];
    }
  }));
  await pending;
  const cached = (await api.storage.session.get(cacheKey(tabId)))[cacheKey(tabId)];
  // Do not combine results across lesson navigation while the scan was running.
  if ((await api.tabs.get(tabId)).url !== tab.url) throw new Error('The page changed during scanning. Scan again.');
  const observed = cached?.pageUrl === tab.url ? cached.videos : [];
  return { videos: normalizeCandidates([...results.flat(), ...observed]), skippedFrames };
}

api.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (sender.id !== api.runtime.id || !sender.url?.startsWith(api.runtime.getURL(''))) return;
  (async () => {
    if (message.type === 'scan') return scan(message.tabId);
    if (message.type === 'download') {
      const id = await api.downloads.download(downloadOptions(message.video));
      return { id };
    }
    throw new Error('Unknown extension action.');
  })().then(value => sendResponse({ ok: true, ...value }), error => sendResponse({ ok: false, error: error.message }));
  return true;
});
