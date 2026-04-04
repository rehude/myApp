import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { useEffect, useState } from "react";
import LoginPage from "./pages/LoginPage";
import RepoListPage from "./pages/RepoListPage";
import CommitListPage from "./pages/CommitListPage";
import CommitDetailPage from "./pages/CommitDetailPage";
import { useAppStore } from "./store/useAppStore";
import { initApp } from "./api/tauri";

function App() {
  const [loading, setLoading] = useState(true);
  const { provider, githubToken, gitlabToken, setProvider, setGithubToken, setGitlabToken } = useAppStore();

  useEffect(() => {
    const checkAuth = async () => {
      try {
        const res = await initApp();
        if (res.logged_in) {
          setProvider(res.provider as "github" | "gitlab");
          if (res.github_token_exists) {
            setGithubToken("stored");
          }
          if (res.gitlab_token_exists) {
            setGitlabToken("stored");
          }
        }
      } catch (e) {
        console.error("Failed to init app:", e);
      } finally {
        setLoading(false);
      }
    };
    checkAuth();
  }, []);

  if (loading) {
    return <div className="loading">Loading...</div>;
  }

  const isLoggedIn = provider === "github" ? githubToken !== null : gitlabToken !== null;

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={isLoggedIn ? <Navigate to="/repos" /> : <LoginPage />} />
        <Route path="/repos" element={isLoggedIn ? <RepoListPage /> : <Navigate to="/" />} />
        <Route path="/repo/:id/commits" element={isLoggedIn ? <CommitListPage /> : <Navigate to="/" />} />
        <Route path="/commit/:sha" element={isLoggedIn ? <CommitDetailPage /> : <Navigate to="/" />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
