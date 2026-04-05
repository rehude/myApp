import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { getRepos, logout as apiLogout, setCurrentAccount, getAccounts, saveAccount, setProvider as apiSetProvider, getCurrentState } from "../api/tauri";
import { useAppStore, Provider } from "../store/useAppStore";
import { Repo } from "../types";

function RepoListPage() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [showAccountModal, setShowAccountModal] = useState(false);
  const [selectedAccountId, setSelectedAccountId] = useState<string | null>(null);
  const [newToken, setNewToken] = useState("");
  const [newLabel, setNewLabel] = useState("");
  const [newGitlabUrl, setNewGitlabUrl] = useState("https://gitlab.com");
  const [addNewMode, setAddNewMode] = useState(false);
  const navigate = useNavigate();
  const { repos, setRepos, setSelectedRepo, setProvider, provider, setAccounts, addAccount, removeAccount, githubAccounts, gitlabAccounts, setLoggedIn } = useAppStore();

  useEffect(() => {
    loadRepos();
  }, []);

  const handleRepoClick = (repo: Repo) => {
    setSelectedRepo(repo);
    navigate(`/repo/${repo.id}/commits`);
  };

  const loadRepos = async () => {
    setLoading(true);
    setError("");
    const response = await getRepos();
    setLoading(false);
    if (response.success && response.data) {
      setRepos(response.data);
    } else {
      if (response.code === "UNAUTHORIZED" || response.code === "NOT_LOGGED_IN") {
        handleLogout();
      } else {
        setError(response.error || "Failed to load repositories");
      }
    }
  };

  const handleLogout = async () => {
    await apiLogout();
    setLoggedIn(false);
    navigate("/");
  };

  const handleSwitchProvider = async (newProvider: Provider) => {
    setProvider(newProvider);
    await apiSetProvider(newProvider);
    loadRepos();
  };

  const handleSwitchAccount = async () => {
    if (!selectedAccountId) return;

    const res = await setCurrentAccount(selectedAccountId);
    if (res.success) {
      // Refresh state from backend
      const state = await getCurrentState();
      setProvider(state.provider as Provider);
      setAccounts(state.github_accounts, state.gitlab_accounts);
      setShowAccountModal(false);
      setSelectedAccountId(null);
      loadRepos();
    } else {
      setError(res.error || "Failed to switch account");
    }
  };

  const handleDeleteAccount = async (accountId: string) => {
    await apiLogout();
    removeAccount(accountId);
    const github = await getAccounts("github");
    const gitlab = await getAccounts("gitlab");
    setAccounts(github, gitlab);
    setShowAccountModal(false);
    setSelectedAccountId(null);

    // If deleted the current account, go to login
    const currentAccounts = provider === "github" ? github : gitlab;
    if (!currentAccounts.find(a => a.id !== accountId)) {
      handleLogout();
    }
  };

  const handleAddNewAccount = async () => {
    if (!newToken.trim()) {
      setError("Please enter a token");
      return;
    }

    const label = newLabel.trim() || `${provider} Account ${provider === "github" ? githubAccounts.length + 1 : gitlabAccounts.length + 1}`;
    const gitlabUrl = provider === "gitlab" ? newGitlabUrl : null;

    const res = await saveAccount(provider, newToken.trim(), gitlabUrl, label);
    if (res.success && res.data) {
      addAccount(res.data);
      const github = await getAccounts("github");
      const gitlab = await getAccounts("gitlab");
      setAccounts(github, gitlab);
      setShowAccountModal(false);
      setAddNewMode(false);
      setNewToken("");
      setNewLabel("");
      loadRepos();
    } else {
      setError(res.error || "Failed to add account");
    }
  };

  const handleCloseModal = () => {
    setShowAccountModal(false);
    setAddNewMode(false);
    setSelectedAccountId(null);
    setNewToken("");
    setNewLabel("");
  };

  const currentAccounts = provider === "github" ? githubAccounts : gitlabAccounts;

  return (
    <div className="page">
      <div className="header">
        <h1>Repositories</h1>
        <div className="header-actions">
          <select
              value={provider}
              onChange={(e) => handleSwitchProvider(e.target.value as Provider)}
              className="select-input"
          >
            <option value="github">GitHub</option>
            <option value="gitlab">GitLab</option>
          </select>
          <button onClick={() => setShowAccountModal(true)} className="btn-secondary">
            Switch Account
          </button>
          <button onClick={loadRepos} className="btn-secondary">Refresh</button>
          <button onClick={handleLogout} className="btn-secondary">Logout</button>
        </div>
      </div>

      {loading && <div className="loading">Loading repositories...</div>}
      {error && <div className="error-container">{error}</div>}

      {!loading && !error && (
        <div className="repo-list">
          {repos.map((repo) => (
            <div key={repo.id} className="repo-card" onClick={() => handleRepoClick(repo)}>
              <h3>{repo.name}</h3>
              <p className="repo-full-name">{repo.full_name}</p>
              {repo.description && <p className="repo-desc">{repo.description}</p>}
              <span className={`badge ${repo.private ? "private" : "public"}`}>
                {repo.private ? "Private" : "Public"}
              </span>
            </div>
          ))}
        </div>
      )}

      {/* Account Switch Modal */}
      {showAccountModal && (
        <div className="modal-overlay" onClick={handleCloseModal}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <h2>Switch Account</h2>
            <p className="modal-subtitle">Current: {provider}</p>

            {!addNewMode ? (
              <>
                <div className="account-list">
                  {currentAccounts.length > 0 ? (
                    currentAccounts.map((account) => (
                      <div
                        key={account.id}
                        className={`account-option ${selectedAccountId === account.id ? "selected" : ""}`}
                        onClick={() => setSelectedAccountId(account.id)}
                      >
                        <input
                          type="radio"
                          name="switchAccount"
                          checked={selectedAccountId === account.id}
                          onChange={() => setSelectedAccountId(account.id)}
                        />
                        <div className="account-info">
                          <span className="account-label">{account.label}</span>
                          {account.gitlab_url && (
                            <span className="account-url">{account.gitlab_url}</span>
                          )}
                        </div>
                        <button
                          className="btn-delete"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleDeleteAccount(account.id);
                          }}
                        >
                          Delete
                        </button>
                      </div>
                    ))
                  ) : (
                    <p className="no-accounts">No saved {provider} accounts</p>
                  )}
                </div>

                {currentAccounts.length > 0 && (
                  <button
                    onClick={handleSwitchAccount}
                    className="btn-primary"
                    disabled={!selectedAccountId}
                  >
                    Switch
                  </button>
                )}

                <button
                  onClick={() => {
                    setAddNewMode(true);
                    setSelectedAccountId(null);
                  }}
                  className="btn-secondary"
                >
                  Add New Account
                </button>
              </>
            ) : (
              <>
                {provider === "gitlab" && (
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
                    placeholder="e.g., Work Account"
                    className="token-input"
                  />
                </div>

                <div className="form-group">
                  <label>Token</label>
                  <input
                    type="password"
                    value={newToken}
                    onChange={(e) => setNewToken(e.target.value)}
                    placeholder={provider === "github" ? "ghp_xxx" : "glpat-xxx"}
                    className="token-input"
                  />
                </div>

                <div className="modal-actions">
                  <button onClick={() => setAddNewMode(false)} className="btn-secondary">
                    Back
                  </button>
                  <button onClick={handleAddNewAccount} className="btn-primary">
                    Add & Switch
                  </button>
                </div>
              </>
            )}

            <button className="btn-close" onClick={handleCloseModal}>
              Cancel
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

export default RepoListPage;
