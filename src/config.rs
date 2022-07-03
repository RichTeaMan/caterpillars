use build_time::build_time_local;
use git_version::git_version;

/// Date (Y-m-d) of when the program was compiled.
pub const BUILD_DATE: &str = build_time_local!("%Y-%m-%d");

/// Git version of the build.
pub const GIT_VERSION: &str = git_version!();

#[cfg(target_arch = "wasm32")]
pub const START_RESOLUTION_WIDTH: f32 = 100.0;

#[cfg(target_arch = "wasm32")]
pub const START_RESOLUTION_HEIGHT: f32 = 100.0;

#[cfg(not(target_arch = "wasm32"))]
pub const START_RESOLUTION_WIDTH: f32 = 1280.0;

#[cfg(not(target_arch = "wasm32"))]
pub const START_RESOLUTION_HEIGHT: f32 = 720.0;
