import { create } from "zustand";
import { Repo } from "../types";

interface AppState {
  token: string | null;
  repos: Repo[];
  selectedRepo: Repo | null;
  setToken: (token: string) => void;
  clearToken: () => void;
  setRepos: (repos: Repo[]) => void;
  setSelectedRepo: (repo: Repo | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  token: null,
  repos: [],
  selectedRepo: null,
  setToken: (token) => set({ token }),
  clearToken: () => set({ token: null }),
  setRepos: (repos) => set({ repos }),
  setSelectedRepo: (repo) => set({ selectedRepo: repo }),
}));
