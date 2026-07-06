export function SubscribeError({ message }) {
  if (!message) return null;

  return (
    <div className="subscribe-error-banner" role="alert">
      <h2>Server not configured:</h2>
      <p>{message}</p>
    </div>
  );
}
