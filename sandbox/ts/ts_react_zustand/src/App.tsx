import "./App.css";
import Menu from "./Menu";
import Content from "./Content";

function App() {
  return (
    <>
      <div className="bg-[oklch(0.18_0_0)] p-2 rounded-md border-2 border-[oklch(0_0_0)]">
        <Menu />
        <Content />
      </div>
    </>
  );
}

export default App;
