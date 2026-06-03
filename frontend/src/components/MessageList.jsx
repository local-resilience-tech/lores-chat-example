export function MessageList({ messages }) {
  return (
    <ul className="message-list">
      {messages.map((msg, i) => (
        <li key={i} className={msg.from === "me" ? "message-me" : "message-server"}>
          {msg.author_node && <span className="message-author">{msg.author_node.slice(0, 8)}</span>}
          {msg.text}
        </li>
      ))}
    </ul>
  );
}
