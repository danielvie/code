import React, { useEffect, useRef, useCallback } from "react";

// Layout
const CANVAS_BG = "#13161c";
const CENTER_Y_OFFSET = 100;

// Scale
const PIXELS_PER_METER = 220;

// Grid
const GRID_COLOR = "#1e232a";
const GRID_LINE_WIDTH = 1;
const GRID_DIVISIONS = 20;
const GRID_SPACING_FACTOR = 1 / 4;

// Track
const TRACK_COLOR = "#252a33";
const TRACK_LINE_WIDTH = 3;
const TRACK_Y_OFFSET = 25;

// Target
const TARGET_COLOR = "rgba(244, 63, 94, 0.65)";
const TARGET_DASH_LENGTH = 4;
const TARGET_LINE_WIDTH = 1.5;

// Cart Body
const CART_HALF_WIDTH = 50;
const CART_HEIGHT = 40;
const CART_CORNER_RADIUS = 4;
const CART_FILL_VALID = "#2a3039";
const CART_FILL_INVALID = "#7f1d1d";
const CART_STROKE_COLOR = "#3a414d";
const CART_STROKE_WIDTH = 2;
const CART_VERTICAL_OFFSET = 24;

// Wheels
const WHEEL_RADIUS = 12;
const WHEEL_HUB_RADIUS = 3;
const WHEEL_RIM_OFFSET = 2;
const WHEEL_SPOKE_OFFSET = 3;
const WHEEL_SPOKE_COUNT = 4;
const WHEEL_SPOKE_COLOR = "#6b5237";
const WHEEL_SPOKE_WIDTH = 1.5;
const WHEEL_RIM_COLOR = "#8b6f47";
const WHEEL_RIM_WIDTH = 2;
const WHEEL_TIRE_COLOR = "#5c4033";
const WHEEL_HUB_COLOR = "#4a3728";

const WHEEL_LEFT_OFFSET = -35;
const WHEEL_RIGHT_OFFSET = 35;
const WHEEL_VERTICAL_OFFSET = 14;

// Pole
const POLE_COLOR = "#64748b";
const POLE_WIDTH = 6;
const POLE_ATTACH_OFFSET = 20;

// Bob
const BOB_RADIUS = 14;
const BOB_FILL_VALID = "#2dd4bf";
const BOB_FILL_INVALID = "#f43f5e";

// Failure Overlay
const FAILURE_OVERLAY_ALPHA = 0.4;
const FAILURE_TITLE_FONT = "bold 24px Inter";
const FAILURE_SUBTITLE_FONT = "16px Inter";
const FAILURE_TEXT_COLOR = "#ffffff";
const FAILURE_TITLE_OFFSET = -20;
const FAILURE_SUBTITLE_OFFSET = 10;

// Interaction
const TARGET_MIN = -5;
const TARGET_MAX = 5;

// Types
interface SimulationState {
  x: number;
  theta: number;
}

interface SimulationParams {
  length: number;
}

interface SimulationCanvasProps {
  state: SimulationState;
  target_x: number;
  params: SimulationParams;
  is_valid: boolean;
  on_target_change: (target: number) => void;
}

// Draw
const draw_wheel = (
  ctx: CanvasRenderingContext2D,
  wx: number,
  wy: number,
  rotation: number,
) => {
  ctx.save();
  ctx.translate(wx, wy);
  ctx.rotate(rotation);

  // Tire
  ctx.fillStyle = WHEEL_TIRE_COLOR;
  ctx.beginPath();
  ctx.arc(0, 0, WHEEL_RADIUS, 0, Math.PI * 2);
  ctx.fill();

  // Rim
  ctx.strokeStyle = WHEEL_RIM_COLOR;
  ctx.lineWidth = WHEEL_RIM_WIDTH;
  ctx.beginPath();
  ctx.arc(0, 0, WHEEL_RADIUS - WHEEL_RIM_OFFSET, 0, Math.PI * 2);
  ctx.stroke();

  // Spokes
  ctx.strokeStyle = WHEEL_SPOKE_COLOR;
  ctx.lineWidth = WHEEL_SPOKE_WIDTH;
  ctx.lineCap = "round";
  for (let i = 0; i < WHEEL_SPOKE_COUNT; i++) {
    const angle = (i * Math.PI * 2) / WHEEL_SPOKE_COUNT;
    ctx.beginPath();
    ctx.moveTo(0, 0);
    ctx.lineTo(
      Math.cos(angle) * (WHEEL_RADIUS - WHEEL_SPOKE_OFFSET),
      Math.sin(angle) * (WHEEL_RADIUS - WHEEL_SPOKE_OFFSET),
    );
    ctx.stroke();
  }

  // Hub
  ctx.fillStyle = WHEEL_HUB_COLOR;
  ctx.beginPath();
  ctx.arc(0, 0, WHEEL_HUB_RADIUS, 0, Math.PI * 2);
  ctx.fill();

  ctx.restore();
};

const draw_cart = (
  ctx: CanvasRenderingContext2D,
  cart_x: number,
  cart_y: number,
  is_valid: boolean,
  wheel_rotation: number,
) => {
  ctx.fillStyle = is_valid ? CART_FILL_VALID : CART_FILL_INVALID;
  ctx.strokeStyle = CART_STROKE_COLOR;
  ctx.lineWidth = CART_STROKE_WIDTH;
  ctx.beginPath();
  ctx.roundRect(
    cart_x - CART_HALF_WIDTH,
    cart_y - CART_VERTICAL_OFFSET,
    CART_HALF_WIDTH * 2,
    CART_HEIGHT,
    CART_CORNER_RADIUS,
  );
  ctx.fill();
  ctx.stroke();

  draw_wheel(
    ctx,
    cart_x + WHEEL_LEFT_OFFSET,
    cart_y + WHEEL_VERTICAL_OFFSET,
    wheel_rotation,
  );
  draw_wheel(
    ctx,
    cart_x + WHEEL_RIGHT_OFFSET,
    cart_y + WHEEL_VERTICAL_OFFSET,
    wheel_rotation,
  );
};

// Component
const SimulationCanvas: React.FC<SimulationCanvasProps> = ({
  state,
  target_x,
  params,
  is_valid,
  on_target_change,
}) => {
  const canvas_ref = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvas_ref.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    let animation_frame_id: number;

    const render = () => {
      ctx.fillStyle = CANVAS_BG;
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const center_x = canvas.width / 2;
      const center_y = canvas.height / 2 + CENTER_Y_OFFSET;

      // Grid
      ctx.strokeStyle = GRID_COLOR;
      ctx.lineWidth = GRID_LINE_WIDTH;
      ctx.beginPath();
      for (let i = -GRID_DIVISIONS; i <= GRID_DIVISIONS; i++) {
        const px = center_x + i * (PIXELS_PER_METER * GRID_SPACING_FACTOR);
        ctx.moveTo(px, 0);
        ctx.lineTo(px, canvas.height);
      }
      ctx.stroke();

      // Track
      ctx.strokeStyle = TRACK_COLOR;
      ctx.lineWidth = TRACK_LINE_WIDTH;
      ctx.beginPath();
      ctx.moveTo(0, center_y + TRACK_Y_OFFSET);
      ctx.lineTo(canvas.width, center_y + TRACK_Y_OFFSET);
      ctx.stroke();

      // Target Indicator
      const tx = center_x + target_x * PIXELS_PER_METER;
      ctx.strokeStyle = TARGET_COLOR;
      ctx.setLineDash([TARGET_DASH_LENGTH, TARGET_DASH_LENGTH]);
      ctx.lineWidth = TARGET_LINE_WIDTH;
      ctx.beginPath();
      ctx.moveTo(tx, 0);
      ctx.lineTo(tx, canvas.height);
      ctx.stroke();
      ctx.setLineDash([]);

      // Cart
      const cart_x = center_x + state.x * PIXELS_PER_METER;
      const cart_y = center_y;
      const wheel_rotation = (state.x * PIXELS_PER_METER) / WHEEL_RADIUS;

      draw_cart(ctx, cart_x, cart_y, is_valid, wheel_rotation);

      // Pole
      const pole_end_x =
        cart_x + Math.sin(state.theta) * params.length * PIXELS_PER_METER;
      const pole_end_y =
        center_y -
        POLE_ATTACH_OFFSET -
        Math.cos(state.theta) * params.length * PIXELS_PER_METER;

      ctx.strokeStyle = POLE_COLOR;
      ctx.lineWidth = POLE_WIDTH;
      ctx.lineCap = "round";
      ctx.beginPath();
      ctx.moveTo(cart_x, center_y - POLE_ATTACH_OFFSET);
      ctx.lineTo(pole_end_x, pole_end_y);
      ctx.stroke();

      // Bob
      ctx.fillStyle = is_valid ? BOB_FILL_VALID : BOB_FILL_INVALID;
      ctx.beginPath();
      ctx.arc(pole_end_x, pole_end_y, BOB_RADIUS, 0, Math.PI * 2);
      ctx.fill();

      // Failure Overlay
      if (!is_valid) {
        ctx.fillStyle = `rgba(0,0,0,${FAILURE_OVERLAY_ALPHA})`;
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = FAILURE_TEXT_COLOR;
        ctx.font = FAILURE_TITLE_FONT;
        ctx.textAlign = "center";
        ctx.fillText(
          "STABILITY FAILURE",
          center_x,
          center_y + FAILURE_TITLE_OFFSET,
        );
        ctx.font = FAILURE_SUBTITLE_FONT;
        ctx.fillText(
          "NUMERICAL OVERFLOW DETECTED",
          center_x,
          center_y + FAILURE_SUBTITLE_OFFSET,
        );
      }

      animation_frame_id = requestAnimationFrame(render);
    };
    render();
    return () => cancelAnimationFrame(animation_frame_id);
  }, [state, params, target_x, is_valid]);

  useEffect(() => {
    const handle_resize = (entries: ResizeObserverEntry[]) => {
      for (const entry of entries) {
        const c = entry.target as HTMLCanvasElement;
        c.width = entry.contentRect.width;
        c.height = entry.contentRect.height;
      }
    };

    const observer = new ResizeObserver(handle_resize);
    if (canvas_ref.current) observer.observe(canvas_ref.current);
    return () => observer.disconnect();
  }, []);

  const handle_click = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      const canvas = canvas_ref.current;
      if (!canvas) return;
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const center_x = canvas.width / 2;
      const new_target_x = (x - center_x) / PIXELS_PER_METER;
      on_target_change(
        Math.max(TARGET_MIN, Math.min(TARGET_MAX, new_target_x)),
      );
    },
    [on_target_change],
  );

  return (
    <canvas
      ref={canvas_ref}
      onClick={handle_click}
      style={{
        cursor: "crosshair",
        display: "block",
        width: "100%",
        height: "100%",
      }}
    />
  );
};

export default SimulationCanvas;
