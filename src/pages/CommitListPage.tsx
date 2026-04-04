import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { getCommits } from "../api/tauri";
import { CommitSummary } from "../types";

function CommitListPage() {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const [commits, setCommits] = useState<CommitSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const navigate = useNavigate();

  useEffect(() => {
    if (owner && repo) {
      loadCommits();
    }
  }, [owner, repo]);

  const loadCommits = async () => {
    if (!owner || !repo) return;
    setLoading(true);
    setError("");
    const response = await getCommits(owner, repo);
    setLoading(false);
    if (response.success && response.data) {
      setCommits(response.data);
    } else {
      setError(response.error || "Failed to load commits");
    }
  };

  const formatDate = (dateStr: string | null) => {
    if (!dateStr) return "Unknown";
    return new Date(dateStr).toLocaleDateString();
  };

  if (loading) return <div className="loading">Loading commits...</div>;
  if (error) return <div className="error-container">{error}</div>;

  return (
    <div className="page">
      <button onClick={() => navigate(`/repo/${owner}/${repo}`)} className="btn-back">← Back</button>
      <h1>Commits - {repo}</h1>
      <div className="commit-list">
        {commits.map((commit) => (
          <div
            key={commit.sha}
            className="commit-card"
            onClick={() => navigate(`/commit/${owner}/${repo}/${commit.sha}`)}
          >
            <div className="commit-sha">{commit.sha.slice(0, 7)}</div>
            <div className="commit-info">
              <p className="commit-message">{commit.commit.message.split('\n')[0]}</p>
              <p className="commit-meta">
                {commit.commit.author.name || "Unknown"} • {formatDate(commit.commit.author.date)}
              </p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default CommitListPage;
