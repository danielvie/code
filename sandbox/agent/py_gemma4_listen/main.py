import os
import sys
import base64
import subprocess
import numpy as np
import sounddevice as sd
import ollama
import keyboard
import threading
from concurrent.futures import ThreadPoolExecutor, TimeoutError
from colorama import init, Fore, Style
from scipy.io.wavfile import write

# Initialize colorama
init(autoreset=True)

# Configuration
MODEL_NAME = "gemma4:e4b" 
SAMPLE_RATE = 16000 
CHANNELS = 1
TIMEOUT_SECONDS = 10 

class HotPTTGemma:
    def __init__(self):
        print(f"{Fore.CYAN}Ready (Optimized 'Hot' PTT).{Style.RESET_ALL}")
        self.system_prompt = (
            "You are a local PC automation assistant. Listen to the audio and provide "
            "the EXACT terminal command (PowerShell/CMD) to perform the requested task.\n"
            "- If they ask a question, answer with: echo [The Answer]\n"
            "- If they seek general information, answer with: echo [The Fact]\n"
            "- If they want to open an app, respond: start [program]\n"
            "- If the intent is unclear, just echo the transcription: echo [Transcript]\n"
            "Respond ONLY with the command text. No commentary, no backticks."
        )
        
        self.recorded_chunks = []
        self.is_recording = False
        self.is_processing = False
        self.executor = ThreadPoolExecutor(max_workers=1)
        
        self.stream = sd.InputStream(
            samplerate=SAMPLE_RATE, 
            channels=CHANNELS, 
            callback=self.audio_callback
        )

    def audio_callback(self, indata, frames, time, status):
        if self.is_recording:
            self.recorded_chunks.append(indata.copy())

    def start_recording(self):
        self.recorded_chunks = []
        self.is_recording = True
        print(f"{Fore.GREEN}[LISTENING]{Style.RESET_ALL}")

    def stop_recording(self):
        self.is_recording = False
        print(f"{Fore.YELLOW}[PROCESSING...]{Style.RESET_ALL} (10s limit)")
        
        if not self.recorded_chunks:
            return None
            
        recording = np.concatenate(self.recorded_chunks, axis=0)
        temp_filename = "hot_input.wav"
        audio_int16 = (recording * 32767).astype(np.int16)
        write(temp_filename, SAMPLE_RATE, audio_int16)
        return temp_filename

    def call_ollama(self, audio_base64):
        """Standard synchronous call to Ollama."""
        return ollama.chat(
            model=MODEL_NAME,
            messages=[
                {'role': 'system', 'content': self.system_prompt},
                {'role': 'user', 'content': 'Execute.', 'images': [audio_base64]}
            ]
        )

    def process_with_model(self, audio_path):
        self.is_processing = True
        try:
            with open(audio_path, "rb") as f:
                audio_base64 = base64.b64encode(f.read()).decode("utf-8")

            # Execute with timeout
            future = self.executor.submit(self.call_ollama, audio_base64)
            try:
                response = future.result(timeout=TIMEOUT_SECONDS)
                command = response['message']['content'].strip()
                command = command.replace("`", "").split('\n')[0]
                
                if command and len(command) > 1:
                    print(f"{Fore.BLUE}Gemma > {Style.BRIGHT}{command}{Style.RESET_ALL}")
                    subprocess.run(command, shell=True, check=True)
                else:
                    print(f"{Fore.YELLOW}No command produced.{Style.RESET_ALL}")
            except TimeoutError:
                print(f"{Fore.RED}Model timed out after {TIMEOUT_SECONDS}s.{Style.RESET_ALL}")
                
        except Exception as e:
            print(f"{Fore.RED}Internal Error: {e}{Style.RESET_ALL}")
        finally:
            self.is_processing = False
            if os.path.exists(audio_path):
                os.remove(audio_path)

    def run(self):
        print(f"\n{Fore.GREEN}=== VOICE COMMAND SYSTEM ONLINE ==={Style.RESET_ALL}")
        print(f"Model: {Style.BRIGHT}{MODEL_NAME}{Style.RESET_ALL}")
        print(f"Hold {Fore.YELLOW}Ctrl{Style.RESET_ALL} to Speak. {Fore.RED}Esc{Style.RESET_ALL} to exit.")

        with self.stream:
            try:
                while True:
                    if keyboard.is_pressed('esc'):
                        break
                    
                    if keyboard.is_pressed('ctrl') and not self.is_recording and not self.is_processing:
                        self.start_recording()
                    
                    if not keyboard.is_pressed('ctrl') and self.is_recording:
                        audio_file = self.stop_recording()
                        if audio_file:
                            threading.Thread(target=self.process_with_model, args=(audio_file,)).start()
                    
                    sd.sleep(10)
            except KeyboardInterrupt:
                pass
            
        self.executor.shutdown(wait=False)
        print(f"\n{Fore.CYAN}Shutting down...{Style.RESET_ALL}")

if __name__ == "__main__":
    app = HotPTTGemma()
    app.run()
