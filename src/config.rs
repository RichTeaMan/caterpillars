use git_version::git_version;

pub const GIT_VERSION: &str = git_version!();

pub const PLANE_SIZE: f32 = 5000.0;

/** Starting caterpillars.  */
pub const STARTING_CATERPILLARS: i32 = 20;

pub const STARTING_CATERPILLAR_RADIUS: f32 = 100.0;

pub const STARTING_BUSHES: i32 = 200;

pub const STARTING_TREES: i32 = 200;

/** Minimum length of a caterpillar. */
pub const CATERPILLAR_MIN_LENGTH: i32 = 3;

/** Maximum length of a caterpillar. */
pub const CATERPILLAR_MAX_LENGTH: i32 = 20;

/** Minimum speed of a caterpillar. */
pub const CATERPILLAR_MIN_SPEED: f32 = 0.5;

/** Maximum speed of a caterpillar. */
pub const CATERPILLAR_MAX_SPEED: f32 = 50.0;
