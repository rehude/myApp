import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { setToken as apiSetToken } from "../api/tauri";
import { useAppStore } from "../store/useAppStore";

function LoginPage() {
  const [token, setTokenInput] = useState("");
  const [error, setError] = useState("");
  const navigate = useNavigate();
  const { setToken } = useAppStore();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!token.trim()) {
      setError("Please enter a token");
      return;
    }

    const response = await apiSetToken(token.trim());
    if (response.success) {
      setToken(token.trim());
      navigate("/repos");
    } else {
      setError(response.error || "Failed to set token");
    }
  };

  return (
    <div className="login-container">
      <div className="login-card">
        <h1>GitHub Repo Viewer</h1>
        <p className="subtitle">Enter your GitHub Personal Access Token</p>
        <form onSubmit={handleSubmit}>
          <input
            type="password"
            value={token}
            onChange={(e) => setTokenInput(e.target.value)}
            placeholder="ghp_xxxxxxxxxxxx"
            className="token-input"
          />
          {error && <p className="error">{error}</p>}
          <button type="submit" className="btn-primary">Login</button>
        </form>
        <p className="hint">
          Need a token? Create one at GitHub → Settings → Developer settings → Personal access tokens
        </p>
      </div>
    </div>
  );
}

export default LoginPage;
