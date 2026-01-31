import { useSetAtom } from "jotai";
import { textAtom } from "./state";

// Component A: The Updater
export const InputPanel = () => {
  const setText = useSetAtom(textAtom);

  return (
    <div className="p-4 border rounded-lg shadow-sm bg-white dark:bg-gray-800">
      <h2 className="text-sm font-bold mb-2">Input Component</h2>
      <input
        type="text"
        className="w-full p-2 border rounded text-black"
        placeholder="Type to update shared state..."
        onChange={(e) => setText(e.target.value)}
      />
    </div>
  );
};
