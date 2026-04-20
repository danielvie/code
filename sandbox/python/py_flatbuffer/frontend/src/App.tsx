import React, { useEffect, useRef, useState, useCallback } from 'react';
import * as flatbuffers from 'flatbuffers';
import { Message } from './generated/Simulation/message';
import { MessageType } from './generated/Simulation/message-type';
import { PendulumState } from './generated/Simulation/pendulum-state';
import { Parameters } from './generated/Simulation/parameters';
import { Command } from './generated/Simulation/command';
import { Reset } from './generated/Simulation/reset';
import { IntegrationMethod } from './generated/Simulation/integration-method';
import { Target, Activity, Cpu, Layers, RotateCcw, AlertTriangle, Undo2 } from 'lucide-react';

const DEFAULT_PARAMS = {
  length: 1.0,
  massCart: 1.0,
  massPole: 0.1,
  qPos: 10.0,
  qVel: 1.0,
  qAng: 100.0,
  qOmg: 10.0,
  rCtrl: 0.1,
  integrationMethod: IntegrationMethod.Euler
};

const App: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const chartRef = useRef<HTMLCanvasElement>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const [connected, setConnected] = useState(false);
  const [isValid, setIsValid] = useState(true);
  const [pps, setPps] = useState(0);
  const packetCountRef = useRef(0);
  const historyRef = useRef<{x: number, target: number}[]>([]);
  
  // Monitoring Throughput
  useEffect(() => {
    const timer = setInterval(() => {
      setPps(packetCountRef.current);
      packetCountRef.current = 0;
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const [state, setState] = useState({
    x: 0,
    v: 0,
    theta: 0,
    omega: 0,
  });

  const [targetX, setTargetX] = useState(0);
  const [params, setParams] = useState(DEFAULT_PARAMS);

  const targetRef = useRef(0);
  useEffect(() => { targetRef.current = targetX; }, [targetX]);

  const sendMsg = useCallback((type: MessageType, dataBuilder: (b: flatbuffers.Builder) => number) => {
    if (!wsRef.current || wsRef.current.readyState !== WebSocket.OPEN) return;
    const innerBuilder = new flatbuffers.Builder(256);
    const offset = dataBuilder(innerBuilder);
    innerBuilder.finish(offset);
    const bytes = innerBuilder.asUint8Array();
    const outerBuilder = new flatbuffers.Builder(512);
    const dataVector = Message.createDataVector(outerBuilder, bytes);
    Message.startMessage(outerBuilder);
    Message.addType(outerBuilder, type);
    Message.addData(outerBuilder, dataVector);
    const msgOffset = Message.endMessage(outerBuilder);
    outerBuilder.finish(msgOffset);
    wsRef.current.send(outerBuilder.asUint8Array());
  }, []);

  const handleReset = () => {
    sendMsg(MessageType.Reset, (b) => {
      Reset.startReset(b);
      return Reset.endReset(b);
    });
    setIsValid(true);
    historyRef.current = [];
  };

  const handleRestoreDefaults = () => {
    setParams(DEFAULT_PARAMS);
    setTargetX(0);
  };

  // WebSocket Setup
  useEffect(() => {
    const ws = new WebSocket('ws://localhost:8765');
    ws.binaryType = 'arraybuffer';
    wsRef.current = ws;
    ws.onopen = () => setConnected(true);
    ws.onclose = () => setConnected(false);
    ws.onmessage = (event) => {
      packetCountRef.current++;
      const buf = new Uint8Array(event.data);
      const builder = new flatbuffers.ByteBuffer(buf);
      const msg = Message.getRootAsMessage(builder);
      if (msg.type() === MessageType.State) {
        const stateData = msg.dataArray();
        if (stateData) {
          const stateBuf = new flatbuffers.ByteBuffer(stateData);
          const pState = PendulumState.getRootAsPendulumState(stateBuf);
          const valid = pState.isValid();
          setIsValid(valid);
          if (valid) {
            const currentX = pState.cartPosition();
            setState({
              x: currentX,
              v: pState.cartVelocity(),
              theta: pState.pendulumAngle(),
              omega: pState.pendulumAngularVelocity(),
            });
            historyRef.current.push({ x: currentX, target: targetRef.current });
            if (historyRef.current.length > 300) historyRef.current.shift();
          }
        }
      }
    };
    return () => ws.close();
  }, []);

  const syncParams = useCallback(() => {
    sendMsg(MessageType.Params, (b) => {
      Parameters.startParameters(b);
      Parameters.addLength(b, params.length);
      Parameters.addMassCart(b, params.massCart);
      Parameters.addMassPole(b, params.massPole);
      Parameters.addQPos(b, params.qPos);
      Parameters.addQVel(b, params.qVel);
      Parameters.addQAng(b, params.qAng);
      Parameters.addQOmg(b, params.qOmg);
      Parameters.addRCtrl(b, params.rCtrl);
      Parameters.addIntegrationMethod(b, params.integrationMethod);
      return Parameters.endParameters(b);
    });
  }, [params, sendMsg]);

  useEffect(() => {
    sendMsg(MessageType.Cmd, (b) => {
      Command.startCommand(b);
      Command.addTargetPosition(b, targetX);
      return Command.endCommand(b);
    });
  }, [targetX, sendMsg]);

  // Main Render
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    let animationFrameId: number;

    const render = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      const centerX = canvas.width / 2;
      const centerY = (canvas.height / 2) + 100;
      const scale = 220;

      // Grid
      ctx.strokeStyle = '#2d333b';
      ctx.lineWidth = 1;
      ctx.beginPath();
      for (let i = -20; i <= 20; i++) {
        const px = centerX + (i * (scale / 4));
        ctx.moveTo(px, 0); ctx.lineTo(px, canvas.height);
      }
      ctx.stroke();

      // Track
      ctx.strokeStyle = '#2d333b';
      ctx.lineWidth = 4;
      ctx.beginPath();
      ctx.moveTo(0, centerY + 25);
      ctx.lineTo(canvas.width, centerY + 25);
      ctx.stroke();

      // Target (Indicator)
      const tx = centerX + targetX * scale;
      ctx.strokeStyle = '#e06c75'; 
      ctx.setLineDash([5, 5]);
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(tx, 0); ctx.lineTo(tx, canvas.height);
      ctx.stroke();
      ctx.setLineDash([]);

      // Cart
      const cartX = centerX + state.x * scale;
      const cartY = centerY;
      
      // Cart Body
      ctx.fillStyle = isValid ? '#3e4451' : '#b91c1c'; 
      ctx.strokeStyle = '#4b5263';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.roundRect(cartX - 50, cartY - 20, 100, 40, 4);
      ctx.fill();
      ctx.stroke();

      // Wheels
      ctx.fillStyle = '#21252b';
      ctx.beginPath(); ctx.arc(cartX - 35, cartY + 20, 12, 0, Math.PI * 2); ctx.fill();
      ctx.beginPath(); ctx.arc(cartX + 35, cartY + 20, 12, 0, Math.PI * 2); ctx.fill();

      // Pole
      const poleEndX = cartX + Math.sin(state.theta) * params.length * scale;
      const poleEndY = cartY - 20 - Math.cos(state.theta) * params.length * scale;
      
      ctx.strokeStyle = '#90a4ae';
      ctx.lineWidth = 6;
      ctx.lineCap = 'round';
      ctx.beginPath(); ctx.moveTo(cartX, cartY - 20); ctx.lineTo(poleEndX, poleEndY); ctx.stroke();
      
      // Bob
      ctx.fillStyle = isValid ? '#61afef' : '#f87171'; 
      ctx.beginPath(); ctx.arc(poleEndX, poleEndY, 14, 0, Math.PI * 2); ctx.fill();

      if (!isValid) {
        ctx.fillStyle = 'rgba(0,0,0,0.4)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = '#fff';
        ctx.font = 'bold 24px Outfit';
        ctx.textAlign = 'center';
        ctx.fillText('STABILITY FAILURE', centerX, centerY - 20);
        ctx.font = '16px Outfit';
        ctx.fillText('NUMERICAL OVERFLOW DETECTED', centerX, centerY + 10);
      }
      
      animationFrameId = requestAnimationFrame(render);
    };
    render();
    return () => cancelAnimationFrame(animationFrameId);
  }, [state, params, targetX, isValid]);

  // Chart Render
  useEffect(() => {
    const canvas = chartRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    let frame: number;
    const render = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      const history = historyRef.current;
      if (history.length < 2) { frame = requestAnimationFrame(render); return; }
      
      const w = canvas.width; const h = canvas.height;
      const scale = 40;
      
      // Grid
      ctx.strokeStyle = '#1c2026'; ctx.lineWidth = 1;
      for(let y = -5; y <= 5; y++) {
        const yPos = h/2 - y * scale;
        ctx.beginPath(); ctx.moveTo(0, yPos); ctx.lineTo(w, yPos); ctx.stroke();
      }

      // Origin
      ctx.strokeStyle = '#2d333b'; ctx.lineWidth = 2;
      ctx.beginPath(); ctx.moveTo(0, h/2); ctx.lineTo(w, h/2); ctx.stroke();

      // Target
      ctx.strokeStyle = '#e06c75'; 
      ctx.setLineDash([5, 5]); 
      ctx.lineWidth = 2;
      ctx.beginPath();
      history.forEach((p, i) => {
        const x = (i / 300) * w;
        const y = h/2 - p.target * scale;
        if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      });
      ctx.stroke(); ctx.setLineDash([]);

      // State
      ctx.strokeStyle = '#61afef'; 
      ctx.lineWidth = 2; 
      ctx.beginPath();
      history.forEach((p, i) => {
        const x = (i / 300) * w;
        const y = h/2 - p.x * scale;
        if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      });
      ctx.stroke();

      frame = requestAnimationFrame(render);
    };
    render();
    return () => cancelAnimationFrame(frame);
  }, []);

  const handleCanvasClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const centerX = canvas.width / 2;
    const scale = 220; // Must match the render scale
    const newTargetX = (x - centerX) / scale;
    // Constrain to reasonable range
    setTargetX(Math.max(-5, Math.min(5, newTargetX)));
  };

  useEffect(() => {
    const handleResize = (entries: ResizeObserverEntry[]) => {
      for (const entry of entries) {
        const canvas = entry.target as HTMLCanvasElement;
        canvas.width = entry.contentRect.width;
        canvas.height = entry.contentRect.height;
      }
    };
    
    const observer = new ResizeObserver(handleResize);
    if (canvasRef.current) observer.observe(canvasRef.current);
    if (chartRef.current) observer.observe(chartRef.current);
    
    return () => observer.disconnect();
  }, []);

  return (
    <div className="dashboard">
      <aside className="sidebar">
        <div className="sidebar-header">
          <div style={{display: 'flex', justifyContent: 'space-between', alignItems: 'center'}}>
            <h1>Antigrav <span style={{fontWeight: 400, opacity: 0.5}}>v2.1</span></h1>
            <div style={{display: 'flex', gap: '0.25rem'}}>
              <button className="icon-btn" onClick={handleRestoreDefaults} title="Restore Default Settings">
                <Undo2 size={16} />
              </button>
              <button className={`icon-btn ${!isValid ? 'pulse' : ''}`} onClick={handleReset} title="Restart Simulation">
                <RotateCcw size={16} />
              </button>
            </div>
          </div>
          <div className="status" style={{display: 'flex', justifyContent: 'space-between'}}>
            <div style={{display: 'flex', alignItems: 'center'}}>
              <div className={`status-dot ${connected ? (isValid ? 'connected' : 'error') : ''}`} />
              {connected ? (isValid ? 'System Stable' : 'Stability Failure') : 'Link Offline'}
            </div>
            {connected && <div className="pps-badge" title="Packets Per Second (Simulation Sample Rate)">{pps} PPS</div>}
          </div>
        </div>

        <div className="sidebar-content">
          {!isValid && (
            <div className="alert-box">
              <AlertTriangle size={16} />
              <span>Simulation Halted</span>
            </div>
          )}

          <div className="control-item">
            <label><Layers size={10} style={{marginRight: 4}}/> Pole Length</label>
            <div className="control-row">
              <input type="range" min="0.5" max="3" step="0.1" value={params.length} onChange={e => setParams(p => ({...p, length: parseFloat(e.target.value)}))} />
              <div className="control-value">{params.length.toFixed(1)}m</div>
            </div>
          </div>

          <div className="control-item">
            <label><Layers size={10} style={{marginRight: 4}}/> Pole Mass</label>
            <div className="control-row">
              <input type="range" min="0.01" max="1" step="0.01" value={params.massPole} onChange={e => setParams(p => ({...p, massPole: parseFloat(e.target.value)}))} />
              <div className="control-value">{params.massPole.toFixed(2)}kg</div>
            </div>
          </div>

          <div className="control-item">
            <label><Cpu size={10} style={{marginRight: 4}}/> Penalty: Position (Q1)</label>
            <div className="control-row">
              <input type="range" min="1" max="200" step="1" value={params.qPos} onChange={e => setParams(p => ({...p, qPos: parseFloat(e.target.value)}))} />
              <div className="control-value">{params.qPos.toFixed(0)}</div>
            </div>
          </div>

          <div className="control-item">
            <label><Cpu size={10} style={{marginRight: 4}}/> Penalty: Velocity (Q2)</label>
            <div className="control-row">
              <input type="range" min="1" max="200" step="1" value={params.qVel} onChange={e => setParams(p => ({...p, qVel: parseFloat(e.target.value)}))} />
              <div className="control-value">{params.qVel.toFixed(0)}</div>
            </div>
          </div>

          <div className="control-item">
            <label><Cpu size={10} style={{marginRight: 4}}/> Penalty: Angle (Q3)</label>
            <div className="control-row">
              <input type="range" min="1" max="200" step="1" value={params.qAng} onChange={e => setParams(p => ({...p, qAng: parseFloat(e.target.value)}))} />
              <div className="control-value">{params.qAng.toFixed(0)}</div>
            </div>
          </div>

          <div className="control-item">
            <label><Cpu size={10} style={{marginRight: 4}}/> Penalty: Ang.Vel (Q4)</label>
            <div className="control-row">
              <input type="range" min="1" max="50" step="1" value={params.qOmg} onChange={e => setParams(p => ({...p, qOmg: parseFloat(e.target.value)}))} />
              <div className="control-value">{params.qOmg.toFixed(0)}</div>
            </div>
          </div>

          <div className="control-item" style={{borderTop: '1px solid var(--border-color)', paddingTop: '0.75rem', marginBottom: '0.5rem'}}>
            <label><Activity size={10} style={{marginRight: 4}}/> Control Effort (R)</label>
            <div className="control-row">
              <input type="range" min="0.01" max="100" step="0.1" value={params.rCtrl} onChange={e => setParams(p => ({...p, rCtrl: parseFloat(e.target.value)}))} />
              <div className="control-value">{params.rCtrl.toFixed(3)}</div>
            </div>
          </div>

          <div className="control-item">
            <label><Cpu size={10} style={{marginRight: 4}}/> Solver Method</label>
            <div className="control-row">
              <select 
                value={params.integrationMethod} 
                onChange={e => setParams(p => ({...p, integrationMethod: parseInt(e.target.value)}))}
                style={{
                  width: '100%',
                  background: 'var(--panel-secondary)',
                  color: 'var(--text-primary)',
                  border: '1px solid var(--border-color)',
                  borderRadius: '4px',
                  padding: '4px 8px',
                  fontSize: '0.75rem',
                  outline: 'none'
                }}
              >
                <option value={IntegrationMethod.Euler}>Euler (Fast)</option>
                <option value={IntegrationMethod.ODE3}>ODE3 (Balanced)</option>
                <option value={IntegrationMethod.RK4}>Runge-Kutta 4 (Precise)</option>
              </select>
            </div>
          </div>

          <div className="telemetry-grid">
            <div className="data-node">
              <div className="data-label">Target</div>
              <div className="data-value">{targetX.toFixed(2)}m</div>
            </div>
            <div className="data-node">
              <div className="data-label">Pos.X</div>
              <div className="data-value">{state.x.toFixed(3)}</div>
            </div>
            <div className="data-node">
              <div className="data-label">Deg.θ</div>
              <div className="data-value">{(state.theta * 180 / Math.PI).toFixed(2)}°</div>
            </div>
          </div>
        </div>

        <div className="sidebar-footer">
          <div className="control-item">
            <label style={{paddingLeft: '1.25rem', paddingRight: '1.25rem'}}>Chart</label>
            <div className="chart-container">
              <div className="chart-overlay">POSITION</div>
              <canvas ref={chartRef} />
            </div>
          </div>
        </div>
      </aside>

      <main className="viewport">
        <canvas ref={canvasRef} onClick={handleCanvasClick} style={{cursor: 'crosshair'}} />
      </main>
    </div>
  );
};

export default App;
