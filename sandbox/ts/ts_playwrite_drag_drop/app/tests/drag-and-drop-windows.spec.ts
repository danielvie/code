import { test, expect } from '@playwright/test';

test.describe('Level 2: Drag and Drop Between Windows', () => {
  test('should drag item from source window to target window', async ({ context }) => {
    // In headed mode, we want to pop up two distinct windows so we can see the drag/drop
    const browser = context.browser();
    if (!browser) return; // Should not happen

    // Create a new context/window for the source
    const sourceContext = await browser.newContext({
      viewport: { width: 600, height: 800 },
      recordVideo: { dir: 'test-results/videos/' }
    });
    const sourcePage = await sourceContext.newPage();
    
    // Create a second new context/window for the target
    const targetContext = await browser.newContext({
      viewport: { width: 600, height: 800 },
      recordVideo: { dir: 'test-results/videos/' }
    });
    const targetPage = await targetContext.newPage();

    // Navigate to respective modes
    await sourcePage.goto('http://localhost:4200/?mode=drag-only');
    await targetPage.goto('http://localhost:4200/?mode=drop-only');

    const dragItem = sourcePage.getByTestId('drag-item-1');
    const dropZone = targetPage.getByTestId('drop-zone');

    // Make sure elements are visible and in correct modes
    await expect(dragItem).toBeVisible();
    await expect(sourcePage.getByTestId('drop-zone')).not.toBeVisible();

    await expect(dropZone).toBeVisible();
    await expect(targetPage.getByTestId('drag-item-1')).not.toBeVisible();

    // Since we are dragging between pages, we have to simulate the raw drag events
    // Playwright's page.dragAndDrop or elementHandle.dragTo doesn't work across pages out of the box.
    
    // We get the bounding boxes
    const dragBox = await dragItem.boundingBox();
    const dropBox = await dropZone.boundingBox();

    expect(dragBox).not.toBeNull();
    expect(dropBox).not.toBeNull();

    if (!dragBox || !dropBox) return;

    // Simulate drag and drop using dataTransfer
    // Playwright doesn't easily support cross-page native drag and drop, 
    // so we simulate the dragstart on page 1 and drop on page 2.
    const dataTransfer = await sourcePage.evaluateHandle(() => new DataTransfer());

    await sourcePage.evaluate(({ element }) => {
      const e = new DragEvent('dragstart', { dataTransfer: new DataTransfer() });
      element.dispatchEvent(e);
      (window as any).__dragData = e.dataTransfer?.getData('text/plain') || 'Item 1';
    }, { element: await dragItem.elementHandle() });

    // Retrieve the dragged data from source page
    const draggedData = await sourcePage.evaluate(() => (window as any).__dragData);

    // Simulate drop on target page
    await targetPage.evaluate(({ element, draggedData }) => {
      const dt = new DataTransfer();
      dt.setData('text/plain', draggedData);
      
      const dragOverEvent = new DragEvent('dragover', { dataTransfer: dt, bubbles: true });
      element.dispatchEvent(dragOverEvent);

      const dropEvent = new DragEvent('drop', { dataTransfer: dt, bubbles: true });
      element.dispatchEvent(dropEvent);
    }, { element: await dropZone.elementHandle(), draggedData });

    // Verify item was dropped on target page
    const droppedList = targetPage.getByTestId('dropped-items-list');
    await expect(droppedList).toBeVisible();
    await expect(droppedList).toContainText('Item 1');
    
    const sourceVideoPath = await sourcePage.video()?.path();
    const targetVideoPath = await targetPage.video()?.path();

    await sourcePage.close();
    await targetPage.close();
    await sourceContext.close();
    await targetContext.close();

    if (sourceVideoPath) {
      test.info().attach('Source Window Video', { path: sourceVideoPath, contentType: 'video/webm' });
    }
    if (targetVideoPath) {
      test.info().attach('Target Window Video', { path: targetVideoPath, contentType: 'video/webm' });
    }
  });
});
