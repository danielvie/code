import type { Message } from "../types";

export const ChatBubbleSent = ({ message }: { message: Message }) => (
  <div className="flex justify-end">
    <div className="max-w-[75%] px-4 py-2 bg-blue-600 text-white rounded-2xl rounded-tr-none shadow-sm">
      {message.text}
    </div>
  </div>
);

export const ChatBubbleReply = ({ message }: { message: Message }) => (
  <div className="flex justify-start">
    <div className="max-w-[75%] px-4 py-2 bg-white text-gray-800 border border-gray-200 rounded-2xl rounded-tl-none shadow-sm">
      {message.text}
    </div>
  </div>
);
