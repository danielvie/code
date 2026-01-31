import { InputPanel } from "./InputPanel";
import { DisplayPanel } from "./DisplayPanel";
import "./App.css";

// Main Page
export default function App() {
  return (
    <main className="mx-auto mt-10 p-6">
      <h1 className="text-2xl font-bold mb-6 text-center">
        Jotai Shared State
      </h1>
      <InputPanel />
      <DisplayPanel />
    </main>
  );
}
