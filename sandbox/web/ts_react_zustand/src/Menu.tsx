import useStore from "./store";

export default function Menu() {
  const setTab = useStore((state) => state.setTab);

  return (
    <div className="flex gap-2">
      <button onClick={() => setTab("Home")}>Home</button>
      <button onClick={() => setTab("Profile")}>Profile</button>
      <button onClick={() => setTab("Settings")}>Settings</button>
    </div>
  );
}
