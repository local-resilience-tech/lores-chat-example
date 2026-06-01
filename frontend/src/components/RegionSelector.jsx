export function RegionSelector({ regions, onSelect }) {
  return (
    <div className="region-selector">
      {regions.map((id) => (
        <button key={id} onClick={() => onSelect(id)}>
          {id}
        </button>
      ))}
    </div>
  );
}
