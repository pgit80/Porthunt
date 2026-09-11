import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [coreStatus, setCoreStatus] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const checkCoreStatus = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await invoke("get_core_status");
      setCoreStatus(response);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: "2rem", fontFamily: "sans-serif" }}>
      <h1>Porthunt</h1>
      <p>Local Developer Port & Process Manager</p>

      <div style={{ marginTop: "2rem", padding: "1rem", border: "1px solid #ccc", borderRadius: "8px" }}>
        <h3>System IPC Status</h3>
        <p>
          Rust Core:{" "}
          {coreStatus ? (
            <span style={{ color: "green", fontWeight: "bold" }}>✅ Connected</span>
          ) : (
            <span style={{ color: "orange", fontWeight: "bold" }}>❌ Not checked</span>
          )}
        </p>

        {coreStatus && (
          <blockquote style={{ background: "#f0f0f0", padding: "0.5rem 1rem", borderRadius: "4px" }}>
            {coreStatus}
          </blockquote>
        )}

        {error && <p style={{ color: "red" }}>Error: {error}</p>}

        <button 
          onClick={checkCoreStatus} 
          disabled={loading}
          style={{ padding: "0.5rem 1rem", cursor: "pointer" }}
        >
          {loading ? "Checking..." : "Check Rust Core"}
        </button>
      </div>
    </div>
  );
}

export default App;