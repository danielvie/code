import sounddevice as sd
from scipy.io.wavfile import write
import numpy as np
import sys
from colorama import init, Fore, Style

init(autoreset=True)

SAMPLE_RATE = 16000
DURATION = 5  # seconds
OUTPUT_FILE = "sample.wav"

try:
    print(f"{Fore.CYAN}Recording for {DURATION} seconds... Please speak clearly into your mic!{Style.RESET_ALL}")
    # Record audio
    recording = sd.rec(int(DURATION * SAMPLE_RATE), samplerate=SAMPLE_RATE, channels=1, dtype='float32')
    # Wait until recording is finished
    sd.wait()
    print(f"{Fore.GREEN}Recording complete.{Style.RESET_ALL}")

    # Scale to int16 for WAV compatibility and write
    audio_int16 = (recording * 32767).astype(np.int16)
    write(OUTPUT_FILE, SAMPLE_RATE, audio_int16)
    print(f"{Fore.YELLOW}Saved generated audio to: {OUTPUT_FILE}{Style.RESET_ALL}")

except Exception as e:
    print(f"{Fore.RED}Audio Recording Error: {e}{Style.RESET_ALL}")
    print("\nAvailable Devices:")
    try:
        print(sd.query_devices())
    except:
        pass
    sys.exit(1)
