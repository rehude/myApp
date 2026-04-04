import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { getRepoDetail } from "../api/tauri";
import { RepoDetail } from "../types";

function RepoDetailPage() {
  const { id } = useParams<{ id: string }>();
  const [detail, setDetail] = useState<RepoDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const navigate = useNavigate();

  useEffect(() => {
    if (id) {
      loadDetail();
    }
  }, [id]);

  const loadDetail = async () => {
    if (!id) return;
    setLoading(true);
    setError("");
    // id is URL-encoded full_name (owner/repo for GitHub, path_with_namespace for GitLab)
    const decodedId = decodeURIComponent(id);
    const response = await getRepoDetail(decodedId, "");
    setLoading(false);
    if (response.success && response.data) {
      setDetail(response.data);
    } else {
      setError(response.error || "Failed to load repository details");
    }
  };

  if (loading) return <div className="loading">Loading...</div>;
  if (error) return <div className="error-container">{error}</div>;
  if (!detail) return null;

  return (
    <div className="page">
      <button onClick={() => navigate("/repos")} className="btn-back">← Back</button>
      <div className="repo-detail">
        <h1>{detail.name}</h1>
        <p className="full-name">{detail.full_name}</p>
        {detail.description && <p className="description">{detail.description}</p>}
        <div className="stats">
          <span>⭐ {detail.stargazers_count}</span>
          <span>🍴 {detail.forks_count}</span>
          {detail.language && <span>📝 {detail.language}</span>}
        </div>
        <div className="actions">
          <button onClick={() => navigate(`/repo/${encodeURIComponent(detail.full_name)}/commits`)} className="btn-primary">
            View Commits
          </button>
          <a href={detail.html_url} target="_blank" rel="noopener noreferrer" className="btn-secondary">
            Open on GitHub
          </a>
        </div>
      </div>
    </div>
  );
}

export default RepoDetailPage;
