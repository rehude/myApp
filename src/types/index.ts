export interface Repo {
  id: number;
  name: string;
  full_name: string;
  private: boolean;
  html_url: string;
  description: string | null;
  owner: Owner;
}

export interface Owner {
  login: string;
  avatar_url: string | null;
}

export interface RepoDetail {
  id: number;
  name: string;
  full_name: string;
  private: boolean;
  html_url: string;
  description: string | null;
  stargazers_count: number;
  forks_count: number;
  language: string | null;
  owner: Owner;
}

export interface CommitSummary {
  sha: string;
  commit: CommitInfo;
  html_url: string;
}

export interface CommitInfo {
  message: string;
  author: CommitAuthor;
}

export interface CommitAuthor {
  name: string | null;
  email: string | null;
  date: string | null;
}

export interface CommitDetail {
  sha: string;
  commit: CommitInfo;
  html_url: string;
  files: CommitFile[] | null;
}

export interface CommitFile {
  filename: string | null;
  status: string | null;
  patch: string | null;
  additions: number | null;
  deletions: number | null;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
