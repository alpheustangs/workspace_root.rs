use async_std::{io, path::PathBuf};

use get_dir::{FileTarget, GetDir, Target, async_std::GetDirAsyncExt as _};

/// Get the workspace root directory by searching for the `Cargo.lock` file.
///
/// Use [`get_workspace_root_async`] to handle the error automatically.
///
/// ```no_run
/// use async_std::path::PathBuf;
///
/// use workspace_root::async_std::get_workspace_root_directory_async;
///
/// # async fn example() {
/// let root: PathBuf = get_workspace_root_directory_async().await.unwrap();
/// # }
/// ```
pub async fn get_workspace_root_directory_async() -> io::Result<PathBuf> {
    GetDir::new()
        .targets(vec![Target::File(FileTarget { name: "Cargo.lock" })])
        .run_reverse_async()
        .await
}

/// Get the workspace root directory by searching for the `Cargo.lock` file.
///
/// Use [`get_workspace_root_directory_async`] to handle the error manually.
///
/// ```no_run
/// use async_std::path::PathBuf;
///
/// use workspace_root::async_std::get_workspace_root_async;
///
/// # async fn example() {
/// let root: PathBuf = get_workspace_root_async().await;
/// # }
/// ```
pub async fn get_workspace_root_async() -> PathBuf {
    get_workspace_root_directory_async()
        .await
        .expect("Failed to get workspace root")
}
