import { create } from "zustand";
import { Repo } from "../types";

export type Provider = "github" | "gitlab";

export interface AccountInfo {
  id: string;
  label: string;
  gitlab_url?: string;
}

interface AppState {
  provider: Provider;
  isLoggedIn: boolean;
  gitlabUrl: string;
  repos: Repo[];
  selectedRepoFullName: string | null;
  selectedRepoId: number | null;
  // Account management
  savedAccounts: AccountInfo[];
  githubAccounts: AccountInfo[];
  gitlabAccounts: AccountInfo[];
  setProvider: (provider: Provider) => void;
  setGitlabUrl: (url: string) => void;
  setRepos: (repos: Repo[]) => void;
  setSelectedRepo: (repo: Repo | null) => void;
  clearSelectedRepo: () => void;
  // Account actions
  setAccounts: (github: AccountInfo[], gitlab: AccountInfo[]) => void;
  setLoggedIn: (loggedIn: boolean) => void;
  removeAccount: (id: string) => void;
  addAccount: (account: AccountInfo) => void;
}

export const useAppStore = create<AppState>((set) => ({
  provider: "github",
  isLoggedIn: false,
  gitlabUrl: "https://gitlab.com",
  repos: [],
  selectedRepoFullName: null,
  selectedRepoId: null,
  savedAccounts: [],
  githubAccounts: [],
  gitlabAccounts: [],
  setProvider: (provider) => set({ provider }),
  setGitlabUrl: (url) => set({ gitlabUrl: url }),
  setRepos: (repos) => set({ repos }),
  setSelectedRepo: (repo) => repo ? set({
    selectedRepoFullName: repo.full_name,
    selectedRepoId: repo.id
  }) : set({ selectedRepoFullName: null, selectedRepoId: null }),
  clearSelectedRepo: () => set({ selectedRepoFullName: null, selectedRepoId: null }),
  setAccounts: (github, gitlab) => set({
    githubAccounts: github,
    gitlabAccounts: gitlab,
    savedAccounts: [...github, ...gitlab],
  }),
  setLoggedIn: (loggedIn) => set({ isLoggedIn: loggedIn }),
  removeAccount: (id) => set((state) => ({
    githubAccounts: state.githubAccounts.filter(a => a.id !== id),
    gitlabAccounts: state.gitlabAccounts.filter(a => a.id !== id),
    savedAccounts: state.savedAccounts.filter(a => a.id !== id),
  })),
  addAccount: (account) => set((state) => ({
    savedAccounts: [...state.savedAccounts, account],
    githubAccounts: account.gitlab_url === undefined
      ? [...state.githubAccounts, account]
      : state.githubAccounts,
    gitlabAccounts: account.gitlab_url !== undefined
      ? [...state.gitlabAccounts, account]
      : state.gitlabAccounts,
  })),
}));
