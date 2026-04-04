import { create } from "zustand";
import { Repo } from "../types";

export type Provider = "github" | "gitlab";

interface AppState {
  provider: Provider;
  githubToken: string | null;
  gitlabToken: string | null;
  gitlabUrl: string;
  repos: Repo[];
  selectedRepoFullName: string | null;  // owner/repo for GitHub, path for GitLab
  selectedRepoId: number | null;        // numeric ID for both
  setProvider: (provider: Provider) => void;
  setGithubToken: (token: string) => void;
  setGitlabToken: (token: string) => void;
  setGitlabUrl: (url: string) => void;
  clearTokens: () => void;
  setRepos: (repos: Repo[]) => void;
  setSelectedRepo: (repo: Repo | null) => void;
  clearSelectedRepo: () => void;
}

export const useAppStore = create<AppState>((set) => ({
  provider: "github",
  githubToken: null,
  gitlabToken: null,
  gitlabUrl: "https://gitlab.com",
  repos: [],
  selectedRepoFullName: null,
  selectedRepoId: null,
  setProvider: (provider) => set({ provider }),
  setGithubToken: (token) => set({ githubToken: token }),
  setGitlabToken: (token) => set({ gitlabToken: token }),
  setGitlabUrl: (url) => set({ gitlabUrl: url }),
  clearTokens: () => set({ githubToken: null, gitlabToken: null }),
  setRepos: (repos) => set({ repos }),
  setSelectedRepo: (repo) => repo ? set({
    selectedRepoFullName: repo.full_name,
    selectedRepoId: repo.id
  }) : set({ selectedRepoFullName: null, selectedRepoId: null }),
  clearSelectedRepo: () => set({ selectedRepoFullName: null, selectedRepoId: null }),
}));
