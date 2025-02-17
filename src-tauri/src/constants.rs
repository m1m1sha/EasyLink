
pub const VERSION: &str = git_version::git_version!(
    args = ["--abbrev=8", "--always", "--dirty=~"],
    prefix = concat!(env!("CARGO_PKG_VERSION"), "-"),
    suffix = "",
    fallback = env!("CARGO_PKG_VERSION")
);

#[allow(dead_code)]
pub const AUTOSTART: &str = "--autostart";

pub const TASK_AUTOSTART: &str = "--task-autostart";
pub const TASK_AUTOSTART_FOLDER: &str = "easylink";
pub const TASK_AUTOSTART_NAME: &str = "auto start";