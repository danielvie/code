import { IMAGE_SIZE, prepareInput, softmax } from "./preprocessing.mjs";

const canvas = document.querySelector("#drawing");
const context = canvas.getContext("2d", { willReadFrequently: true });
const preview = document.querySelector("#preview").getContext("2d");
const predictButton = document.querySelector("#predict");
const runtimeStatus = document.querySelector("#runtime-status");
const predictionStatus = document.querySelector("#prediction-status");
const prediction = document.querySelector("#prediction");
const detail = document.querySelector("#prediction-detail");
const rows = Array.from({ length: 10 }, (_, digit) => {
  const row = document.createElement("div");
  row.className = "score-row";
  row.innerHTML = `<span>${digit}</span><div class="track"><div class="fill"></div></div><span class="score-value">—</span>`;
  document.querySelector("#scores").append(row);
  return row;
});

let session = null;
let activePointer = null;
let revision = 0;
let busy = false;
let timer;
let hasInk = false;

function updateButton() {
  predictButton.disabled = !session || !hasInk || busy || activePointer !== null;
}

function resetScores(message = "Draw on the pad to begin.") {
  prediction.textContent = "?";
  detail.textContent = "Waiting for a digit";
  predictionStatus.textContent = message;
  for (const row of rows) {
    row.classList.remove("winner");
    row.querySelector(".fill").style.width = "0%";
    row.querySelector(".score-value").textContent = "—";
  }
}

function showPreview(pixels) {
  const image = preview.createImageData(IMAGE_SIZE, IMAGE_SIZE);
  for (let i = 0; i < IMAGE_SIZE * IMAGE_SIZE; i++) {
    const value = pixels ? Math.round(pixels[i] * 255) : 0;
    image.data[i * 4] = image.data[i * 4 + 1] = image.data[i * 4 + 2] = value;
    image.data[i * 4 + 3] = 255;
  }
  preview.putImageData(image, 0, 0);
}

function clearPad() {
  if (activePointer !== null && canvas.hasPointerCapture(activePointer)) {
    canvas.releasePointerCapture(activePointer);
  }
  activePointer = null;
  clearTimeout(timer);
  revision++; // An in-flight prediction for the old drawing must not reappear.
  hasInk = false;
  context.fillStyle = "black";
  context.fillRect(0, 0, canvas.width, canvas.height);
  showPreview(null);
  resetScores();
  updateButton();
}

// Step 1: Collect mouse, pen, and touch input in the same coordinate system.
// The CSS size may differ from the fixed 280 x 280 drawing buffer on mobile.
function position(event) {
  const bounds = canvas.getBoundingClientRect();
  return {
    x: (event.clientX - bounds.left) * canvas.width / bounds.width,
    y: (event.clientY - bounds.top) * canvas.height / bounds.height,
  };
}

canvas.addEventListener("pointerdown", event => {
  if (activePointer !== null || event.button !== 0) return;
  event.preventDefault();
  activePointer = event.pointerId;
  canvas.setPointerCapture(event.pointerId);
  revision++;
  clearTimeout(timer);
  resetScores("Finish your digit to see a prediction.");
  const { x, y } = position(event);
  context.strokeStyle = "white";
  context.fillStyle = "white";
  context.lineWidth = 20;
  context.lineCap = context.lineJoin = "round";
  // Draw a dot too, so a tap has visible ink even without a pointermove.
  context.beginPath();
  context.arc(x, y, context.lineWidth / 2, 0, 2 * Math.PI);
  context.fill();
  context.beginPath();
  context.moveTo(x, y);
  hasInk = true;
  updateButton();
});

canvas.addEventListener("pointermove", event => {
  if (event.pointerId !== activePointer) return;
  const { x, y } = position(event);
  context.lineTo(x, y);
  context.stroke();
});

function finishStroke(event) {
  if (event.pointerId !== activePointer) return;
  activePointer = null;
  if (canvas.hasPointerCapture(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
  showPreview(prepareInput(canvas));
  updateButton();
  // A short pause lets nearby strokes become part of the same digit.
  timer = setTimeout(predict, 200);
}
canvas.addEventListener("pointerup", finishStroke);
canvas.addEventListener("pointercancel", finishStroke);
canvas.addEventListener("lostpointercapture", finishStroke);

// Step 2: Convert pixels to a tensor, then execute the exported model in WASM.
async function predict() {
  clearTimeout(timer);
  if (!session || busy || activePointer !== null) return;
  const pixels = prepareInput(canvas);
  showPreview(pixels);
  if (!pixels) {
    hasInk = false;
    resetScores();
    updateButton();
    return;
  }
  const requestedRevision = revision;
  busy = true;
  updateButton();
  predictionStatus.textContent = "Running the model locally…";
  try {
    // Match the exported input name and PyTorch's [batch, channel, height, width].
    const input = new ort.Tensor("float32", pixels, [1, 1, IMAGE_SIZE, IMAGE_SIZE]);
    const start = performance.now();
    const output = await session.run({ image: input });
    const elapsed = performance.now() - start;
    if (requestedRevision !== revision) return;

    // Step 3: Turn logits into scores for display. Training used raw logits.
    const probabilities = softmax(output.logits.data);
    const winner = probabilities.indexOf(Math.max(...probabilities));
    prediction.textContent = String(winner);
    detail.textContent = `${(probabilities[winner] * 100).toFixed(1)}% model score`;
    probabilities.forEach((probability, digit) => {
      rows[digit].classList.toggle("winner", digit === winner);
      rows[digit].querySelector(".fill").style.width = `${probability * 100}%`;
      rows[digit].querySelector(".score-value").textContent = `${(probability * 100).toFixed(1)}%`;
    });
    predictionStatus.textContent = `Predicted ${winner}. Inference took ${elapsed.toFixed(0)} ms using WebAssembly.`;
  } catch (error) {
    console.error(error);
    if (requestedRevision === revision) resetScores("Prediction failed. Try again or reload the page.");
  } finally {
    busy = false;
    updateButton();
    // If the user drew while inference was running, process the new drawing.
    if (requestedRevision !== revision && hasInk && activePointer === null) {
      timer = setTimeout(predict, 200);
    }
  }
}

// Step 4: Load once, then reuse the session for every drawing.
async function loadModel() {
  try {
    if (!globalThis.ort) throw new Error("ONNX Runtime could not load from jsDelivr. Check your connection.");
    ort.env.wasm.wasmPaths = "https://cdn.jsdelivr.net/npm/onnxruntime-web@1.22.0/dist/";
    // One thread avoids requiring cross-origin isolation headers on the server.
    // Explicitly select WASM; this example does not use WebGPU or a remote API.
    ort.env.wasm.numThreads = 1;
    const response = await fetch("./mnist.onnx", { cache: "no-cache" });
    if (!response.ok) throw new Error("Model missing. Run uv run -m app.main --export, then reload.");
    session = await ort.InferenceSession.create(await response.arrayBuffer(), {
      executionProviders: ["wasm"],
    });
    runtimeStatus.textContent = "Ready · WebAssembly · Predictions stay in your browser";
    updateButton();
    if (hasInk) await predict();
  } catch (error) {
    console.error(error);
    runtimeStatus.classList.add("error");
    runtimeStatus.textContent = `Could not load the model. ${error.message}`;
    predictionStatus.textContent = "Check the loading error above, then reload to retry.";
  }
}

document.querySelector("#clear").addEventListener("click", clearPad);
predictButton.addEventListener("click", predict);
clearPad();
// Wait for the deferred runtime script as well as this module before loading.
if (document.readyState === "complete") loadModel();
else window.addEventListener("load", loadModel, { once: true });
