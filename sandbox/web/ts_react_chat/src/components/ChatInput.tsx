import { useState, type SyntheticEvent } from "react";

interface ChatInputProps {
  onSendMessage: (text: string) => void;
}

export const ChatInput = ({ onSendMessage }: ChatInputProps) => {
  const [text, setText] = useState("");

  function handle_submit(e: SyntheticEvent<HTMLFormElement>) {
    e.preventDefault(); // prevent the browser to reload on submit
    if (!text.trim()) return;

    onSendMessage(text);
    setText("");
  }

  return (
    <form
      onSubmit={handle_submit}
      className="p-4 bg-gray-900 border-t flex gap-2"
    >
      <input
        type="text"
        className="flex-1 border bg-gray-200 border-gray-300 rounded-md px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none"
        placeholder="Type a message..."
        value={text}
        onChange={(e) => setText(e.target.value)}
      />
      <button
        type="submit"
        className="bg-blue-600 hover:bg-blue-700 text-white p-2 px-6 rounded-md transition-colors font-medium"
      >
        Send
      </button>
    </form>
  );
};
