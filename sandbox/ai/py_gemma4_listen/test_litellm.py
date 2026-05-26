import litellm
import base64
litellm.model_cost['ollama_chat/gemma4:e4b'] = {'supports_vision': True, 'supports_audio': True}
audio_bytes = open('sample.wav', 'rb').read()
base64_str = base64.b64encode(audio_bytes).decode('utf-8')
try:
    response = litellm.completion(
        model='ollama_chat/gemma4:e4b',
        messages=[{
            'role': 'user',
            'content': [
                {'type': 'text', 'text': 'Transcribe this exactly.'},
                {'type': 'image_url', 'image_url': {'url': f'data:image/wav;base64,{base64_str}'}}
            ]
        }],
        api_base='http://localhost:11434'
    )
    print(response.choices[0].message.content)
except Exception as e:
    print(e)
