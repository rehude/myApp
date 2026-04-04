import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { getRepos } from "../api/tauri";
import { useAppStore } from "../store/useAppStore";
import { Repo } from "../types";

function RepoListPage() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const navigate = useNavigate();
  const { repos, setRepos, setSelectedRepo } = useAppStore();

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
      setError(response.error || "Failed to load repositories");
    }
  };

  const handleRepoClick = (repo: Repo) => {
    setSelectedRepo(repo);
    navigate(`/repo/${repo.owner.login}/${repo.name}`);
  };

  if (loading) return <div className="loading">Loading repositories...</div>;
  if (error) return <div className="error-container">{error}</div>;

  return (
    <div className="page">
      <div className="header">
        <h1>Repositories</h1>
        <button onClick={loadRepos} className="btn-secondary">Refresh</button>
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
