import os
import sys
import base64
import subprocess
import numpy as np
import sounddevice as sd
import keyboard
import threading
from concurrent.futures import ThreadPoolExecutor, TimeoutError
from colorama import init, Fore, Style
from scipy.io.wavfile import write
from pydantic import BaseModel, Field

# Google ADK Imports
from google.adk.agents.llm_agent import Agent
from google.adk import Runner
from google.adk.sessions.in_memory_session_service import InMemorySessionService
from google.genai import types
from google.adk.models.lite_llm import LiteLlm

# Set local model environment
os.environ["OLLAMA_API_BASE"] = "http://localhost:11434"

# Initialize colorama
init(autoreset=True)

# Configuration
MODEL_NAME = "gemma4:e4b" 
SAMPLE_RATE = 16000 
CHANNELS = 1
TIMEOUT_SECONDS = 10 

# Tool Definitions for ADK
class TerminalCommand(BaseModel):
    command: str = Field(description="The formal terminal command to execute (PowerShell/CMD).")

def execute_terminal_command(params: TerminalCommand) -> str:
    """Executes a local PC command and returns the output."""
    print(f"{Fore.BLUE}Executing: {Style.BRIGHT}{params.command}{Style.RESET_ALL}")
    try:
        result = subprocess.run(params.command, shell=True, capture_output=True, text=True)
        output = result.stdout or result.stderr or "Execution successful."
        return f"Output: {output}"
    except Exception as e:
        return f"Error executing command: {str(e)}"

class HotPTTGemma:
    def __init__(self):
        print(f"{Fore.CYAN}Ready (ADK Powered Loop).{Style.RESET_ALL}")
        
        # Override LiteLLM defaults to stop it from silently dropping multimodal payloads
        import litellm
        litellm.model_cost[f"ollama_chat/{MODEL_NAME}"] = {"supports_vision": True, "supports_audio": True}
        
        # Initialize ADK Agent
        model = LiteLlm(model=f"ollama_chat/{MODEL_NAME}")
        self.agent = Agent(
            name="PCAssistant",
            model=model,
            instruction=(
                "You are a local PC automation assistant. "
                "Analyze the user's intent from the provided audio/text and execute the correct tool command.\n"
                "- If asked a question, use your knowledge.\n"
                "- If told to perform a PC task, use the execute_terminal_command tool.\n"
                "Respond naturally after executing tools."
            ),
            tools=[execute_terminal_command]
        )

        self.session_service = InMemorySessionService()
        self.runner = Runner(
            app_name="PCAssistant",
            agent=self.agent,
            session_service=self.session_service,
            auto_create_session=True
        )

        self.recorded_chunks = []
        self.is_recording = False
        self.is_processing = False
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

            print(f"{Fore.YELLOW}Transcribing audio natively to bypass LiteLLM format filters...{Style.RESET_ALL}")
            import ollama
            transcription = ollama.chat(
                model=MODEL_NAME,
                messages=[{
                    'role': 'user',
                    'content': 'Transcribe this audio precisely. Output only the raw transcript without any other dialogue.',
                    'images': [audio_bytes]
                }]
            )['message']['content'].strip()
            
            print(f"{Fore.GREEN}[Transcription]: {transcription}{Style.RESET_ALL}")

            if not transcription:
                print(f"{Fore.RED}Transcription was empty or unintelligible.{Style.RESET_ALL}")
                return

            # Formulating a pure text request for the ADK Agent handling logic
            prompt = f"Please analyze and execute the following task using your tools: {transcription}"
            new_message = types.Content(role="user", parts=[types.Part.from_text(text=prompt)])

            try:
                # ADK executes the reasoning loop and tool-calling automatically via Runner
                print(f"{Fore.CYAN}--- ADK Interaction Start ---{Style.RESET_ALL}")
                for event in self.runner.run(
                    user_id="local_user",
                    session_id="local_session",
                    new_message=new_message
                ):
                    if getattr(event, "content", None) and getattr(event.content, "parts", None):
                        for part in event.content.parts:
                            if getattr(part, "thought", False) and part.text:
                                print(f"{Fore.LIGHTBLACK_EX}[Thought]: {part.text.replace(chr(10), ' ')}{Style.RESET_ALL}")
                            elif getattr(part, "text", None):
                                print(f"{Fore.MAGENTA}Response: {part.text}{Style.RESET_ALL}")
                    elif getattr(event, "actions", None) and getattr(event.actions, "state_delta", None):
                        pass # Ignore verbose internal state events for visual output
                print(f"{Fore.CYAN}--- ADK Interaction End ---{Style.RESET_ALL}")
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
                            threading.Thread(target=self.process_with_model, args=(audio_file,)).start()
                    
                    sd.sleep(10)
            except KeyboardInterrupt:
                pass
            
        self.executor.shutdown(wait=False)
        print(f"\n{Fore.CYAN}Shutting down...{Style.RESET_ALL}")

if __name__ == "__main__":
    app = HotPTTGemma()
    app.run()
