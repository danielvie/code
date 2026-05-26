import "./App.css";
import { Dashboard } from "./components/Dashboard";
import { Header } from "./components/Header";
import { SettingsProvider } from "./hooks/Settings/SettingsProvider";
// import useWindowSize from "./hooks/useWindowSize";

function App() {
  // const { width } = useWindowSize();

  return (
    <SettingsProvider>
      <Header />
      <Dashboard />
    </SettingsProvider>
  );
}

export default App;
