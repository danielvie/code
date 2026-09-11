# Learn MNIST with Python

This example trains a small PyTorch neural network to recognize handwritten digits from 0 to 9. Read `app/main.py` starting with `main()`. Its seven numbered steps follow the training experiment, with the weight updates written out in `train_one_epoch()`. An optional eighth step exports the model for a browser drawing demo.

MNIST contains 60,000 training images and 10,000 test images. Each is a 28 x 28 grayscale picture with a known digit label. The program uses the real MNIST dataset, not scikit-learn's smaller digits dataset.

For an illustrated walkthrough, open [the MNIST tutorial](web/tutorial.html). It explains the dataset, flattening, the neural network, training, ONNX export, and browser inference. With the demo server running, it is also available at http://127.0.0.1:8000/tutorial.html.

## Run it

Install [uv](https://docs.astral.sh/uv/getting-started/installation/), then run these commands from this directory. The project requests Python 3.13 and CPU-only PyTorch, so no GPU is needed.

```sh
uv sync
uv run -m app.main
```

The first run downloads MNIST into `data/`. Later runs reuse those files. Installation and the first dataset download require internet access. The script saves `outputs/predictions.png` instead of opening a window. Each run replaces that picture.

For a shorter smoke test:

```sh
uv run -m app.main --quick --epochs 1
```

Quick mode uses 2,000 training images and 1,000 test images. It still downloads the full dataset, and its accuracy is not a full MNIST benchmark.

To train for more passes through the data:

```sh
uv run -m app.main --epochs 5
```

If you use [Task](https://taskfile.dev/), `task run` runs the default example.

## Draw a digit in your browser

Train and export once, then start the local static file server:

```sh
uv run -m app.main --export
uv run -m app.serve
```

Open **http://127.0.0.1:8000**. Draw a single digit with a mouse, finger, or pen. The page predicts after each stroke and shows the normalized 28 x 28 input, the winning digit, and scores for all ten digits. Use **Clear pad** before drawing another number.

The export creates `web/mnist.onnx`, including the trained weights. You do not need to retrain each time you open the page. The generated model is ignored by Git, so export it again after a fresh checkout. Each export replaces the previous model. Avoid `--quick` when training a model you want to use for drawing.

`app.serve` only serves static files; it does not receive drawings or run predictions. It also sets the correct JavaScript module MIME type, which Python's generic HTTP server can get wrong on Windows. Stop it with Ctrl+C. Use `--port 8001` if port 8000 is occupied. Do not open `index.html` directly through `file://`.

With Task, use `task export` followed by `task web`.

### What is running in WebAssembly?

```text
Python + PyTorch       train weights and export the computation
          ↓
web/mnist.onnx         portable model, including learned weights
          ↓
ONNX Runtime Web       executes the model through its WASM backend
          ↑
Canvas → 28 x 28 float tensor → ten logits → softmax → displayed scores
```

The Python program itself is not compiled into a `.wasm` file. ONNX Runtime supplies the WebAssembly engine; the `.onnx` file supplies the network. No Python interpreter, GPU, or prediction API is needed in the browser.

The runtime JavaScript and WASM assets load from jsDelivr at the pinned version `1.22.0`. The model and page load from your static server. Drawings stay in browser memory. Internet access is required for the CDN assets; this is not a self-contained offline HTML file. For offline deployment, host that runtime release's assets locally and update the script URL in `web/index.html` and `wasmPaths` in `web/app.mjs` together.

To publish the demo, upload `web/`, including the generated `mnist.onnx`, to any static host serving JavaScript modules with a JavaScript MIME type. The runtime uses one WASM thread, so cross-origin isolation headers are not required. The included local server is for development, not public hosting.

### Read the browser code

- `web/index.html` contains the drawing pad, input preview, scores, and a short explanation.
- `web/preprocessing.mjs` crops the ink, preserves its aspect ratio inside a 20 x 20 box, centers it by pixel mass on a 28 x 28 image, and scales intensities to `[0, 1]`.
- `web/app.mjs` handles pointer input, loads the model once, creates an input tensor with shape `[1, 1, 28, 28]`, and runs inference.
- `web/styles.css` lays out the page for desktop and mobile.

Matching preprocessing matters. MNIST has light digits on a dark background, a margin around the digit, and centered handwriting. Resizing the entire drawing pad without cropping and centering often produces poor guesses. This preprocessing approximates MNIST's preparation; it cannot make every drawing match the training data.

Softmax scores sum to one, but they are not calibrated confidence estimates. A scribble can get a high score because this network only knows ten digit classes. A blank pad deliberately receives no prediction.

## Follow the steps

1. **Set the seed.** A fixed random seed makes experiments easier to compare. Exact results may differ across hardware and library versions.
2. **Load and scale the images.** `ToTensor()` turns byte pixels into floating-point numbers between 0 and 1. Labels remain integer digits.
3. **Create mini-batches.** Each training update uses up to 64 images. The training loader shuffles the examples each epoch.
4. **Build the model.** Flatten each image into 784 inputs, pass them through 128 hidden units and a ReLU, then produce 10 scores. This is a fully connected network, not a convolutional network.
5. **Train.** Clear old gradients, predict, calculate loss, compute new gradients, then update weights with Adam. An epoch visits every training example once.
6. **Evaluate.** Measure accuracy on the separate test set without updating weights. Accuracy is the fraction of images classified correctly.
7. **Inspect predictions.** Open `outputs/predictions.png`. Green titles mark correct predictions, red titles mark mistakes. These are the first sixteen examples in the selected test set, not hand-picked successes.

## Understand the tensors

| Value | Shape | Meaning |
| --- | --- | --- |
| Image batch | `[batch, 1, 28, 28]` | Images with one grayscale channel |
| Flattened batch | `[batch, 784]` | One row of pixels per image |
| Labels | `[batch]` | Correct digit IDs from 0 to 9 |
| Model output | `[batch, 10]` | One raw score per possible digit |

The output scores are **logits**, not probabilities. `CrossEntropyLoss` accepts these raw scores and integer labels. Adding a softmax layer before this loss would be incorrect. For a predicted digit, `argmax(dim=1)` selects the highest-scoring class directly.

Loss measures how poorly the model scores the correct labels. Accuracy counts correct guesses. They describe different things, so a lower loss does not guarantee higher accuracy on every run. Training loss is averaged over predictions made while the weights are changing; test loss uses the final weights.

The default three-epoch run should usually exceed 90% test accuracy. This is an expectation, not a guaranteed or precomputed result. Quick mode learns from much less data and can score lower.

## Experiments to try

- Change `LEARNING_RATE` in `app/main.py` from `0.001` to `0.01`. Does training loss decrease consistently?
- Change the hidden layer from 128 units to 32. Update both `Linear` layers so their dimensions still match.
- Remove `nn.ReLU()`. Without the nonlinearity, the two linear layers act like one linear transformation.
- Compare one epoch with five epochs. More training can eventually overfit, meaning it improves performance on training data without improving performance on unseen images.

For repeated model comparisons, split a validation set out of the training data and use that to choose settings. Reserve the test set for the final evaluation. Choosing settings based on repeated test scores makes that test less trustworthy.

Every training invocation starts a new model. `--export` saves its learned weights in ONNX format for inference, not as a checkpoint for resuming training.

## Tests

```sh
uv run python -m unittest discover -s tests -v
```

These checks use small synthetic tensors and do not download MNIST. They check model dimensions, pixel scaling, weight updates, evaluation metrics, image output, and numerical agreement between PyTorch and the ONNX export. The quick command above separately checks the real dataset path.

For the browser's pure preprocessing helpers, use Node 18 or newer:

```sh
node --test tests/web.test.mjs
```

For an end-to-end browser check, install Chrome and export the model first, then run:

```sh
uv run --with playwright python tests/browser_smoke.py
```

This optional test starts its own temporary static server and isolated headless Chrome. It checks a drawn zero through the real WASM runtime, the input tensor, clearing, blank input, mobile touch input, and a missing-model error. It requires internet access to jsDelivr and saves a screenshot to `outputs/web-preview.png`.

## References

- [MNIST dataset description](https://www.tensorflow.org/datasets/catalog/mnist)
- [PyTorch training tutorial](https://docs.pytorch.org/tutorials/beginner/basics/optimization_tutorial.html)
- [Torchvision MNIST loader](https://docs.pytorch.org/vision/stable/generated/torchvision.datasets.MNIST.html)
- [PyTorch ONNX export](https://docs.pytorch.org/docs/stable/onnx.html)
- [ONNX Runtime Web](https://onnxruntime.ai/docs/get-started/with-javascript/web.html)
