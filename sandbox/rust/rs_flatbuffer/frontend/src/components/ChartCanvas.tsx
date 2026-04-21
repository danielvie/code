import React, { useEffect, useRef } from "react";

interface ChartCanvasProps {
  historyRef: React.MutableRefObject<{ x: number; target: number }[]>;
}

const ChartCanvas: React.FC<ChartCanvasProps> = ({ historyRef }) => {
  const chartRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = chartRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    let frame: number;

    const render = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      const history = historyRef.current;
      if (history.length < 2) {
        frame = requestAnimationFrame(render);
        return;
      }

      const w = canvas.width;
      const h = canvas.height;
      const scale = 40;

      // Grid
      ctx.strokeStyle = "#1e232a";
      ctx.lineWidth = 1;
      for (let y = -5; y <= 5; y++) {
        const yPos = h / 2 - y * scale;
        ctx.beginPath();
        ctx.moveTo(0, yPos);
        ctx.lineTo(w, yPos);
        ctx.stroke();
      }

      // Origin
      ctx.strokeStyle = "#2a3039";
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(0, h / 2);
      ctx.lineTo(w, h / 2);
      ctx.stroke();

      // Target
      ctx.strokeStyle = "rgba(244, 63, 94, 0.65)";
      ctx.setLineDash([4, 4]);
      ctx.lineWidth = 1.5;
      ctx.beginPath();
      history.forEach((p, i) => {
        const x = (i / 300) * w;
        const y = h / 2 - p.target * scale;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      });
      ctx.stroke();
      ctx.setLineDash([]);

      // State
      ctx.strokeStyle = "rgba(45, 212, 191, 0.85)";
      ctx.lineWidth = 2;
      ctx.beginPath();
      history.forEach((p, i) => {
        const x = (i / 300) * w;
        const y = h / 2 - p.x * scale;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      });
      ctx.stroke();

      frame = requestAnimationFrame(render);
    };
    render();
    return () => cancelAnimationFrame(frame);
  }, [historyRef]);

  useEffect(() => {
    const canvas = chartRef.current;
    if (!canvas) return;

    const handleResize = (entries: ResizeObserverEntry[]) => {
      for (const entry of entries) {
        const c = entry.target as HTMLCanvasElement;
        c.width = entry.contentRect.width;
        c.height = entry.contentRect.height;
      }
    };

    const observer = new ResizeObserver(handleResize);
    observer.observe(canvas);
    return () => observer.disconnect();
  }, []);

  return (
    <div className="chart-container">
      <canvas ref={chartRef} />
    </div>
  );
};

export default ChartCanvas;
