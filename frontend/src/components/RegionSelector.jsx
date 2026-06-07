export function RegionSelector({ regions, onSelect }) {
  return (
    <div className="region-selector">
      <strong>Select a region:</strong>
      <div className="region-selector">
        {regions.map((id) => (
          <button onClick={() => onSelect(id)}>{id}</button>
        ))}
      </div>
    </div>
  );
}
