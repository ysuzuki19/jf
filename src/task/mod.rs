mod context;
mod mode;
mod runner;
mod taskdef;
mod taskdefs;

pub use context::Context;
pub use runner::Runner;
pub use taskdef::Taskdef;
pub use taskdefs::Taskdefs;

#[derive(Clone)]
pub(super) enum Agent {
    Cli,
    Task,
}
