import { useSettings } from "../hooks/Settings/useSettings";

export function Header() {
  // We only need the toggle function and current theme string here
  const { theme, toggleTheme } = useSettings();

  return (
    <header
      style={{
        padding: "20px",
        borderBottom: "2px solid #ccc",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
      }}
    >
      <h1>MyApp</h1>

      <button
        onClick={toggleTheme}
        style={{ padding: "8px 16px", height: "4rem" }}
      >
        {/* The text updates automatically when theme changes */}
        {theme.toUpperCase()}
      </button>
    </header>
  );
}
