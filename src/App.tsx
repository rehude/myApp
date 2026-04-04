import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import LoginPage from "./pages/LoginPage";
import RepoListPage from "./pages/RepoListPage";
import RepoDetailPage from "./pages/RepoDetailPage";
import CommitListPage from "./pages/CommitListPage";
import CommitDetailPage from "./pages/CommitDetailPage";
import { useAppStore } from "./store/useAppStore";

function App() {
  const { provider, githubToken, gitlabToken } = useAppStore();
  const isLoggedIn = provider === "github" ? githubToken !== null : gitlabToken !== null;

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={isLoggedIn ? <Navigate to="/repos" /> : <LoginPage />} />
        <Route path="/repos" element={isLoggedIn ? <RepoListPage /> : <Navigate to="/" />} />
        <Route path="/repo/:id" element={isLoggedIn ? <RepoDetailPage /> : <Navigate to="/" />} />
        <Route path="/repo/:id/commits" element={isLoggedIn ? <CommitListPage /> : <Navigate to="/" />} />
        <Route path="/commit/:id/:sha" element={isLoggedIn ? <CommitDetailPage /> : <Navigate to="/" />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
