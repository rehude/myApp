  import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import LoginPage from "./pages/LoginPage";
import RepoListPage from "./pages/RepoListPage";
import RepoDetailPage from "./pages/RepoDetailPage";
import CommitListPage from "./pages/CommitListPage";
import CommitDetailPage from "./pages/CommitDetailPage";
import { useAppStore } from "./store/useAppStore";

function App() {
  const token = useAppStore((state) => state.token);

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={token ? <Navigate to="/repos" /> : <LoginPage />} />
        <Route path="/repos" element={token ? <RepoListPage /> : <Navigate to="/" />} />
        <Route path="/repo/:owner/:repo" element={token ? <RepoDetailPage /> : <Navigate to="/" />} />
        <Route path="/repo/:owner/:repo/commits" element={token ? <CommitListPage /> : <Navigate to="/" />} />
        <Route path="/commit/:owner/:repo/:sha" element={token ? <CommitDetailPage /> : <Navigate to="/" />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
