import { test, expect } from '@playwright/test';

test.describe('Level 1: Basic Drag and Drop', () => {
  test('should drag item 1 to drop zone', async ({ page }) => {
    await page.goto('/');

    const dragItem = page.getByTestId('drag-item-1');
    const dropZone = page.getByTestId('drop-zone');

    // Make sure elements are visible
    await expect(dragItem).toBeVisible();
    await expect(dropZone).toBeVisible();

    // Perform drag and drop
    await dragItem.dragTo(dropZone);

    // Verify item was dropped
    const droppedList = page.getByTestId('dropped-items-list');
    await expect(droppedList).toBeVisible();
    await expect(droppedList).toContainText('Item 1');
  });
});
