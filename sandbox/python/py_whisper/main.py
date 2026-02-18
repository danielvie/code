import os
import sys
import tempfile
import time
from typing import cast

import numpy as np
import scipy.io.wavfile as wav
import sounddevice as sd
import whisper
from pynput import keyboard

# | Model              | Parameters | Required VRAM | Relative Speed | Accuracy                            |
# | ------------------ | ---------- | ------------- | -------------- | ----------------------------------- |
# | tiny / tiny.en     | 39 M       | ~1 GB         | 32x            | Base level; good for clear audio.   |
# | base / base.en     | 74 M       | ~1 GB         | 16x            | Excellent for real-time apps.       |
# | small / small.en   | 244 M      | ~2 GB         | 6x             | Solid balance for technical terms.  |
# | medium / medium.en | 769 M      | ~5 GB         | 2x             | High quality; great for PhD work.   |
# | large / large-v3   | 1550 M     | ~10 GB        | 1x             | The gold standard (no .en version). |
# | turbo              | 809 M      | ~6 GB         | 8x             | Large-v3 quality but much faster.   |

print("Loading whisper model...")
model = whisper.load_model("small")


def audio_record(duration=5, fs=16000) -> np.ndarray:
    # record audio
    recording = sd.rec(int(duration * fs), samplerate=fs, channels=1, dtype="float32")

    # countdown
    for i in range(duration, 0, -1):
        sys.stdout.write(f"\rRecording: {i} seconds...")
        sys.stdout.flush()
        time.sleep(1)

    print("Recording complete. Play back....")

    sd.wait()

    return recording


def audio_record_push_to_talk(fs=16000) -> np.ndarray:
    recorded_chunks = []
    is_recording = False

    def callback(indata, frames, time, status):
        """this function is called for every audio block captured."""
        if is_recording:
            recorded_chunks.append(indata.copy())

    # Initialize the Stream
    stream = sd.InputStream(samplerate=fs, channels=1, callback=callback)

    print("--- HOLD [L_CTRL] TO RECORD | [L_ALT] TO CANCEL | [ESC] or [Q] TO QUIT --- ")

    with stream, keyboard.Events() as events:
        for event in events:
            if hasattr(event, "key") and (event.key == keyboard.Key.esc or event.key == keyboard.KeyCode.from_char("q")):
                print("\nExiting...")
                sys.exit(0)
            if hasattr(event, "key") and event.key == keyboard.Key.alt_l:
                if isinstance(event, keyboard.Events.Press) and is_recording:
                    is_recording = False
                    recorded_chunks.clear()
                    print("\rCancelled!", flush=True)
                    break
            if hasattr(event, "key") and event.key == keyboard.Key.ctrl_l:
                if isinstance(event, keyboard.Events.Press) and not is_recording:
                    is_recording = True
                    print(
                        "\rRecording... (Release L_CTRL to Stop | L_ALT to Cancel)",
                        end="",
                        flush=True,
                    )
                elif isinstance(event, keyboard.Events.Release) and is_recording:
                    is_recording = False
                    print(
                        "\rStopped! Processing...",
                        flush=True,
                    )
                    break

    # concatenate all the small chunks
    return np.concatenate(recorded_chunks, axis=0) if recorded_chunks else np.array([])


def audio_play(recording: np.ndarray, fs: int) -> None:
    sd.play(recording, fs)
    sd.wait()
    print("Playback finished")


def audio_transcribe(recording: np.ndarray, fs: int) -> str:
    with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as tmpfile:
        wav.write(tmpfile.name, fs, recording)
        tmp_path = tmpfile.name

    # result = model.transcribe(tmp_path, language="pt")
    context_prompt = "aircraft status, give me the aircraft status, closest airport, where is the closest airport, airport locations"
    # context_prompt = "status da aeronave, aeroporto mais proximo, distancia até aeroporto"
        
    # en: english
    # pt: portuguese
        
    result = model.transcribe(
        tmp_path,
        language="en",
        initial_prompt=context_prompt,
    )
    os.remove(tmp_path)

    return cast(str, result["text"])


def main():
    fs = 16000

    print("\n=== Push-to-Talk Transcriber ===")
    print("  HOLD [L_CTRL] to record")
    print("  Press [ESC] or [Q] to quit\n")

    while True:
        recording = audio_record_push_to_talk(fs=fs)

        if recording.size == 0:
            print("No audio recorded, skipping transcription.")
            continue

        text = audio_transcribe(recording, fs)
        print(f'\nTranscription: "{text}"\n')


if __name__ == "__main__":
    main()