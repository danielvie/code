import { useEffect, useState, useRef, createContext, useContext } from 'react';
import { sensor_proto } from './generated/sensor_proto';
import { SensorList } from './generated/sensor-fbs';
import { ByteBuffer } from 'flatbuffers';
import './index.css';

const FilterContext = createContext({ enabled: false, alpha: 0.1 });

function ProtoClient() {
  const { enabled: filterEnabled, alpha } = useContext(FilterContext);
  const [stats, setStats] = useState({ size: 0, time: 0, count: 0 });
  const wsRef = useRef<WebSocket | null>(null);
  
  const filterRef = useRef({ enabled: false, alpha: 0.1, smoothedTime: 0, smoothedSize: 0 });

  useEffect(() => {
    filterRef.current.enabled = filterEnabled;
    filterRef.current.alpha = alpha;
  }, [filterEnabled, alpha]);

  useEffect(() => {
    const ws = new WebSocket('ws://127.0.0.1:8080');
    ws.binaryType = 'arraybuffer';
    wsRef.current = ws;

    ws.onopen = () => {
      ws.send('/proto');
    };

    ws.onmessage = (event) => {
      if (event.data instanceof ArrayBuffer) {
        const start = performance.now();
        const arr = new Uint8Array(event.data);
        const decoded = sensor_proto.SensorList.decode(arr);
        
        const count = decoded.sensors?.length || 0;
        
        let sum = 0;
        for (let i = 0; i < count; i++) {
          sum += decoded.sensors![i].value;
        }

        const rawTime = performance.now() - start;
        const rawSize = event.data.byteLength;

        let time = rawTime;
        let size = rawSize;
        const f = filterRef.current;

        if (f.enabled) {
          if (f.smoothedTime === 0) f.smoothedTime = rawTime;
          if (f.smoothedSize === 0) f.smoothedSize = rawSize;
          f.smoothedTime = f.alpha * rawTime + (1 - f.alpha) * f.smoothedTime;
          f.smoothedSize = f.alpha * rawSize + (1 - f.alpha) * f.smoothedSize;
          time = f.smoothedTime;
          size = f.smoothedSize;
        } else {
          f.smoothedTime = rawTime;
          f.smoothedSize = rawSize;
        }

        setStats({ size, time, count });
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div className="client-card proto">
      <div className="card-header">
        <h2>Protobuf</h2>
        <div className="status-indicator">
          <div className="pulse"></div> LIVE
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

function FlatClient() {
  const { enabled: filterEnabled, alpha } = useContext(FilterContext);
  const [stats, setStats] = useState({ size: 0, time: 0, count: 0 });
  const wsRef = useRef<WebSocket | null>(null);
  
  const filterRef = useRef({ enabled: false, alpha: 0.1, smoothedTime: 0, smoothedSize: 0 });

  useEffect(() => {
    filterRef.current.enabled = filterEnabled;
    filterRef.current.alpha = alpha;
  }, [filterEnabled, alpha]);

  useEffect(() => {
    const ws = new WebSocket('ws://127.0.0.1:8080');
    ws.binaryType = 'arraybuffer';
    wsRef.current = ws;

    ws.onopen = () => {
      ws.send('/flat');
    };

    ws.onmessage = (event) => {
      if (event.data instanceof ArrayBuffer) {
        const start = performance.now();
        const arr = new Uint8Array(event.data);
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
        const rawSize = event.data.byteLength;

        let time = rawTime;
        let size = rawSize;
        const f = filterRef.current;

        if (f.enabled) {
          if (f.smoothedTime === 0) f.smoothedTime = rawTime;
          if (f.smoothedSize === 0) f.smoothedSize = rawSize;
          f.smoothedTime = f.alpha * rawTime + (1 - f.alpha) * f.smoothedTime;
          f.smoothedSize = f.alpha * rawSize + (1 - f.alpha) * f.smoothedSize;
          time = f.smoothedTime;
          size = f.smoothedSize;
        } else {
          f.smoothedTime = rawTime;
          f.smoothedSize = rawSize;
        }

        setStats({ size, time, count });
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div className="client-card flat">
      <div className="card-header">
        <h2>FlatBuffers</h2>
        <div className="status-indicator">
          <div className="pulse"></div> LIVE
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

function App() {
  const presets = [1000, 5000, 20000, 50000];
  const [val, setVal] = useState(5000);
  const [filterEnabled, setFilterEnabled] = useState(false);
  const [alpha, setAlpha] = useState(0.1);

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
    <FilterContext.Provider value={{ enabled: filterEnabled, alpha }}>
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
            <label style={{ marginRight: '8px' }}>Cutoff Config (α): </label>
            <input 
              type="range" 
              step="0.001"
              min="0.001"
              max="1.0"
              value={alpha} 
              onChange={e => setAlpha(Number(e.target.value))} 
              style={{ width: '200px', cursor: filterEnabled ? 'ew-resize' : 'default' }}
            />
            <input 
              type="number" 
              step="0.001"
              min="0.001"
              max="1.0"
              value={alpha} 
              onChange={e => setAlpha(Number(e.target.value))} 
              style={{ width: '80px', marginLeft: '8px' }}
            />
          </div>
        </div>
      </div>

      <div className="comparison-container">
        <ProtoClient />
        <FlatClient />
      </div>
    </FilterContext.Provider>
  );
}

export default App;
