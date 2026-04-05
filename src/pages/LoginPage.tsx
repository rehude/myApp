import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { saveAccount, setCurrentAccount, getAccounts } from "../api/tauri";
import { useAppStore, Provider } from "../store/useAppStore";

function LoginPage() {
  const navigate = useNavigate();
  const { provider, githubAccounts, gitlabAccounts, setAccounts, addAccount, setProvider, setLoggedIn } = useAppStore();

  const [loginMode, setLoginMode] = useState<"select" | "new">("select");
  const [selectedAccountId, setSelectedAccountId] = useState<string | null>(null);
  const [newProvider, setNewProvider] = useState<Provider>(provider);
  const [newToken, setNewToken] = useState("");
  const [newLabel, setNewLabel] = useState("");
  const [newGitlabUrl, setNewGitlabUrl] = useState("https://gitlab.com");
  const [rememberAccount, setRememberAccount] = useState(true);
  const [error, setError] = useState("");
  const [saving, setSaving] = useState(false);

  const currentAccounts = newProvider === "github" ? githubAccounts : gitlabAccounts;

  const handleSelectExisting = async () => {
    if (!selectedAccountId) {
      setError("Please select an account");
      return;
    }

    const res = await setCurrentAccount(selectedAccountId);
    if (res.success) {
      // Refresh accounts list
      const github = await getAccounts("github");
      const gitlab = await getAccounts("gitlab");
      setAccounts(github, gitlab);
      setLoggedIn(true);
      setProvider(newProvider);
      navigate("/repos");
    } else {
      setError(res.error || "Failed to select account");
    }
  };

  const handleAddNew = async () => {
    if (!newToken.trim()) {
      setError("Please enter a token");
      return;
    }

    setSaving(true);
    setError("");

    const label = newLabel.trim() || (newProvider === "github" ? `GitHub Account ${githubAccounts.length + 1}` : `GitLab Account ${gitlabAccounts.length + 1}`);
    const gitlabUrl = newProvider === "gitlab" ? newGitlabUrl : null;

    const res = await saveAccount(newProvider, newToken.trim(), gitlabUrl, label);

    if (res.success && res.data) {
      addAccount(res.data);
      // Refresh accounts list
      const github = await getAccounts("github");
      const gitlab = await getAccounts("gitlab");
      setAccounts(github, gitlab);
      setLoggedIn(true);
      setProvider(newProvider);
      navigate("/repos");
    } else {
      setError(res.error || "Failed to save account");
    }

    setSaving(false);
  };

  const handleProviderChange = (p: Provider) => {
    setNewProvider(p);
    setLoginMode("select");
    setSelectedAccountId(null);
  };

  return (
    <div className="login-container">
      <div className="login-card">
        <h1>Git Repo Viewer</h1>
        <p className="subtitle">Sign in to continue</p>

        {/* Provider Selector */}
        <div className="provider-select">
          <label>Provider</label>
          <select
            value={newProvider}
            onChange={(e) => handleProviderChange(e.target.value as Provider)}
            className="select-input"
          >
            <option value="github">GitHub</option>
            <option value="gitlab">GitLab</option>
          </select>
        </div>

        {/* Login Mode Toggle */}
        <div className="login-mode-toggle">
          <button
            className={`toggle-btn ${loginMode === "select" ? "active" : ""}`}
            onClick={() => setLoginMode("select")}
          >
            Use Existing Account
          </button>
          <button
            className={`toggle-btn ${loginMode === "new" ? "active" : ""}`}
            onClick={() => setLoginMode("new")}
          >
            Add New Account
          </button>
        </div>

        {/* Existing Accounts */}
        {loginMode === "select" && (
          <div className="account-select-section">
            {currentAccounts.length > 0 ? (
              <div className="account-list">
                {currentAccounts.map((account) => (
                  <div
                    key={account.id}
                    className={`account-option ${selectedAccountId === account.id ? "selected" : ""}`}
                    onClick={() => setSelectedAccountId(account.id)}
                  >
                    <input
                      type="radio"
                      name="account"
                      checked={selectedAccountId === account.id}
                      onChange={() => setSelectedAccountId(account.id)}
                    />
                    <div className="account-info">
                      <span className="account-label">{account.label}</span>
                      {account.gitlab_url && (
                        <span className="account-url">{account.gitlab_url}</span>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <p className="no-accounts">No saved {newProvider} accounts</p>
            )}
            {selectedAccountId && (
              <button onClick={handleSelectExisting} className="btn-primary">
                Sign In
              </button>
            )}
          </div>
        )}

        {/* New Account Form */}
        {loginMode === "new" && (
          <div className="new-account-section">
            {newProvider === "gitlab" && (
              <div className="form-group">
                <label>GitLab URL</label>
                <input
                  type="text"
                  value={newGitlabUrl}
                  onChange={(e) => setNewGitlabUrl(e.target.value)}
                  placeholder="https://gitlab.com"
                  className="token-input"
                />
              </div>
            )}

            <div className="form-group">
              <label>Account Label (optional)</label>
              <input
                type="text"
                value={newLabel}
                onChange={(e) => setNewLabel(e.target.value)}
                placeholder={newProvider === "github" ? "e.g., Work Account" : "e.g., Company GitLab"}
                className="token-input"
              />
            </div>

            <div className="form-group">
              <label>Personal Access Token</label>
              <input
                type="password"
                value={newToken}
                onChange={(e) => setNewToken(e.target.value)}
                placeholder={newProvider === "github" ? "ghp_xxxxxxxxxxxx" : "glpat-xxxxxxxxxxxx"}
                className="token-input"
              />
            </div>

            <div className="form-group checkbox-group">
              <label>
                <input
                  type="checkbox"
                  checked={rememberAccount}
                  onChange={(e) => setRememberAccount(e.target.checked)}
                />
                <span>Remember this account</span>
              </label>
            </div>

            <button
              onClick={handleAddNew}
              className="btn-primary"
              disabled={saving}
            >
              {saving ? "Signing In..." : "Sign In"}
            </button>
          </div>
        )}

        {error && <p className="error">{error}</p>}

        <p className="hint">
          {newProvider === "github"
            ? "Create a token at GitHub → Settings → Developer settings → Personal access tokens"
            : "Create a token at GitLab → Settings → Access Tokens (needs read_api scope)"}
        </p>
      </div>
    </div>
  );
}

export default LoginPage;
