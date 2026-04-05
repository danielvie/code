import os
import sys
import base64
import subprocess
import numpy as np
import sounddevice as sd
import keyboard
import threading
import warnings
from concurrent.futures import ThreadPoolExecutor, TimeoutError
from colorama import init, Fore, Style
from scipy.io.wavfile import write

warnings.filterwarnings("ignore", module="langgraph.*")
warnings.filterwarnings("ignore", module="langchain.*")
warnings.filterwarnings("ignore", category=DeprecationWarning)

# LangChain Imports
from langchain_ollama import ChatOllama
from langchain_core.messages import HumanMessage
from langchain_core.tools import tool
from langgraph.prebuilt import create_react_agent

# Set local model environment
os.environ["OLLAMA_API_BASE"] = "http://localhost:11434"

# Initialize colorama
init(autoreset=True)

# Configuration
MODEL_NAME = "gemma4:e4b" 
SAMPLE_RATE = 16000 
CHANNELS = 1
TIMEOUT_SECONDS = 10 

@tool
def execute_terminal_command(command: str) -> str:
    """Executes a formal terminal command (PowerShell or CMD) on the local PC."""
    print(f"{Fore.BLUE}Executing (PowerShell): {Style.BRIGHT}{command}{Style.RESET_ALL}")
    try:
        result = subprocess.run(["powershell", "-Command", command], capture_output=True, text=True)
        output = result.stdout or result.stderr or "Execution successful."
        return f"Output: {output}"
    except Exception as e:
        return f"Error executing command: {str(e)}"

class HotPTTGemma:
    def __init__(self):
        print(f"{Fore.CYAN}Ready (LangChain Powered Loop).{Style.RESET_ALL}")
        
        self.llm = ChatOllama(model=MODEL_NAME, base_url="http://localhost:11434")
        
        self.system_instruction = (
            "You are a local PC automation assistant. "
            "Analyze the user's intent from the provided text and execute the correct tool command.\n"
            "- If asked a question, use your knowledge.\n"
            "- If told to perform a PC task, use the execute_terminal_command tool.\n"
            "Respond naturally after executing tools."
        )

        self.recorded_chunks = []
        self.is_recording = False
        self.is_processing = False
        self.active_threads = []
        self.executor = ThreadPoolExecutor(max_workers=1)
        
        try:
            self.stream = sd.InputStream(
                samplerate=SAMPLE_RATE, 
                channels=CHANNELS, 
                callback=self.audio_callback
            )
        except Exception as e:
            print(f"\n{Fore.RED}Audio Device Error: {e}{Style.RESET_ALL}")
            print(f"{Fore.YELLOW}Attempting to list available audio devices:{Style.RESET_ALL}")
            try:
                print(sd.query_devices())
            except Exception:
                print("Could not query devices.")
            print(f"\n{Fore.RED}Please ensure a microphone is connected and enabled in Windows privacy settings.{Style.RESET_ALL}")
            sys.exit(1)

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

    def process_with_model(self, audio_path):
        self.is_processing = True
        try:
            with open(audio_path, "rb") as f:
                audio_bytes = f.read()
                audio_base64 = base64.b64encode(audio_bytes).decode("utf-8")

            # Phase 1: Pure Voice Transcription 
            prompt_stage1 = "Transcribe the attached audio exactly. Output only the transcription."
            transcribe_message = HumanMessage(
                content=[
                    {"type": "text", "text": prompt_stage1},
                    {"type": "image_url", "image_url": f"data:image/wav;base64,{audio_base64}"}
                ]
            )

            print(f"{Fore.GREEN}Starting Phase 1 (Transcription Pipeline)...{Style.RESET_ALL}")
            
            transcription = ""
            try:
                transcription_res = self.llm.invoke([transcribe_message])
                transcription = transcription_res.content.strip()
            except Exception as e:
                print(f"{Fore.RED}Transcription error: {e}{Style.RESET_ALL}")
                
            print(f"{Fore.YELLOW}Transcribed: '{transcription}'{Style.RESET_ALL}")
            
            if not transcription or "you have not provided" in transcription.lower():
                print(f"{Fore.RED}Transcription Phase empty.{Style.RESET_ALL}")
                return

            # Phase 2: Execution based on extracted intent
            print(f"{Fore.CYAN}--- Phase 2 (Execution Pipeline) Start ---{Style.RESET_ALL}")
            
            try:
                # We use create_react_agent to handle tool calling, with our agent tool logic.
                agent = create_react_agent(self.llm, tools=[execute_terminal_command])
                
                res = agent.invoke({"messages": [("system", self.system_instruction), ("user", f"Task generated from audio: {transcription}")]})
                
                for message in res["messages"]:
                    if message.type == "ai":
                        if getattr(message, "tool_calls", None):
                            print(f"{Fore.LIGHTBLACK_EX}[Thought]: Calling tool {message.tool_calls[0]['name']}{Style.RESET_ALL}")
                        if message.content:
                            print(f"{Fore.MAGENTA}Response: {message.content}{Style.RESET_ALL}")
                    elif message.type == "tool":
                        print(f"{Fore.CYAN}Tool Output: {message.content}{Style.RESET_ALL}")

                print(f"{Fore.CYAN}--- Interaction End ---{Style.RESET_ALL}")
            except Exception as e:
                import traceback
                traceback.print_exc()
                print(f"{Fore.RED}Agent Error: {e}{Style.RESET_ALL}")
                
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
                            t = threading.Thread(target=self.process_with_model, args=(audio_file,))
                            self.active_threads.append(t)
                            t.start()
                    
                    sd.sleep(10)
            except KeyboardInterrupt:
                pass
            
        print(f"\n{Fore.CYAN}Waiting for any active processes to finish...{Style.RESET_ALL}")
        for t in self.active_threads:
            if t.is_alive():
                t.join(timeout=1.0) # Force a clean exit without hanging forever
                
        self.executor.shutdown(wait=False)
        print(f"\n{Fore.CYAN}Shutting down...{Style.RESET_ALL}")

if __name__ == "__main__":
    app = HotPTTGemma()
    app.run()
