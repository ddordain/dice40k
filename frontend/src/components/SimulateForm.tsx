import { createSignal } from "solid-js";
import "./simulate-form.css"; // Import the CSS file

const SimulateForm = () => {
  const [numAttacks, setNumAttacks] = createSignal();
  const [minToHit, setMinToHit] = createSignal();
  const [minToWound, setMinToWound] = createSignal();
  const [minToSave, setMinToSave] = createSignal();
  const [thresholdPercent, setThresholdPercent] = createSignal(80);
  const [result, setResult] = createSignal(null);

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    const payload = {
      num_attacks: numAttacks(),
      min_to_hit: minToHit(),
      min_to_wound: minToWound(),
      min_to_save: minToSave(),
      threshold_percent: thresholdPercent(),
    };

    try {
      const response = await fetch("http://127.0.0.1:8000/simulate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      const data = await response.json();
      setResult(data);
    } catch (error) {
      console.error("Error fetching data:", error);
      setResult({ error: "Failed to fetch results" });
    }
  };

  return (
    <div className="form-container">
      <form onSubmit={handleSubmit} className="simulate-form">
        <div className="form-group">
          <label>
            Number of Attacks:
            <input
              type="number"
              value={numAttacks()}
              onInput={(e) => setNumAttacks(Number(e.currentTarget.value))}
            />
          </label>
        </div>
        <div className="form-group">
          <label>
            Min to Hit:
            <input
              type="number"
              value={minToHit()}
              onInput={(e) => setMinToHit(Number(e.currentTarget.value))}
            />
          </label>
        </div>
        <div className="form-group">
          <label>
            Min to Wound:
            <input
              type="number"
              value={minToWound()}
              onInput={(e) => setMinToWound(Number(e.currentTarget.value))}
            />
          </label>
        </div>
        <div className="form-group">
          <label>
            Min to Save:
            <input
              type="number"
              value={minToSave()}
              onInput={(e) => setMinToSave(Number(e.currentTarget.value))}
            />
          </label>
        </div>
        <div className="form-group">
          <label>
            Threshold Percent:
            <input
              type="number"
              step="0.1"
              value={thresholdPercent()}
              onInput={(e) => setThresholdPercent(Number(e.currentTarget.value))}
            />
          </label>
        </div>
        <button type="submit" className="simulate-btn">Simulate</button>
      </form>
      {result() && (
        <div className="result-container">
          <h3>Results:</h3>
          {result().error ? (
            <p className="error">{result().error}</p>
          ) : (
            <div>
              <p>Average Successful Attacks: {result().average_successful_attacks}</p>
              <p>Threshold Successful Attacks: {result().threshold_successful_attacks}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default SimulateForm;
