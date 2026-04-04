import { invoke } from "@tauri-apps/api/tauri";

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  code?: string;
}

export interface InitAppResponse {
  logged_in: boolean;
  provider: string;
  github_token_exists: boolean;
  gitlab_token_exists: boolean;
}

export async function initApp(): Promise<InitAppResponse> {
  return await invoke("init_app");
}

export async function logout(): Promise<{ success: boolean; error?: string }> {
  try {
    await invoke("logout");
    return { success: true };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function setProvider(provider: string): Promise<{ success: boolean; error?: string }> {
  try {
    await invoke("set_provider", { provider });
    return { success: true };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function setGithubToken(token: string): Promise<{ success: boolean; error?: string }> {
  try {
    await invoke("set_github_token", { token });
    return { success: true };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function setGitlabToken(token: string): Promise<{ success: boolean; error?: string }> {
  try {
    await invoke("set_gitlab_token", { token });
    return { success: true };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function setGitlabUrl(url: string): Promise<{ success: boolean; error?: string }> {
  try {
    await invoke("set_gitlab_url", { url });
    return { success: true };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function getRepos(): Promise<ApiResponse<any>> {
  return await invoke("get_repos");
}

export async function getRepoDetail(owner: string, repo: string): Promise<ApiResponse<any>> {
  return await invoke("get_repo_detail", { owner, repo });
}

export async function getCommits(owner: string, repo: string): Promise<ApiResponse<any>> {
  return await invoke("get_commits", { owner, repo });
}

export async function getCommitDetail(owner: string, repo: string, sha: string): Promise<ApiResponse<any>> {
  return await invoke("get_commit_detail", { owner, repo, sha });
}
