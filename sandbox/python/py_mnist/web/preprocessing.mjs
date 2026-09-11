// This module owns the contract between the drawing pad and the Python model.
export const IMAGE_SIZE = 28;

export function findInkBounds(rgba, width, height) {
  let left = width, top = height, right = -1, bottom = -1;
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      // The pad is opaque black with white ink. Ignore nearly black edge pixels.
      if (rgba[(y * width + x) * 4] <= 8) continue;
      left = Math.min(left, x);
      right = Math.max(right, x);
      top = Math.min(top, y);
      bottom = Math.max(bottom, y);
    }
  }
  return right < 0 ? null : { left, top, width: right - left + 1, height: bottom - top + 1 };
}

export function centerByMass(pixels) {
  let mass = 0, weightedX = 0, weightedY = 0;
  for (let y = 0; y < IMAGE_SIZE; y++) {
    for (let x = 0; x < IMAGE_SIZE; x++) {
      const value = pixels[y * IMAGE_SIZE + x];
      mass += value;
      weightedX += x * value;
      weightedY += y * value;
    }
  }
  const centered = new Float32Array(IMAGE_SIZE * IMAGE_SIZE);
  if (mass === 0) return centered;

  // MNIST digits are centered by pixel mass, not just by their bounding box.
  // Use a whole-pixel shift to preserve grayscale intensities after resizing.
  const dx = Math.round((IMAGE_SIZE - 1) / 2 - weightedX / mass);
  const dy = Math.round((IMAGE_SIZE - 1) / 2 - weightedY / mass);
  for (let y = 0; y < IMAGE_SIZE; y++) {
    for (let x = 0; x < IMAGE_SIZE; x++) {
      const newX = x + dx, newY = y + dy;
      if (newX >= 0 && newX < IMAGE_SIZE && newY >= 0 && newY < IMAGE_SIZE) {
        centered[newY * IMAGE_SIZE + newX] = pixels[y * IMAGE_SIZE + x];
      }
    }
  }
  return centered;
}

export function prepareInput(canvas) {
  // Step 1: Find the ink. A blank pad must not receive a digit prediction.
  const context = canvas.getContext("2d", { willReadFrequently: true });
  const rgba = context.getImageData(0, 0, canvas.width, canvas.height).data;
  const bounds = findInkBounds(rgba, canvas.width, canvas.height);
  if (!bounds) return null;

  // Step 2: Preserve aspect ratio and fit the cropped ink into a 20 x 20 box.
  // MNIST has a margin around its digits. Stretching to 28 x 28 would lose it.
  const small = document.createElement("canvas");
  small.width = small.height = IMAGE_SIZE;
  const smallContext = small.getContext("2d", { willReadFrequently: true });
  smallContext.fillStyle = "black";
  smallContext.fillRect(0, 0, IMAGE_SIZE, IMAGE_SIZE);
  const scale = 20 / Math.max(bounds.width, bounds.height);
  const width = Math.max(1, Math.round(bounds.width * scale));
  const height = Math.max(1, Math.round(bounds.height * scale));
  smallContext.imageSmoothingEnabled = true;
  smallContext.imageSmoothingQuality = "high";
  smallContext.drawImage(
    canvas, bounds.left, bounds.top, bounds.width, bounds.height,
    Math.floor((IMAGE_SIZE - width) / 2), Math.floor((IMAGE_SIZE - height) / 2), width, height,
  );

  // Step 3: Match ToTensor in Python: white ink = 1, black background = 0.
  const resized = smallContext.getImageData(0, 0, IMAGE_SIZE, IMAGE_SIZE).data;
  const pixels = new Float32Array(IMAGE_SIZE * IMAGE_SIZE);
  for (let i = 0; i < pixels.length; i++) pixels[i] = resized[i * 4] / 255;
  return centerByMass(pixels);
}

export function softmax(logits) {
  // Subtracting the maximum avoids overflow without changing probabilities.
  const maximum = Math.max(...logits);
  const exponentials = Array.from(logits, value => Math.exp(value - maximum));
  const total = exponentials.reduce((sum, value) => sum + value, 0);
  return exponentials.map(value => value / total);
}
