"""Learn to recognize handwritten digits with MNIST and a small neural network.

Run from the project directory: uv run -m app.main
Read main() first for the overall sequence, then the functions it calls.
"""

import argparse
from pathlib import Path

import matplotlib

# Save pictures without requiring a desktop window, including on headless machines.
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import torch
from torch import nn
from torch.utils.data import DataLoader, Subset
from torchvision import datasets, transforms


PROJECT_DIR = Path(__file__).resolve().parents[1]
DATA_DIR = PROJECT_DIR / "data"
OUTPUT_DIR = PROJECT_DIR / "outputs"
BATCH_SIZE = 64
LEARNING_RATE = 0.001
SEED = 42


def build_model() -> nn.Sequential:
    """Turn a 28 x 28 image into ten scores, one for each possible digit."""
    return nn.Sequential(
        # Input shape: [batch, 1, 28, 28]. Flatten keeps the batch dimension
        # and turns each image into a row of 784 pixel values.
        nn.Flatten(),
        nn.Linear(28 * 28, 128),
        nn.ReLU(),
        nn.Linear(128, 64),
        # ReLU replaces negative values with zero. This nonlinearity lets the
        # network learn more than a single linear mapping of pixels to scores.
        nn.ReLU(),
        nn.Linear(64, 10),
        # Output shape: [batch, 10]. These raw scores are called logits.
        # Do NOT add softmax here: CrossEntropyLoss handles that calculation.
    )


def train_one_epoch(model, loader, loss_function, optimizer) -> float:
    """Visit each training example once and return the average training loss."""
    model.train()
    total_loss = 0.0
    total_examples = 0

    for images, labels in loader:
        # A batch contains up to 64 images and their correct digit labels.
        # 1. Clear old gradients. PyTorch otherwise adds new gradients to them.
        optimizer.zero_grad()

        # 2. Forward pass: ask the current model to score each digit.
        logits = model(images)

        # 3. Measure the error. Labels are integer class IDs, not one-hot vectors.
        loss = loss_function(logits, labels)

        # 4. Backpropagation: compute how each weight contributes to the loss.
        loss.backward()

        # 5. Update the weights using those gradients and the learning rate.
        optimizer.step()

        # Weight the batch's mean loss by its size because the last batch may
        # be smaller. item() extracts a number without retaining the graph.
        total_loss += loss.item() * labels.size(0)
        total_examples += labels.size(0)

    return total_loss / total_examples


@torch.no_grad()
def evaluate(model, loader, loss_function) -> tuple[float, float]:
    """Measure loss and accuracy without gradients or weight updates."""
    # eval() changes the behavior of layers such as dropout, if added later.
    # no_grad() separately disables gradient tracking to save memory and work.
    model.eval()
    total_loss = 0.0
    correct = 0
    total_examples = 0

    for images, labels in loader:
        logits = model(images)
        total_loss += loss_function(logits, labels).item() * labels.size(0)
        # Each row has ten scores. The index of its largest score is the digit.
        predictions = logits.argmax(dim=1)
        correct += (predictions == labels).sum().item()
        total_examples += labels.size(0)

    return total_loss / total_examples, correct / total_examples


@torch.no_grad()
def save_predictions(model, loader, path: Path) -> None:
    """Save sixteen test images with their predicted and actual labels."""
    model.eval()
    images, labels = next(iter(loader))
    predictions = model(images).argmax(dim=1)
    figure, axes = plt.subplots(4, 4, figsize=(8, 8))

    for index, axis in enumerate(axes.flat):
        axis.axis("off")
        if index >= len(images):
            continue
        predicted = predictions[index].item()
        actual = labels[index].item()
        # Remove the single grayscale channel dimension for imshow.
        axis.imshow(images[index, 0].numpy(), cmap="gray", vmin=0, vmax=1)
        axis.set_title(
            f"Predicted {predicted} | Actual {actual}",
            color="green" if predicted == actual else "red",
            fontsize=9,
        )

    figure.suptitle("MNIST test predictions: green = correct, red = incorrect")
    figure.tight_layout()
    path.parent.mkdir(parents=True, exist_ok=True)
    figure.savefig(path, dpi=150)
    plt.close(figure)


def export_model(model, path: Path) -> None:
    """Save the network and learned weights for ONNX Runtime in the browser."""
    model.eval()
    path.parent.mkdir(parents=True, exist_ok=True)
    # ONNX describes the computation and stores weights. It is not Python code
    # compiled to WASM. The browser's ONNX Runtime executes it using WASM.
    # Our page predicts one image at a time, so the input shape is fixed.
    torch.onnx.export(
        model,
        (torch.zeros(1, 1, 28, 28),),
        str(path),
        input_names=["image"],
        output_names=["logits"],
        opset_version=18,
        dynamo=True,
        external_data=False,  # Keep weights inside one portable .onnx file.
        verbose=False,
    )


def positive_int(value: str) -> int:
    """Reject zero or negative epoch counts at the command line."""
    number = int(value)
    if number < 1:
        raise argparse.ArgumentTypeError("must be at least 1")
    return number


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--epochs", type=positive_int, default=3)
    parser.add_argument(
        "--quick",
        action="store_true",
        help="use 2,000 training images and 1,000 test images for a smoke test",
    )
    parser.add_argument(
        "--export",
        action="store_true",
        help="save the trained model to web/mnist.onnx for browser prediction",
    )
    args = parser.parse_args()

    # Step 1: Make runs repeatable and choose simple CPU execution.
    # The seed controls initial weights, subset selection, and batch shuffling.
    # Results can still differ across PyTorch versions and hardware.
    print("Step 1: Set the random seed. This example runs on the CPU.")
    torch.manual_seed(SEED)

    # Step 2: Download MNIST and convert images into tensors.
    # MNIST has 60,000 training images and a separate 10,000-image test set.
    # Each image is 28 x 28 grayscale pixels; each label is a digit from 0 to 9.
    # ToTensor converts pixel bytes from [0, 255] into float values in [0, 1]
    # and adds a channel dimension, giving each image shape [1, 28, 28].
    print(f"Step 2: Load MNIST into {DATA_DIR}. First run needs internet access.")
    to_tensor = transforms.ToTensor()
    training_data = datasets.MNIST(
        root=DATA_DIR, train=True, download=True, transform=to_tensor
    )
    test_data = datasets.MNIST(
        root=DATA_DIR, train=False, download=True, transform=to_tensor
    )

    if args.quick:
        # Both subsets still come from separate original splits. A dedicated
        # generator makes subset selection independent of other random draws.
        generator = torch.Generator().manual_seed(SEED)
        training_data = Subset(
            training_data,
            torch.randperm(len(training_data), generator=generator)[:2000].tolist(),
        )
        test_data = Subset(
            test_data,
            torch.randperm(len(test_data), generator=generator)[:1000].tolist(),
        )
        print("Quick mode: smaller subsets, not a full benchmark.")

    # Step 3: Group images into mini-batches instead of processing all at once.
    # Shuffle training examples so batches change each epoch. Keep the test
    # order fixed. num_workers=0 avoids multiprocessing setup in this lesson.
    print(f"Step 3: Batch {len(training_data):,} training and {len(test_data):,} test images.")
    training_loader = DataLoader(
        training_data, batch_size=BATCH_SIZE, shuffle=True, num_workers=0
    )
    test_loader = DataLoader(
        test_data, batch_size=BATCH_SIZE, shuffle=False, num_workers=0
    )

    # Step 4: Define the network, the error measure, and the update rule.
    # Cross entropy penalizes assigning a low probability to the correct digit.
    # Adam adjusts the weights; the learning rate sets the scale of its steps.
    print("Step 4: Build a 784 -> 128 -> 10 neural network.")
    model = build_model()
    loss_function = nn.CrossEntropyLoss()
    optimizer = torch.optim.Adam(model.parameters(), lr=LEARNING_RATE)
    print(model)

    # Step 5: Learn only from the training set. An epoch is one complete pass.
    print("Step 5: Train. Lower loss generally means better training predictions.")
    for epoch in range(1, args.epochs + 1):
        training_loss = train_one_epoch(model, training_loader, loss_function, optimizer)
        print(f"  Epoch {epoch}/{args.epochs} | Training loss: {training_loss:.4f}")

    # Step 6: Test on images the model did not train on.
    # Keep this final test separate from training. To tune the architecture or
    # learning rate, first reserve a validation split from the training data.
    print("Step 6: Evaluate on the held-out test set.")
    test_loss, test_accuracy = evaluate(model, test_loader, loss_function)
    print(f"  Test loss: {test_loss:.4f} | Test accuracy: {test_accuracy:.2%}")

    # Step 7: Look at individual predictions, not just the average score.
    prediction_path = OUTPUT_DIR / "predictions.png"
    save_predictions(model, test_loader, prediction_path)
    print(f"Step 7: Open {prediction_path} to inspect predictions.")

    # Step 8, optional: deploy the learned model, without a Python runtime.
    if args.export:
        model_path = PROJECT_DIR / "web" / "mnist.onnx"
        export_model(model, model_path)
        print(f"Step 8: Exported browser model to {model_path}.")


# Importing this module for tests must not download data or start training.
if __name__ == "__main__":
    main()
