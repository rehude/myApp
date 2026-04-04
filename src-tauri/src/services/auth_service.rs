use crate::state::AppState;

pub fn set_token(state: &AppState, token: String) {
    let mut guard = state.github_token.lock().unwrap();
    *guard = Some(token);
}

pub fn get_token(state: &AppState) -> Option<String> {
    let guard = state.github_token.lock().unwrap();
    guard.clone()
}
