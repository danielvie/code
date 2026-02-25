import { test, expect } from '@playwright/test';

test.describe('Level 3: Random Elements by Text', () => {
  test('should find randomly placed element by text and drag it', async ({ page }) => {
    // Navigate to random mode
    await page.goto('/?mode=random');

    // Wait for the random area to be visible
    const randomArea = page.getByTestId('random-area');
    await expect(randomArea).toBeVisible();

    // Find the specific element by its text
    const targetDraggable = page.getByText('Find Me To Drag');
    
    // Ensure it exists and is visible
    await expect(targetDraggable).toBeVisible();
    
    // The drop zone
    const dropZone = page.getByTestId('drop-zone');
    await expect(dropZone).toBeVisible();

    // Perform drag and drop using evaluate to be robust
    await page.evaluate(({ dragEl, dropEl }) => {
      if (!dragEl || !dropEl) return;
      
      const dataTransfer = new DataTransfer();
      
      const dragStartEvent = new DragEvent('dragstart', { dataTransfer, bubbles: true });
      dragEl.dispatchEvent(dragStartEvent);
      
      const itemText = dragStartEvent.dataTransfer?.getData('text/plain') || 'Find Me To Drag';
      
      const dt = new DataTransfer();
      dt.setData('text/plain', itemText);
      
      const dragOverEvent = new DragEvent('dragover', { dataTransfer: dt, bubbles: true });
      dropEl.dispatchEvent(dragOverEvent);

      const dropEvent = new DragEvent('drop', { dataTransfer: dt, bubbles: true });
      dropEl.dispatchEvent(dropEvent);
    }, { 
      dragEl: await targetDraggable.elementHandle(), 
      dropEl: await dropZone.elementHandle() 
    });

    // Verify it was dropped
    const droppedList = page.getByTestId('dropped-items-list');
    await expect(droppedList).toBeVisible();
    await expect(droppedList).toContainText('Find Me To Drag');
  });
});
