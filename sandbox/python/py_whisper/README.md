
# models

| Model              | Parameters | Required VRAM | Relative Speed | Accuracy                            |
| ------------------ | ---------- | ------------- | -------------- | ----------------------------------- |
| tiny / tiny.en     | 39 M       | ~1 GB         | 32x            | Base level; good for clear audio.   |
| base / base.en     | 74 M       | ~1 GB         | 16x            | Excellent for real-time apps.       |
| small / small.en   | 244 M      | ~2 GB         | 6x             | Solid balance for technical terms.  |
| medium / medium.en | 769 M      | ~5 GB         | 2x             | High quality; great for PhD work.   |
| large / large-v3   | 1550 M     | ~10 GB        | 1x             | The gold standard (no .en version). |
| turbo              | 809 M      | ~6 GB         | 8x             | Large-v3 quality but much faster.   |