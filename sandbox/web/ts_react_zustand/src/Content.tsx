import useStore from "./store";

export default function Content() {
  const activeTab = useStore((state) => state.activeTab);

  return (
    <main className="p-4">
      <h1>
        Current View:{" "}
        <span className="bg-green-700 text-gray-200 p-1 rounded-xs">
          {activeTab}
        </span>
      </h1>
    </main>
  );
}
