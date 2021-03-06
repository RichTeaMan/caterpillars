use build_time::build_time_local;
use git_version::git_version;

/// Date (Y-m-d) of when the program was compiled.
pub const BUILD_DATE: &str = build_time_local!("%Y-%m-%d");

/// Git version of the build.
pub const GIT_VERSION: &str = git_version!();

pub const PLANE_SIZE: f32 = 5000.0;

/** Starting caterpillars.  */
pub const STARTING_CATERPILLARS: i32 = 20;

pub const STARTING_CATERPILLAR_RADIUS: f32 = 100.0;

pub const STARTING_BUSHES: i32 = 20000;

pub const STARTING_TREES: i32 = 200;

pub const TREE_HEIGHT: f32 = 100.0;

/** Minimum length of a caterpillar. */
pub const CATERPILLAR_MIN_LENGTH: i32 = 3;

/** Maximum length of a caterpillar. */
pub const CATERPILLAR_MAX_LENGTH: i32 = 20;

/** Minimum speed of a caterpillar. */
pub const CATERPILLAR_MIN_SPEED: f32 = 0.5;

/** Maximum speed of a caterpillar. */
pub const CATERPILLAR_MAX_SPEED: f32 = 50.0;

#[cfg(target_arch = "wasm32")]
pub const ENABLE_SHADOWS: bool = false;

#[cfg(target_arch = "wasm32")]
pub const START_RESOLUTION_WIDTH: f32 = 100.0;

#[cfg(target_arch = "wasm32")]
pub const START_RESOLUTION_HEIGHT: f32 = 100.0;

#[cfg(not(target_arch = "wasm32"))]
pub const ENABLE_SHADOWS: bool = true;

#[cfg(not(target_arch = "wasm32"))]
pub const START_RESOLUTION_WIDTH: f32 = 1280.0;

#[cfg(not(target_arch = "wasm32"))]
pub const START_RESOLUTION_HEIGHT: f32 = 720.0;
