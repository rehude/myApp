import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { setProvider, setGithubToken, setGitlabToken, setGitlabUrl } from "../api/tauri";
import { useAppStore, Provider } from "../store/useAppStore";

function LoginPage() {
  const [provider, setProviderState] = useState<Provider>("github");
  const [gitlabUrl, setGitlabUrlState] = useState("https://gitlab.com");
  const [token, setTokenInput] = useState("");
  const [error, setError] = useState("");
  const navigate = useNavigate();
  const { setProvider: setStoreProvider, setGithubToken: setStoreGithubToken, setGitlabToken: setStoreGitlabToken, setGitlabUrl: setStoreGitlabUrl } = useAppStore();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!token.trim()) {
      setError("Please enter a token");
      return;
    }

    // Set provider first
    const providerResponse = await setProvider(provider);
    if (!providerResponse.success) {
      setError(providerResponse.error || "Failed to set provider");
      return;
    }

    // For GitLab, set the URL first
    if (provider === "gitlab") {
      const urlResponse = await setGitlabUrl(gitlabUrl);
      if (!urlResponse.success) {
        setError(urlResponse.error || "Failed to set GitLab URL");
        return;
      }
      setStoreGitlabUrl(gitlabUrl);
    }

    // Set token based on provider
    const tokenResponse = provider === "github"
      ? await setGithubToken(token.trim())
      : await setGitlabToken(token.trim());

    if (tokenResponse.success) {
      setStoreProvider(provider);
      if (provider === "github") {
        setStoreGithubToken(token.trim());
      } else {
        setStoreGitlabToken(token.trim());
      }
      navigate("/repos");
    } else {
      setError(tokenResponse.error || "Failed to set token");
    }
  };

  return (
    <div className="login-container">
      <div className="login-card">
        <h1>Git Repo Viewer</h1>
        <p className="subtitle">Enter your credentials to continue</p>
        <form onSubmit={handleSubmit}>
          <div className="provider-select">
            <label>Provider</label>
            <select
              value={provider}
              onChange={(e) => setProviderState(e.target.value as Provider)}
              className="select-input"
            >
              <option value="github">GitHub</option>
              <option value="gitlab">GitLab</option>
            </select>
          </div>

          {provider === "gitlab" && (
            <div className="provider-select">
              <label>GitLab URL</label>
              <input
                type="text"
                value={gitlabUrl}
                onChange={(e) => setGitlabUrlState(e.target.value)}
                placeholder="https://gitlab.com"
                className="token-input"
              />
            </div>
          )}

          <input
            type="password"
            value={token}
            onChange={(e) => setTokenInput(e.target.value)}
            placeholder={provider === "github" ? "ghp_xxxxxxxxxxxx" : "glpat-xxxxxxxxxxxx"}
            className="token-input"
          />
          {error && <p className="error">{error}</p>}
          <button type="submit" className="btn-primary">Login</button>
        </form>
        <p className="hint">
          {provider === "github"
            ? "Create a token at GitHub → Settings → Developer settings → Personal access tokens"
            : "Create a token at GitLab → Settings → Access Tokens (needs read_api scope)"}
        </p>
      </div>
    </div>
  );
}

export default LoginPage;
