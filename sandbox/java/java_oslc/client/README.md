# React + TypeScript + Vite (OSLC Client)

This is the **React** client for the OSLC proof-of-concept. It’s a Vite + React + TypeScript app using Tailwind CSS, and it talks to the Java OSLC server via Vite’s dev proxy.

## Prerequisites

- Node.js (recommended: current LTS)
- npm

## Install

From the repo root:

```sh
cd client
npm install
```

If your environment blocks very new package versions, this project pins `rollup` via npm `overrides` to avoid policy issues.

## Run (dev)

Start the Java server on `http://127.0.0.1:8080` first.

Then:

```sh
cd client
npm run dev
```

Vite runs on `http://127.0.0.1:3000`.

### API proxy

The dev server proxies:

- `/catalog` → `http://127.0.0.1:8080`
- `/provider` → `http://127.0.0.1:8080`

So the React app can call `fetch("/catalog")` and `fetch("/provider/1/defects")` without CORS setup.

## Build

```sh
cd client
npm run build
npm run preview
```

## Typecheck

```sh
cd client
npm run check
```

## Test

```sh
cd client
npm test
```

## Notes

- Entry file: `src/main.tsx`
- Main UI: `src/App.tsx`
- Defect UI component: `src/DefectCard.tsx`
- Tailwind styles: `src/app.css`
