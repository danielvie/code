import { useState } from 'react';
import { defineCatalog, createSpecStreamCompiler } from "@json-render/core";
import { schema } from "@json-render/react/schema";
import { defineRegistry, Renderer, JSONUIProvider } from "@json-render/react";
import { z } from "zod";

// 1. Define Your Catalog
// @ts-ignore
const catalog = defineCatalog(schema, {
  components: {
    Card: {
// @ts-ignore
      props: z.object({ title: z.string() }),
      description: "A card container",
    },
    Button: {
// @ts-ignore
      props: z.object({
        label: z.string(),
      }),
      description: "Clickable button",
    },
  },
  actions: {},
});

// 2. Define Your Components
const { registry } = defineRegistry(catalog, {
  components: {
    Card: ({ props, children }) => (
      <div style={{ border: '1px solid #ccc', padding: '16px', borderRadius: '8px', margin: '16px 0' }}>
        <h3 style={{ margin: '0 0 16px 0' }}>{(props as any).title}</h3>
        {children}
      </div>
    ),
    Button: ({ props }) => (
      <button style={{ padding: '8px 16px', background: '#0070f3', color: '#fff', border: 'none', borderRadius: '4px', cursor: 'pointer' }} onClick={() => alert('Clicked: ' + (props as any).label)}>
        {(props as any).label}
      </button>
    ),
  },
  actions: {},
});

function App() {
  const [spec, setSpec] = useState<any>(null);
  const [prompt, setPrompt] = useState("");
  const [loading, setLoading] = useState(false);

  const generate = async () => {
    if (!prompt.trim()) return;
    setLoading(true);
    setSpec(null); // Clear previous

    const systemPrompt = catalog.prompt() + `

CRITICAL: You are an expert UI generator. You must output an absolutely valid, complete JSON object. Do not add markdown or conversational text.
Your JSON must strictly match this shape:
{
  "root": "id-of-main-element",
  "elements": {
    "id-of-main-element": {
      "type": "Card",
      "props": { "title": "Dashboard" },
      "children": ["id-of-child"]
    },
    "id-of-child": {
      "type": "Button",
      "props": { "label": "Click Me" },
      "children": []
    }
  }
}
Do not return an empty elements object or just the root ID. Generate the elements map!
`;
    
    try {
      const res = await fetch('/api/ollama/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: "qwen3.5:9b",
          messages: [
            { role: "system", content: systemPrompt },
            { role: "user", content: "Create a UI for: " + prompt }
          ],
          stream: true
        })
      });

      console.log("Ollama API Response Status:", res.status);
      if (!res.ok) {
        console.error("Ollama API Error:", await res.text());
        return;
      }

      if (!res.body) {
         console.warn("Response body is empty!");
         return;
      }
      
      const reader = res.body.getReader();
      const decoder = new TextDecoder();
      const compiler = createSpecStreamCompiler<any>();
      
      let buffer = "";

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        
        buffer += decoder.decode(value, { stream: true });
        const lines = buffer.split('\n');
        
        // Process all complete lines
        buffer = lines.pop() || "";
        
        let chunkStr = "";
        for (const line of lines) {
          if (!line.trim()) continue;
          try {
            const data = JSON.parse(line);
            if (data.message?.content) {
              chunkStr += data.message.content;
            }
          } catch (e) {
            // Partial JSON line or error
            console.warn("Parse error on chunk line", line);
          }
        }

        if (chunkStr) {
          console.log("Passing chunk to compiler:", chunkStr);
          const { result } = compiler.push(chunkStr);
          if (result) {
            console.log("Compiler produced result:", result);
            setSpec(result);
          }
        }
      }

      // Final flush
      if (buffer.trim()) {
         try {
            const data = JSON.parse(buffer);
            if (data.message?.content) {
              const { result } = compiler.push(data.message.content);
              if (result) {
                console.log("Compiler produced final result:", result);
                setSpec(result);
              }
            }
          } catch (e) {
            console.error("Error during final flush JSON parse", buffer);
          }
      }
      
      const finalSpec = compiler.getResult();
      console.log("Final Generated Spec JSON:", finalSpec);

    } catch (e) {
      console.error("Fetch Exception:", e);
      alert("Error generating UI: " + String(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', maxWidth: '600px', margin: '40px auto' }}>
      <h1>Vercel json-render + Ollama Chat</h1>
      <p style={{ color: '#666' }}>Using model <b>qwen3.5:9b</b></p>
      
      <div style={{ display: 'flex', gap: '8px', marginBottom: '24px' }}>
        <input 
          style={{ flex: 1, padding: '8px', borderRadius: '4px', border: '1px solid #ccc' }}
          value={prompt} 
          onChange={(e) => setPrompt(e.target.value)} 
          placeholder="e.g. A card with a blue button labeled 'Checkout'"
          onKeyDown={(e) => e.key === 'Enter' && generate()}
        />
        <button disabled={loading} onClick={generate} style={{ padding: '8px 16px', background: '#333', color: '#fff', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
          {loading ? 'Generating...' : 'Generate'}
        </button>
      </div>

      {spec && typeof spec === 'object' && spec.elements ? (
        <JSONUIProvider registry={registry} initialState={{}}>
          <Renderer spec={spec} registry={registry} />
        </JSONUIProvider>
      ) : spec && typeof spec === 'object' && !spec.elements && !loading ? (
        <div style={{ padding: '16px', background: '#ffebee', color: '#c62828', borderRadius: '4px', border: '1px solid #ffcdd2' }}>
          <strong>Error: API generated an invalid or empty UI spec.</strong>
          <pre style={{ fontSize: '12px', marginTop: '8px', overflowX: 'auto' }}>
            {JSON.stringify(spec, null, 2)}
          </pre>
        </div>
      ) : null}
    </div>
  );
}

export default App;
