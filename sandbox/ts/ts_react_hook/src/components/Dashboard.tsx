import { useSettings } from "../hooks/Settings/useSettings";

export function Dashboard() {
  // We read the theme to style the background
  const { theme } = useSettings();

  // Dynamic styles based on shared state
  const containerStyle = {
    padding: "50px",
    height: "100vh",
    transition: "all 0.3s ease", // Smooth transition
    backgroundColor: theme === "light" ? "oklch(0.85 0 0)" : "oklch(0.21 0 0)",
    color: theme === "light" ? "oklch(0.21 0 0)" : "oklch(0.85 0 0)",
  };

  return (
    <div style={containerStyle}>
      <h2>Dashboard Overview</h2>
      <p>
        Because you are using Context, I know exactly what the Header is doing!
      </p>

      <div
        style={{
          marginTop: "20px",
          padding: "20px",
          border: "1px dashed gray",
        }}
      >
        Status: <strong>{theme} mode active</strong>
      </div>
    </div>
  );
}
