import { invoke } from "@tauri-apps/api/tauri";

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

export async function getRepos(): Promise<{ success: boolean; data?: any; error?: string }> {
  try {
    const data = await invoke("get_repos");
    return { success: true, data };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function getRepoDetail(owner: string, repo: string): Promise<{ success: boolean; data?: any; error?: string }> {
  try {
    const data = await invoke("get_repo_detail", { owner, repo });
    return { success: true, data };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function getCommits(owner: string, repo: string): Promise<{ success: boolean; data?: any; error?: string }> {
  try {
    const data = await invoke("get_commits", { owner, repo });
    return { success: true, data };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function getCommitDetail(owner: string, repo: string, sha: string): Promise<{ success: boolean; data?: any; error?: string }> {
  try {
    const data = await invoke("get_commit_detail", { owner, repo, sha });
    return { success: true, data };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}
