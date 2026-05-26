import { useAtomValue } from "jotai";
import { textAtom, charCountAtom } from "./state";

// Component B: The Listener
export const DisplayPanel = () => {
  const text = useAtomValue(textAtom);
  const count = useAtomValue(charCountAtom);

  return (
    <div className="p-4 border rounded-lg shadow-sm bg-blue-50 dark:bg-blue-900 mt-4">
      <h2 className="text-sm font-bold mb-2">Display Component</h2>
      <p className="text-lg">
        Value: <span className="font-mono">{text || "(empty)"}</span>
      </p>
      <p className="text-xs text-gray-400 mt-2">Character count: {count}</p>
    </div>
  );
};
