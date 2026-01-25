import "@xyflow/react/dist/style.css";

import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import CustomNodeFlow from "./ExampleCustomNode";
import ConnectionLineFlow from "./ExampleConnectionLine";
import TailwindFlow from "./ExampleTailwind";

const routes = [
  {
    path: "/",
    label: "Custom Node",
    element: <CustomNodeFlow />,
  },
  {
    path: "/connection-line",
    label: "Connection Line",
    element: <ConnectionLineFlow />,
  },
  {
    path: "/tailwind",
    label: "Tailwind",
    element: <TailwindFlow />,
  },
];

const App = () => {
  return (
    <BrowserRouter>
      <div
        style={{
          width: "100vw",
          height: "100vh",
          display: "flex",
          flexDirection: "column",
        }}
      >
        <nav style={{ padding: "10px", background: "#333", color: "white" }}>
          <ul
            style={{
              display: "flex",
              listStyle: "none",
              gap: "20px",
              margin: 0,
              padding: 0,
            }}
          >
            {routes.map((route) => (
              <li key={route.path}>
                <Link
                  style={{ color: "white", textDecoration: "none" }}
                  to={route.path}
                >
                  {route.label}
                </Link>
              </li>
            ))}
          </ul>
        </nav>
        <div style={{ flex: 1, position: "relative" }}>
          <Routes>
            {routes.map((route) => (
              <Route
                key={route.path}
                path={route.path}
                element={route.element}
              />
            ))}
          </Routes>
        </div>
      </div>
    </BrowserRouter>
  );
};

export default App;
