"""Small offline checks for the lesson; no MNIST download is needed."""

import argparse
from pathlib import Path
import tempfile
import unittest

from PIL import Image
import numpy as np
import onnx
from onnx.reference import ReferenceEvaluator
import torch
from torch import nn
from torch.utils.data import DataLoader, TensorDataset
from torchvision import transforms

from app.main import (
    build_model,
    evaluate,
    export_model,
    positive_int,
    save_predictions,
    train_one_epoch,
)


class MnistLessonTests(unittest.TestCase):
    def setUp(self):
        torch.manual_seed(42)

    def test_model_produces_ten_scores_per_image(self):
        scores = build_model()(torch.zeros(3, 1, 28, 28))
        self.assertEqual(tuple(scores.shape), (3, 10))
        self.assertTrue(torch.isfinite(scores).all())

    def test_pixels_are_scaled_and_channel_is_added(self):
        image = Image.new("L", (28, 28), color=255)
        image.putpixel((0, 0), 0)
        tensor = transforms.ToTensor()(image)
        self.assertEqual(tuple(tensor.shape), (1, 28, 28))
        self.assertEqual(tensor.dtype, torch.float32)
        self.assertEqual(tensor[0, 0, 0].item(), 0.0)
        self.assertEqual(tensor[0, 0, 1].item(), 1.0)

    def test_training_updates_weights(self):
        model = build_model()
        model.eval()
        before = [parameter.detach().clone() for parameter in model.parameters()]
        loader = DataLoader(
            TensorDataset(torch.rand(5, 1, 28, 28), torch.tensor([0, 1, 2, 3, 4])),
            batch_size=3,
        )
        optimizer = torch.optim.Adam(model.parameters(), lr=0.001)
        loss = train_one_epoch(model, loader, nn.CrossEntropyLoss(), optimizer)
        self.assertGreater(loss, 0)
        self.assertTrue(model.training)
        self.assertTrue(
            any(not torch.equal(old, new) for old, new in zip(before, model.parameters()))
        )

    def test_evaluation_weights_partial_batch_and_does_not_update_model(self):
        # Feed known logits through an identity layer. Two of three guesses
        # are correct. Uneven batches catch incorrect averaging by batch count.
        logits = torch.tensor([[4.0, 0.0], [0.0, 2.0], [3.0, 0.0]])
        labels = torch.tensor([0, 1, 1])
        loader = DataLoader(TensorDataset(logits, labels), batch_size=2)
        model = nn.Linear(2, 2, bias=False)
        with torch.no_grad():
            model.weight.copy_(torch.eye(2))
        before = model.weight.detach().clone()
        loss_function = nn.CrossEntropyLoss()

        loss, accuracy = evaluate(model, loader, loss_function)

        self.assertAlmostEqual(loss, loss_function(logits, labels).item(), places=6)
        self.assertAlmostEqual(accuracy, 2 / 3)
        self.assertFalse(model.training)
        self.assertTrue(torch.equal(before, model.weight))
        self.assertIsNone(model.weight.grad)

    def test_prediction_grid_is_saved_even_for_a_small_batch(self):
        loader = DataLoader(
            TensorDataset(torch.zeros(3, 1, 28, 28), torch.tensor([0, 1, 2])),
            batch_size=3,
        )
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "outputs" / "predictions.png"
            save_predictions(build_model(), loader, path)
            with Image.open(path) as image:
                self.assertEqual(image.format, "PNG")
                self.assertGreater(image.width, 0)
                self.assertGreater(image.height, 0)

    def test_onnx_export_matches_pytorch(self):
        model = build_model().eval()
        image = torch.rand(1, 1, 28, 28)
        with torch.no_grad():
            expected = model(image).numpy()
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "web" / "mnist.onnx"
            export_model(model, path)
            exported = onnx.load(path)
            onnx.checker.check_model(exported)
            self.assertEqual(exported.graph.input[0].name, "image")
            self.assertEqual(exported.graph.output[0].name, "logits")
            self.assertEqual([file.name for file in path.parent.iterdir()], ["mnist.onnx"])
            actual = ReferenceEvaluator(exported).run(None, {"image": image.numpy()})[0]
            np.testing.assert_allclose(actual, expected, rtol=1e-5, atol=1e-6)

    def test_epoch_count_must_be_positive(self):
        self.assertEqual(positive_int("3"), 3)
        for value in ("0", "-1"):
            with self.assertRaises(argparse.ArgumentTypeError):
                positive_int(value)


if __name__ == "__main__":
    unittest.main()
