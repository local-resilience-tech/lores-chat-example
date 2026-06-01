export function MessageList({ messages }) {
  return (
    <ul className="message-list">
      {messages.map((msg, i) => (
        <li key={i} className={msg.from === "me" ? "message-me" : "message-server"}>
          {msg.text}
        </li>
      ))}
    </ul>
  );
}
