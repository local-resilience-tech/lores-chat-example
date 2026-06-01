import { useState } from "react";
import useWebSocket from "react-use-websocket";
import "./App.css";
import { ServerIndicator } from "./components/ServerIndicator";

const WS_URL = `${window.location.protocol === "https:" ? "wss:" : "ws:"}//${window.location.host}/ws`;

export function App() {
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");
  const [hasError, setHasError] = useState(false);
  const { sendMessage, readyState } = useWebSocket(WS_URL, {
    onError: () => setHasError(true),
    onOpen: () => setHasError(false),
    onMessage: (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === "error") {
          setHasError(true);
          return;
        }
      } catch {
        // non-JSON messages fall through
      }
      console.log("[subscribe] received message from server:", event.data);
      setMessages((prev) => [...prev, { text: event.data, from: "server" }]);
    },
  });

  function handleSubmit(e) {
    e.preventDefault();
    if (!input.trim()) return;
    setHasError(false);
    console.log("[publish] sending message to backend:", input);
    sendMessage(input);
    setInput("");
  }

  return (
    <div className="app on-light-blue">
      <div className="header">
        <div className="title">
          <h1>Lores Chat Example</h1>
          <ServerIndicator readyState={readyState} hasError={hasError} />
        </div>
        <form onSubmit={handleSubmit} className="message-form">
          <input type="text" value={input} onChange={(e) => setInput(e.target.value)} placeholder="Type a message..." />
          <button type="submit">Send</button>
        </form>
      </div>

      <ul className="message-list">
        {messages.map((msg, i) => (
          <li key={i} className={msg.from === "me" ? "message-me" : "message-server"}>
            {msg.text}
          </li>
        ))}
      </ul>
    </div>
  );
}
