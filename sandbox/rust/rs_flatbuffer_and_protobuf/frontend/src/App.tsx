import { useEffect, useState, useRef, createContext, useContext } from 'react';
import { sensor_proto } from './generated/sensor_proto';
import { SensorList } from './generated/sensor-fbs';
import { ByteBuffer } from 'flatbuffers';
import './index.css';

const FilterContext = createContext({ enabled: false, tau: 100 });

class LowPassFilter {
  private x = 0; // Position/Value
  private v = 0; // Velocity/Rate of Change

  process(target: number, tau: number, enabled: boolean): number {
    const dt = 16.667 / 1000; // Delta time in seconds (60Hz)
    const t = tau / 1000;      // Tau in seconds

    if (!enabled || t < dt) {
      this.x = target;
      this.v = 0;
      return target;
    }

    // 2nd Order Dynamical System (Critical Damping: zeta = 1.0)
    // Formula: d2x/dt2 = (1/tau^2) * (target - x) - (2/tau) * dx/dt
    const accel = (1 / (t * t)) * (target - this.x) - (2 / t) * this.v;
    
    // Simple Euler Integration
    this.v += accel * dt;
    this.x += this.v * dt;

    return this.x;
  }
}

function ProtoClient(props: { onReset: () => void }) {
  const { enabled: filterEnabled, tau } = useContext(FilterContext);
  const [stats, setStats] = useState({ size: 0, time: 0, count: 0 });
  const [status, setStatus] = useState<'CONNECTING' | 'CONNECTED' | 'DISCONNECTED' | 'ERROR'>('CONNECTING');
  const setStatsRef = useRef(setStats);
  setStatsRef.current = setStats;
  const wsRef = useRef<WebSocket | null>(null);
  const processingRef = useRef(false);
  const decoderRef = useRef(new TextDecoder());
  const timeFilter = useRef(new LowPassFilter());
  const sizeFilter = useRef(new LowPassFilter());
  const paramsRef = useRef({ tau, filterEnabled });

  useEffect(() => {
    paramsRef.current = { tau, filterEnabled };
  }, [tau, filterEnabled]);

  useEffect(() => {
    const ws = new WebSocket('ws://127.0.0.1:8080');
    ws.binaryType = 'arraybuffer';
    wsRef.current = ws;

    ws.onopen = () => {
      console.log('✅ [PROTOBUF] WebSocket Connected');
      setStatus('CONNECTED');
      ws.send('/proto');
    };

    ws.onclose = () => {
      console.warn('❌ [PROTOBUF] WebSocket Disconnected');
      setStatus('DISCONNECTED');
    };

    ws.onerror = (err) => {
      console.error('⚠ [PROTOBUF] WebSocket Error:', err);
      setStatus('ERROR');
    };

    ws.onmessage = (event) => {
      if (event.data instanceof ArrayBuffer) {
        if (processingRef.current) return;
        processingRef.current = true;
        
        const data = event.data;
        setTimeout(() => {
          try {
            const start = performance.now();
            const arr = new Uint8Array(data);
            const decoded = sensor_proto.SensorList.decode(arr);
            
            const count = decoded.sensors?.length || 0;
            
            let sum = 0;
            for (let i = 0; i < count; i++) {
              sum += decoded.sensors![i].value;
            }

            const rawTime = performance.now() - start;
            const rawSize = data.byteLength;

            const time = timeFilter.current.process(rawTime, paramsRef.current.tau, paramsRef.current.filterEnabled);
            const size = sizeFilter.current.process(rawSize, paramsRef.current.tau, paramsRef.current.filterEnabled);

            setStatsRef.current({ size, time, count });
          } catch (e) {
            console.error("Proto parse error", e);
          } finally {
            processingRef.current = false;
          }
        }, 0);
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div className="client-card proto">
      <div className="card-header">
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <h2>Protobuf</h2>
          <button onClick={() => props.onReset()} className="mini-btn">RE-SYNC</button>
        </div>
        <div style={{ fontSize: '0.6rem', color: status === 'CONNECTED' ? '#4ade80' : '#f87171', fontWeight: 'bold' }}>
          {status}
        </div>
      </div>
      
      <div className="metric-grid">
        <div className="metric-box">
          <div className="metric-label">Extract & Processing Time</div>
          <div className="metric-value">{stats.time.toFixed(3)}<span className="metric-unit">ms</span></div>
        </div>
        <div className="metric-box">
          <div className="metric-label">Payload Footprint</div>
          <div className="metric-value">{(stats.size / 1024).toFixed(1)}<span className="metric-unit">KB</span></div>
        </div>
      </div>
    </div>
  );
}

function FlatClient(props: { onReset: () => void }) {
  const { enabled: filterEnabled, tau } = useContext(FilterContext);
  const [stats, setStats] = useState({ size: 0, time: 0, count: 0 });
  const [status, setStatus] = useState<'CONNECTING' | 'CONNECTED' | 'DISCONNECTED' | 'ERROR'>('CONNECTING');
  const setStatsRef = useRef(setStats);
  setStatsRef.current = setStats;
  const wsRef = useRef<WebSocket | null>(null);
  const processingRef = useRef(false);
  
  const timeFilter = useRef(new LowPassFilter());
  const sizeFilter = useRef(new LowPassFilter());
  const paramsRef = useRef({ tau, filterEnabled });

  useEffect(() => {
    paramsRef.current = { tau, filterEnabled };
  }, [tau, filterEnabled]);

  useEffect(() => {
    const ws = new WebSocket('ws://127.0.0.1:8080');
    ws.binaryType = 'arraybuffer';
    wsRef.current = ws;

    ws.onopen = () => {
      console.log('✅ [FLATBUFFERS] WebSocket Connected');
      setStatus('CONNECTED');
      ws.send('/flat');
    };

    ws.onclose = () => {
      console.warn('❌ [FLATBUFFERS] WebSocket Disconnected');
      setStatus('DISCONNECTED');
    };

    ws.onerror = (err) => {
      console.error('⚠ [FLATBUFFERS] WebSocket Error:', err);
      setStatus('ERROR');
    };

    ws.onmessage = (event) => {
      if (event.data instanceof ArrayBuffer) {
        if (processingRef.current) return;
        processingRef.current = true;

        const data = event.data;
        setTimeout(() => {
          try {
            const start = performance.now();
            const arr = new Uint8Array(data);
            const bb = new ByteBuffer(arr);
            
            const decoded = SensorList.getRootAsSensorList(bb);
            const count = decoded.sensorsLength();
            
            let sum = 0;
            for (let i = 0; i < count; i++) {
              const sensor = decoded.sensors(i);
              if (sensor) {
                sum += sensor.value();
              }
            }

            const rawTime = performance.now() - start;
            const rawSize = data.byteLength;

            const time = timeFilter.current.process(rawTime, paramsRef.current.tau, paramsRef.current.filterEnabled);
            const size = sizeFilter.current.process(rawSize, paramsRef.current.tau, paramsRef.current.filterEnabled);

            setStatsRef.current({ size, time, count });
          } catch (e) {
            console.error("Flat parse error", e);
          } finally {
            processingRef.current = false;
          }
        }, 0);
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div className="client-card flat">
      <div className="card-header">
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <h2>FlatBuffers</h2>
          <button onClick={() => props.onReset()} className="mini-btn">RE-SYNC</button>
        </div>
        <div style={{ fontSize: '0.6rem', color: status === 'CONNECTED' ? '#4ade80' : '#f87171', fontWeight: 'bold' }}>
          {status}
        </div>
      </div>
      
      <div className="metric-grid">
        <div className="metric-box">
          <div className="metric-label">Access & Traversal Time</div>
          <div className="metric-value">{stats.time.toFixed(3)}<span className="metric-unit">ms</span></div>
        </div>
        <div className="metric-box">
          <div className="metric-label">Payload Footprint</div>
          <div className="metric-value">{(stats.size / 1024).toFixed(1)}<span className="metric-unit">KB</span></div>
        </div>
      </div>
    </div>
  );
}

function JsonClient(props: { onReset: () => void }) {
  const { enabled: filterEnabled, tau } = useContext(FilterContext);
  const [stats, setStats] = useState({ size: 0, time: 0, count: 0 });
  const [status, setStatus] = useState<'CONNECTING' | 'CONNECTED' | 'DISCONNECTED' | 'ERROR'>('CONNECTING');
  const setStatsRef = useRef(setStats);
  setStatsRef.current = setStats;

  const statsMeta = useRef({ lastFrameTime: Date.now(), totalFrames: 0, droppedFrames: 0 });
  
  const wsRef = useRef<WebSocket | null>(null);
  const processingRef = useRef(false);
  const decoderRef = useRef(new TextDecoder());
  
  const timeFilter = useRef(new LowPassFilter());
  const sizeFilter = useRef(new LowPassFilter());
  const paramsRef = useRef({ tau, filterEnabled });

  useEffect(() => {
    paramsRef.current = { tau, filterEnabled };
  }, [tau, filterEnabled]);

  useEffect(() => {
    const ws = new WebSocket('ws://127.0.0.1:8080');
    ws.binaryType = 'arraybuffer';
    wsRef.current = ws;

    ws.onopen = () => {
      console.log('✅ [NATIVE-JSON] WebSocket Connected');
      setStatus('CONNECTED');
      ws.send('/json');
    };

    ws.onclose = () => {
      console.warn('❌ [NATIVE-JSON] WebSocket Disconnected');
      setStatus('DISCONNECTED');
    };

    ws.onerror = (err) => {
      console.error('⚠ [NATIVE-JSON] WebSocket Error:', err);
      setStatus('ERROR');
    };

    ws.onmessage = (event) => {
      // JSON is sent as binary exactly like the others to measure identical extraction conditions
      if (event.data instanceof ArrayBuffer) {
        statsMeta.current.totalFrames++;

        if (processingRef.current) {
          statsMeta.current.droppedFrames++;
          if (statsMeta.current.droppedFrames % 100 === 0) {
            console.warn(`📉 [NATIVE-JSON] High Pressure: Dropped ${statsMeta.current.droppedFrames} frames so far...`);
          }
          return;
        }

        processingRef.current = true;
        const data = event.data;
        setTimeout(() => {
          try {
            const start = performance.now();
            const text = decoderRef.current.decode(data);
            const decoded = JSON.parse(text);
            
            statsMeta.current.lastFrameTime = Date.now();
            const count = decoded.length || 0;
            // ... (rest of processing stays same)
            
            let sum = 0;
            for (let i = 0; i < count; i++) {
              sum += decoded[i].value;
            }

            const rawTime = performance.now() - start;
            const rawSize = data.byteLength;

            const time = timeFilter.current.process(rawTime, paramsRef.current.tau, paramsRef.current.filterEnabled);
            const size = sizeFilter.current.process(rawSize, paramsRef.current.tau, paramsRef.current.filterEnabled);

            setStatsRef.current({ size, time, count });
          } catch (e) {
            console.error("JSON parse error", e);
          } finally {
            processingRef.current = false;
          }
        }, 0);
      }
    };

    // Stall Detection Watchdog
    const watchdog = setInterval(() => {
      const delta = Date.now() - statsMeta.current.lastFrameTime;
      if (wsRef.current?.readyState === WebSocket.OPEN && delta > 2000) {
        console.error(`🚨 [NATIVE-JSON] STALL DETECTED: No frame processed for ${Math.round(delta/1000)}s. Main thread might be locked.`);
      }
    }, 2000);

    return () => {
      clearInterval(watchdog);
      ws.close();
    };
  }, []);

  return (
    <div className="client-card json">
      <div className="card-header">
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <h2>Native JSON</h2>
          <button onClick={() => props.onReset()} className="mini-btn">RE-SYNC</button>
        </div>
        <div style={{ fontSize: '0.6rem', color: status === 'CONNECTED' ? '#4ade80' : '#f87171', fontWeight: 'bold' }}>
          {status}
        </div>
      </div>
      
      <div className="metric-grid">
        <div className="metric-box">
          <div className="metric-label">Parse & Traversal Time</div>
          <div className="metric-value">{stats.time.toFixed(3)}<span className="metric-unit">ms</span></div>
        </div>
        <div className="metric-box">
          <div className="metric-label">Payload Footprint</div>
          <div className="metric-value">{(stats.size / 1024).toFixed(1)}<span className="metric-unit">KB</span></div>
        </div>
      </div>
    </div>
  );
}

function App() {
  const presets = [1000, 5000, 20000, 50000];
  const [val, setVal] = useState(5000);
  const [filterEnabled, setFilterEnabled] = useState(false);
  const [tau, setTau] = useState(100); // Time constant in ms
  const [resetKeys, setResetKeys] = useState({ proto: 0, flat: 0, json: 0 });

  // Convert Time Constant (ms) to Alpha for the 60Hz loop
  const dt = 1000 / 60;
  const alpha = dt / (tau + dt);

  const reset = (key: 'proto'|'flat'|'json') => {
    setResetKeys(prev => ({ ...prev, [key]: prev[key] + 1 }));
  };

  const applySize = (newVal: number) => {
    setVal(newVal);
    const ws = new WebSocket('ws://127.0.0.1:8080');
    ws.onopen = () => {
      ws.send('/flat'); // Target arbitrary route to invoke channel send
      setTimeout(() => {
        ws.send(newVal.toString());
        setTimeout(() => ws.close(), 100);
      }, 50);
    };
  };

  return (
    <FilterContext.Provider value={{ enabled: filterEnabled, tau }}>
      <div className="title-bar">
        <h1>Telemetry Bench</h1>
        <p style={{ color: 'var(--text-dim)', letterSpacing: '1px' }}>60Hz Streaming Vector Simulation</p>
      </div>

      <div className="panel controls-grid">
        <div className="control-group">
          <label>Active Sensor Matrix [{val}]</label>
          <div className="btn-group">
            {presets.map(p => (
              <button key={p} className={val === p ? 'active' : ''} onClick={() => applySize(p)}>
                {p / 1000}k
              </button>
            ))}
            <button key="plus10k" onClick={() => applySize(val + 10000)} style={{ background: 'rgba(74, 222, 128, 0.2)', borderColor: '#4ade80', color: '#4ade80' }}>
              +10k
            </button>
            <input 
              type="number" 
              value={val} 
              onChange={e => setVal(Number(e.target.value))} 
              onBlur={() => applySize(val)}
              onKeyDown={(e) => e.key === 'Enter' && applySize(val)}
              placeholder="Custom"
              style={{ marginLeft: '12px' }}
            />
          </div>
        </div>

        <div className="control-group">
          <div className="toggle-container" style={{ marginBottom: '8px' }}>
            <label>Low-Pass Signal Filter</label>
            <label className="switch">
              <input type="checkbox" checked={filterEnabled} onChange={e => setFilterEnabled(e.target.checked)} />
              <span className="slider"></span>
            </label>
          </div>
          <div className="btn-group" style={{ 
            alignItems: 'center', 
            marginTop: '10px',
            opacity: filterEnabled ? 1 : 0.3,
            pointerEvents: filterEnabled ? 'auto' : 'none',
            transition: 'opacity 0.3s'
          }}>
            <label style={{ marginRight: '8px' }}>Time Constant (ms): </label>
            <input 
              type="range" 
              step="1"
              min="1"
              max="2000"
              value={tau} 
              onChange={e => setTau(Number(e.target.value))} 
              style={{ width: '200px', cursor: filterEnabled ? 'ew-resize' : 'default' }}
            />
            <input 
              type="number" 
              step="1"
              min="1"
              max="10000"
              value={tau} 
              onChange={e => setTau(Number(e.target.value))} 
              style={{ 
                width: '80px', 
                background: 'rgba(0,0,0,0.3)', 
                border: '1px solid rgba(255,255,255,0.1)',
                color: 'var(--accent-color)',
                padding: '4px',
                borderRadius: '4px',
                marginLeft: '12px',
                fontFamily: 'inherit'
              }}
            />
          </div>
        </div>
      </div>

      <div className="comparison-container">
        <JsonClient key={`json-${resetKeys.json}`} onReset={() => reset('json')} />
        <ProtoClient key={`proto-${resetKeys.proto}`} onReset={() => reset('proto')} />
        <FlatClient key={`flat-${resetKeys.flat}`} onReset={() => reset('flat')} />
      </div>
    </FilterContext.Provider>
  );
}

export default App;
