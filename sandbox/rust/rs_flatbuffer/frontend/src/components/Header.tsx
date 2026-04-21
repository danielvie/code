import React from "react";
import { RotateCcw } from "lucide-react";

interface HeaderProps {
  connected: boolean;
  isValid: boolean;
  pps: number;
  targetX: number;
  state: {
    x: number;
    theta: number;
  };
  onReset: () => void;
}

const Header: React.FC<HeaderProps> = ({
  connected,
  isValid,
  pps,
  targetX,
  state,
  onReset,
}) => {
  return (
    <header className="viewport-header">
      <div className="viewport-header-title">
        <h1>Inverted Pendulum</h1>
        <div className="status">
          <div
            className={`status-dot ${connected ? (isValid ? "connected" : "error") : "disconnected"}`}
            title={
              connected
                ? isValid
                  ? "Connected — simulation running"
                  : "Connected — stability failure"
                : "Disconnected from backend"
            }
          />
          <span>
            {connected
              ? isValid
                ? "Online"
                : "Stability Failure"
              : "Disconnected"}
          </span>
        </div>
      </div>
      <div className="viewport-header-actions">
        <button
          className={`icon-btn ${!isValid ? "pulse" : ""}`}
          onClick={onReset}
          title="Restart Simulation"
          aria-label="Restart Simulation"
          style={{ gap: "0.5rem", padding: "0.4rem 0.75rem" }}
        >
          <RotateCcw size={16} />
          <span style={{ fontSize: "0.7rem", fontWeight: 700 }}>Restart</span>
        </button>
      </div>
      <div className="viewport-header-telemetry">
        <div className="telemetry-grid">
          <div
            className={`data-node ${Math.abs(targetX - state.x) > 0.5 ? "highlight" : ""}`}
          >
            <div className="data-label">Target</div>
            <div className="data-value">{targetX.toFixed(2)}m</div>
          </div>
          <div className="data-node">
            <div className="data-label">Pos.X</div>
            <div className="data-value">{state.x.toFixed(3)}</div>
          </div>
          <div className="data-node">
            <div className="data-label">Deg.θ</div>
            <div className="data-value">
              {((state.theta * 180) / Math.PI).toFixed(2)}°
            </div>
          </div>
        </div>
        {connected && (
          <div className="pps-badge" title="Packets Per Second">
            {pps} PPS
          </div>
        )}
      </div>
    </header>
  );
};

export default Header;
