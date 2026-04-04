import { create } from "zustand";
import { Repo } from "../types";

export type Provider = "github" | "gitlab";

interface AppState {
  provider: Provider;
  githubToken: string | null;
  gitlabToken: string | null;
  gitlabUrl: string;
  repos: Repo[];
  selectedRepo: Repo | null;
  setProvider: (provider: Provider) => void;
  setGithubToken: (token: string) => void;
  setGitlabToken: (token: string) => void;
  setGitlabUrl: (url: string) => void;
  clearTokens: () => void;
  setRepos: (repos: Repo[]) => void;
  setSelectedRepo: (repo: Repo | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  provider: "github",
  githubToken: null,
  gitlabToken: null,
  gitlabUrl: "https://gitlab.com",
  repos: [],
  selectedRepo: null,
  setProvider: (provider) => set({ provider }),
  setGithubToken: (token) => set({ githubToken: token }),
  setGitlabToken: (token) => set({ gitlabToken: token }),
  setGitlabUrl: (url) => set({ gitlabUrl: url }),
  clearTokens: () => set({ githubToken: null, gitlabToken: null }),
  setRepos: (repos) => set({ repos }),
  setSelectedRepo: (repo) => set({ selectedRepo: repo }),
}));
