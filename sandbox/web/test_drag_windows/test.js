import { chromium } from 'playwright';
import { startServer } from './server.js';

const server = await startServer(0);
const { port } = server.address();
const prototypeUrl = `http://127.0.0.1:${port}/prototype.html`;
let browser;

try {
  console.log('Server ready');

  browser = await chromium.launch({ headless: true, timeout: 10000 });
  console.log('Headless Chromium launched');
  const source = await browser.newPage({ viewport: { width: 900, height: 700 } });
  source.setDefaultTimeout(5000);
  await source.goto(`${prototypeUrl}?window=A`);
  const popupPromise = source.waitForEvent('popup');
  await source.getByRole('button', { name: 'Open another window' }).click();
  const destination = await popupPromise;
  destination.setDefaultTimeout(5000);
  await destination.waitForLoadState();
  console.log('Two windows ready');

  const data = await source.getByTestId('item').evaluate(item => {
    const transfer = new DataTransfer();
    item.dispatchEvent(new DragEvent('dragstart', { bubbles: true, cancelable: true, dataTransfer: transfer }));
    return Object.fromEntries(transfer.types.map(type => [type, transfer.getData(type)]));
  });

  await destination.getByTestId('dropzone').drop({ data });
  console.log('Synthetic drop dispatched');
  const result = JSON.parse(await destination.getByTestId('state').textContent());
  if (result.status !== 'dropped' || result.id !== 'item-42' || result.sourceWindow !== 'A' || result.windowId !== 'B') {
    throw new Error(`Unexpected destination state: ${JSON.stringify(result)}`);
  }

  console.log('PASS: synthetic DataTransfer moved item-42 from window A to window B in headless Chromium');
} finally {
  await browser?.close();
  await new Promise(resolve => server.close(resolve));
}
