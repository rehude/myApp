import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { getRepos, logout as apiLogout, setProvider as apiSetProvider } from "../api/tauri";
import { useAppStore, Provider } from "../store/useAppStore";
import { Repo } from "../types";

function RepoListPage() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const navigate = useNavigate();
  const { repos, setRepos, setSelectedRepo, clearTokens, setProvider, provider } = useAppStore();

  useEffect(() => {
    loadRepos();
  }, []);

  const loadRepos = async () => {
    setLoading(true);
    setError("");
    const response = await getRepos();
    setLoading(false);
    if (response.success && response.data) {
      setRepos(response.data);
    } else {
      if (response.code === "UNAUTHORIZED" || response.code === "NOT_LOGGED_IN") {
        clearTokens();
        navigate("/");
      } else {
        setError(response.error || "Failed to load repositories");
      }
    }
  };

  const handleRepoClick = (repo: Repo) => {
    setSelectedRepo(repo);
    navigate(`/repo/${repo.id}/commits`);
  };

  const handleLogout = async () => {
    await apiLogout();
    clearTokens();
    navigate("/");
  };

  const handleSwitchProvider = async (newProvider: Provider) => {
    const res = await apiSetProvider(newProvider);
    if (res.success) {
      setProvider(newProvider);
      loadRepos();
    }
  };

  if (loading) return <div className="loading">Loading repositories...</div>;
  if (error) return <div className="error-container">{error}</div>;

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
          <button onClick={loadRepos} className="btn-secondary">Refresh</button>
          <button onClick={handleLogout} className="btn-secondary">Logout</button>
        </div>
      </div>
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
    </div>
  );
}

export default RepoListPage;
