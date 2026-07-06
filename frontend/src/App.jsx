import { useState, useMemo } from "react";
import useWebSocket from "react-use-websocket";
import "./App.css";
import { ServerIndicator } from "./components/ServerIndicator";
import { UserIdentity } from "./components/UserIdentity";
import { MessageSender } from "./components/MessageSender";
import { MessageList } from "./components/MessageList";
import { SubscribeError } from "./components/SubscribeError";
import { generateIdentity } from "./identity";

const WS_URL = `${window.location.protocol === "https:" ? "wss:" : "ws:"}//${window.location.host}/ws`;

export function App() {
  const identity = useMemo(() => {
    const stored = localStorage.getItem("identity");
    if (stored) {
      try {
        return JSON.parse(stored);
      } catch {
        // corrupted, generate new
      }
    }
    const newIdentity = generateIdentity();
    localStorage.setItem("identity", JSON.stringify(newIdentity));
    return newIdentity;
  }, []);
  const [messages, setMessages] = useState([]);
  const [hasError, setHasError] = useState(false);
  const [subscribeError, setSubscribeError] = useState(null);

  const { sendMessage, readyState } = useWebSocket(WS_URL, {
    onError: () => setHasError(true),
    onOpen: () => {
      setHasError(false);
      setSubscribeError(null);
    },
    onMessage: (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === "error") {
          setHasError(true);
          return;
        }
        if (data.type === "subscribe_ok") {
          setSubscribeError(null);
          return;
        }
        if (data.type === "subscribe_error") {
          const match = data.message.match(/message:\s*"([^"]+)"/);
          setSubscribeError(match ? match[1] : data.message);
          return;
        }
        if (data.author_node !== undefined && data.text !== undefined) {
          console.log("[subscribe] received message from server:", event.data);
          let text = data.text;
          let senderIdentity = null;
          try {
            const payload = JSON.parse(data.text);
            if (payload.text !== undefined && payload.identity !== undefined) {
              text = payload.text;
              senderIdentity = payload.identity;
            }
          } catch {
            // plain text message, no identity
          }
          setMessages((prev) => [...prev, { text, identity: senderIdentity, author_node: data.author_node, from: "server" }]);
          return;
        }
      } catch {
        // non-JSON messages fall through
      }
      console.log("[subscribe] received message from server:", event.data);
      setMessages((prev) => [...prev, { text: event.data, from: "server" }]);
    },
  });

  function handleSend(text) {
    setHasError(false);
    const payload = JSON.stringify({ identity, text });
    console.log("[publish] sending message to backend:", payload);
    sendMessage(payload);
  }

  return (
    <div className="app on-light-blue">
      <SubscribeError message={subscribeError} />
      <div className="header">
        <div className="title">
          <h1>Lores Chat Example</h1>
          <div className="title-right">
            <UserIdentity identity={identity} />
            <ServerIndicator readyState={readyState} hasError={hasError} />
          </div>
        </div>
        <MessageSender onSend={handleSend} />
      </div>

      <MessageList messages={messages} />
    </div>
  );
}
