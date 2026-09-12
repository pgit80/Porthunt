import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [ports, setPorts] = useState([]);
  const [status, setStatus] = useState("Loading...");
  const [expandedPid, setExpandedPid] = useState(null);
  const [actionMessage, setActionMessage] = useState(null);
  const [actionError, setActionError] = useState(null);
  
  // New state for filtering
  const [searchQuery, setSearchQuery] = useState("");

  const fetchPorts = async () => {
    try {
      const result = await invoke("get_listening_ports");
      setPorts(result);
    } catch (err) {
      console.error("Failed to fetch ports:", err);
    }
  };

  const checkStatus = async () => {
    try {
      const res = await invoke("get_core_status");
      setStatus("Connected");
    } catch (err) {
      setStatus("Disconnected");
    }
  };

  const handleKillProcess = async (pid, processName, port) => {
    const confirmKill = window.confirm(
      `Are you sure you want to kill '${processName}' (PID ${pid}) running on port ${port}?`
    );

    if (!confirmKill) return;

    setActionMessage(null);
    setActionError(null);

    try {
      const response = await invoke("kill_process_by_pid", { pid });
      setActionMessage(response);
      fetchPorts();
    } catch (err) {
      setActionError(String(err));
    }
  };

  useEffect(() => {
    checkStatus();
    fetchPorts();
    // Auto-refresh ports every 10 seconds
    const interval = setInterval(fetchPorts, 10000);
    return () => clearInterval(interval);
  }, []);

  const toggleExpand = (pid) => {
    setExpandedPid(expandedPid === pid ? null : pid);
  };

  // Filter logic
  const filteredPorts = useMemo(() => {
    const query = searchQuery.toLowerCase();
    return ports.filter(
      (item) =>
        item.port.toString().includes(query) ||
        item.processName.toLowerCase().includes(query) ||
        item.appType.toLowerCase().includes(query)
    );
  }, [ports, searchQuery]);

  return (
    <div className="container">
      <header className="header">
        <div>
          <h1>Porthunt</h1>
          <div className="status-indicator">
            <span className={`dot ${status === "Connected" ? "green" : "red"}`}></span>
            Core: {status}
          </div>
        </div>
        <div className="controls">
          <input
            type="text"
            placeholder="Search port, process, or app..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="search-input"
          />
          <button onClick={fetchPorts} className="btn-primary">
            Refresh
          </button>
        </div>
      </header>

      {actionMessage && <div className="alert alert-success">{actionMessage}</div>}
      {actionError && <div className="alert alert-error">{actionError}</div>}

      <div className="table-container">
        <table className="styled-table">
          <thead>
            <tr>
              <th>Port</th>
              <th>Protocol</th>
              <th>PID</th>
              <th>Process Name</th>
              <th>App Identification</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredPorts.length === 0 ? (
              <tr>
                <td colSpan="7" style={{ textAlign: "center", padding: "20px", color: "#666" }}>
                  No active ports match your search.
                </td>
              </tr>
            ) : (
              filteredPorts.map((item, idx) => (
                <div key={`${item.port}-${item.pid}-${idx}`}style={{ display: "contents" }}>
                  <tr>
                    <td className="font-bold text-lg">{item.port}</td>
                    <td>{item.protocol}</td>
                    <td>{item.pid}</td>
                    <td className="font-medium">{item.processName}</td>
                    <td>
                      <span className={`badge ${item.appType !== "Generic Process" ? "badge-blue" : "badge-gray"}`}>
                        {item.appType}
                      </span>
                    </td>
                    <td><span className="status-listening">{item.status}</span></td>
                    <td className="actions-cell">
                      <button onClick={() => toggleExpand(item.pid)} className="btn-secondary">
                        {expandedPid === item.pid ? "Hide" : "Details"}
                      </button>
                      <button 
                        onClick={() => handleKillProcess(item.pid, item.processName, item.port)}
                        className="btn-danger"
                      >
                        Kill
                      </button>
                    </td>
                  </tr>
                  {expandedPid === item.pid && (
                    <tr className="details-row">
                      <td colSpan="7">
                        <div className="details-card">
                          <p><strong>Executable:</strong> <code>{item.exePath}</code></p>
                          <p><strong>Command:</strong> <code>{item.commandLine}</code></p>
                        </div>
                      </td>
                    </tr>
                  )}
                </div>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default App;