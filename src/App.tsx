import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { useEffect, useState } from "react";
import LoginPage from "./pages/LoginPage";
import RepoListPage from "./pages/RepoListPage";
import CommitListPage from "./pages/CommitListPage";
import CommitDetailPage from "./pages/CommitDetailPage";
import { useAppStore } from "./store/useAppStore";
import { initApp } from "./api/tauri";

function AppContent() {
  const [loading, setLoading] = useState(true);
  const { isLoggedIn, setProvider, setAccounts, setLoggedIn } = useAppStore();

  useEffect(() => {
    const checkAuth = async () => {
      try {
        const res = await initApp();
        setAccounts(res.github_accounts, res.gitlab_accounts);

        if (res.logged_in) {
          setProvider(res.provider as "github" | "gitlab");
          setLoggedIn(true);
        } else {
          setLoggedIn(false);
        }
      } catch (e) {
        console.error("Failed to init app:", e);
        setLoggedIn(false);
      } finally {
        setLoading(false);
      }
    };
    checkAuth();
  }, []);  // 只在首次挂载时执行

  if (loading) {
    return <div className="loading">Loading...</div>;
  }

  return (
    <Routes>
      <Route path="/" element={isLoggedIn ? <Navigate to="/repos" /> : <LoginPage />} />
      <Route path="/repos" element={isLoggedIn ? <RepoListPage /> : <Navigate to="/" />} />
      <Route path="/repo/:id/commits" element={isLoggedIn ? <CommitListPage /> : <Navigate to="/" />} />
      <Route path="/commit/:sha" element={isLoggedIn ? <CommitDetailPage /> : <Navigate to="/" />} />
    </Routes>
  );
}

function App() {
  return (
    <BrowserRouter>
      <AppContent />
    </BrowserRouter>
  );
}

export default App;
