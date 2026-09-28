# Cart-Pole Lab

An interactive inverted-pendulum experiment built with Three.js. The pendulum starts hanging down. Click **Begin swing-up** to add energy with cart motion; near upright, the simulation switches to an LQR feedback controller. Left-drag outside the cart stand to orbit, or middle-drag anywhere. Scroll to zoom. Hover over the stand, including the space between the rail and its feet, to preview a target. Click or drag inside that slice to send the cart there.

## Run locally

```bash
npm install
npm run dev
```

Open the URL Vite prints. Run `npm test` for physics checks or `npm run build` for a production bundle.

## What is modeled

The cart-pole is a nonlinear point-mass pendulum on a massless rod. A fixed-step RK4 integrator advances the four states: cart position and velocity, pole angle and angular velocity. Angle zero is upright; π is down. The swing-up controller pumps energy toward the upright energy level, with a small excess to offset damping. A discrete-time Riccati iteration recomputes the LQR gain whenever a parameter changes. LQR controls only near upright; when the pole moves too far away, swing-up resumes. Motor force is capped and the rail has end stops.

The control desk shows LQR priorities for position error, cart velocity, pole angle, and pole angular velocity, plus an effort penalty. Open Advanced settings to change physical parameters or choose a preset. Clicking or dragging within the stand sets a position target within ±2.6 m; LQR uses it on the next simulation step and moves the balanced cart to it. You can also set a target before starting the swing-up. The left and right push buttons add a short disturbance while the simulation is running. Reset returns the system and target to their initial state.

This is a teaching simulation, not a hardware-ready controller. The rod has no mass, the motor has no dynamics, the rail collisions are simplified, and the LQR design uses a linearization around upright.
