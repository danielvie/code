import { useState, useEffect, useRef } from "react";
import type { Message } from "./types";
import { ChatBubbleSent, ChatBubbleReply } from "./components/ChatBubbles";
import { ChatInput } from "./components/ChatInput";
import "./App.css";

function App() {
  const [messages, setMessages] = useState<Message[]>([]);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    scrollRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSendMessage = (text: string) => {
    const newUserMsg: Message = { id: Date.now(), text, sender: "user" };
    setMessages((prev) => [...prev, newUserMsg]);

    // Mock response logic
    setTimeout(() => {
      const botReply: Message = {
        id: Date.now() + 1,
        text: text.toUpperCase(),
        sender: "bot",
      };
      setMessages((prev) => [...prev, botReply]);
    }, 600);
  };

  return (
    <div className="w-screen h-screen">
      <div className="flex flex-col w-full h-full bg-gray-100 mx-auto border-x shadow-xl text-gray-900">
        <header className="p-4 bg-white border-b font-bold text-center text-blue-600">
          MODULAR CHAT
        </header>

        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {messages.map((msg) =>
            msg.sender === "user" ? (
              <ChatBubbleSent key={msg.id} message={msg} />
            ) : (
              <ChatBubbleReply key={msg.id} message={msg} />
            ),
          )}
          <div ref={scrollRef} />
        </div>

        <ChatInput onSendMessage={handleSendMessage} />
      </div>
    </div>
  );
}

export default App;
