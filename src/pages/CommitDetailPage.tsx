import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { getCommitDetail } from "../api/tauri";
import { CommitDetail } from "../types";

function CommitDetailPage() {
  const { owner, repo, sha } = useParams<{ owner: string; repo: string; sha: string }>();
  const [detail, setDetail] = useState<CommitDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const navigate = useNavigate();

  useEffect(() => {
    if (owner && repo && sha) {
      loadDetail();
    }
  }, [owner, repo, sha]);

  const loadDetail = async () => {
    if (!owner || !repo || !sha) return;
    setLoading(true);
    setError("");
    const response = await getCommitDetail(owner, repo, sha);
    setLoading(false);
    if (response.success && response.data) {
      setDetail(response.data);
    } else {
      setError(response.error || "Failed to load commit details");
    }
  };

  const formatDate = (dateStr: string | null) => {
    if (!dateStr) return "Unknown";
    return new Date(dateStr).toLocaleString();
  };

  if (loading) return <div className="loading">Loading commit...</div>;
  if (error) return <div className="error-container">{error}</div>;
  if (!detail) return null;

  return (
    <div className="page">
      <button onClick={() => navigate(`/repo/${owner}/${repo}/commits`)} className="btn-back">
        ← Back to Commits
      </button>
      <div className="commit-detail">
        <h1>Commit {sha?.slice(0, 7)}</h1>
        <div className="commit-header">
          <p className="commit-message">{detail.commit.message}</p>
          <p className="commit-meta">
            {detail.commit.author.name || "Unknown"} authored on {formatDate(detail.commit.author.date)}
          </p>
        </div>

        {detail.files && detail.files.length > 0 && (
          <div className="files-section">
            <h2>Changed Files ({detail.files.length})</h2>
            {detail.files.map((file, index) => (
              <div key={index} className="file-card">
                <div className="file-header">
                  <span className="file-name">{file.filename || "Unknown"}</span>
                  <span className="file-stats">
                    {file.additions !== null && <span className="additions">+{file.additions}</span>}
                    {file.deletions !== null && <span className="deletions">-{file.deletions}</span>}
                  </span>
                </div>
                {file.patch && (
                  <pre className="diff-content">{file.patch}</pre>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default CommitDetailPage;
