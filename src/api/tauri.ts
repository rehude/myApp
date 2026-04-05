import { invoke } from "@tauri-apps/api/tauri";

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  code?: string;
}

export interface AccountInfo {
  id: string;
  label: string;
  gitlab_url?: string;
}

export interface InitAppResponse {
  logged_in: boolean;
  provider: string;
  current_token_exists: boolean;
  has_saved_accounts: boolean;
  github_accounts: AccountInfo[];
  gitlab_accounts: AccountInfo[];
}

export async function initApp(): Promise<InitAppResponse> {
  return await invoke("init_app");
}

export async function getCurrentState(): Promise<InitAppResponse> {
  return await invoke("get_current_state");
}

export async function logout(): Promise<{ success: boolean; error?: string }> {
  try {
    await invoke("logout");
    return { success: true };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function saveAccount(
  provider: string,
  token: string,
  gitlabUrl: string | null,
  label: string
): Promise<ApiResponse<AccountInfo>> {
  return await invoke("save_account", { provider, token, gitlabUrl, label });
}

export async function getAccounts(provider: string): Promise<AccountInfo[]> {
  return await invoke("get_accounts", { provider });
}

export async function deleteAccount(accountId: string): Promise<void> {
  return await invoke("delete_account", { accountId });
}

export async function setCurrentAccount(accountId: string): Promise<ApiResponse<AccountInfo>> {
  return await invoke("set_current_account", { accountId });
}

export async function setProvider(provider: string): Promise<{ success: boolean; error?: string }> {
  try {
    await invoke("set_provider", { provider });
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
