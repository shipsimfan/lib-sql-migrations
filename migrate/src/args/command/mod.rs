mod applied;
mod apply;
mod apply_down;
mod apply_up;
mod available;
mod required;
mod required_down;
mod required_up;

pub(super) use applied::APPLIED;
pub(super) use apply::APPLY;
pub(super) use apply_down::APPLY_DOWN;
pub(super) use apply_up::APPLY_UP;
pub(super) use available::AVAILABLE;
pub(super) use required::REQUIRED;
pub(super) use required_down::REQUIRED_DOWN;
pub(super) use required_up::REQUIRED_UP;

/// The selected command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// List the available migrations
    Available,

    /// List the applied migrations
    Applied,

    /// List the required migrations
    Required,

    /// List the required up migrations
    RequiredUp,

    /// List the required down migrations
    RequiredDown,

    /// Apply all required migrations
    Apply,

    /// Apply all required up migrations
    ApplyUp,

    /// Apply all required down migrations
    ApplyDown,
}
