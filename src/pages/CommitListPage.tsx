import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { getCommits, getRepoDetail } from "../api/tauri";
import { useAppStore } from "../store/useAppStore";
import { CommitSummary, RepoDetail } from "../types";

function CommitListPage() {
  const [commits, setCommits] = useState<CommitSummary[]>([]);
  const [repoDetail, setRepoDetail] = useState<RepoDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const navigate = useNavigate();
  const { provider, selectedRepoFullName, selectedRepoId, clearSelectedRepo } = useAppStore();

  useEffect(() => {
    loadData();
    return () => {
      // Clear selected repo when leaving
    };
  }, []);

  const loadData = async () => {
    if (!selectedRepoFullName && !selectedRepoId) {
      setError("No repository selected");
      setLoading(false);
      return;
    }

    setLoading(true);
    setError("");

    try {
      // Load repo detail
      let repoResponse;
      if (provider === "github") {
        repoResponse = await getRepoDetail(selectedRepoFullName!, "");
      } else {
        // For GitLab, use the numeric ID
        repoResponse = await getRepoDetail("", String(selectedRepoId));
      }

      if (repoResponse.success && repoResponse.data) {
        setRepoDetail(repoResponse.data);
      } else {
        setError(repoResponse.error || "Failed to load repository");
        setLoading(false);
        return;
      }

      // Load commits
      let commitsResponse;
      if (provider === "github") {
        commitsResponse = await getCommits(selectedRepoFullName!, "");
      } else {
        commitsResponse = await getCommits("", String(selectedRepoId));
      }

      setLoading(false);
      if (commitsResponse.success && commitsResponse.data) {
        setCommits(commitsResponse.data);
      } else {
        setError(commitsResponse.error || "Failed to load commits");
      }
    } catch (e) {
      setLoading(false);
      setError(String(e));
    }
  };

  const formatDate = (dateStr: string | null) => {
    if (!dateStr) return "Unknown";
    return new Date(dateStr).toLocaleDateString();
  };

  const handleBack = () => {
    clearSelectedRepo();
    navigate("/repos");
  };

  if (loading) return <div className="loading">Loading...</div>;
  if (error) return <div className="error-container">{error}</div>;

  return (
    <div className="page">
      <button onClick={handleBack} className="btn-back">← Back</button>
      <h1>Commits{repoDetail ? `: ${repoDetail.name}` : ""}</h1>
      <div className="commit-list">
        {commits.map((commit) => (
          <div
            key={commit.sha}
            className="commit-card"
            onClick={() => navigate(`/commit/${commit.sha}`)}
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
