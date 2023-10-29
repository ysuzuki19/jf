use super::taskdefs::Taskdefs;

#[derive(Clone)]
pub struct Context {
    task_defs: Taskdefs,
}

impl Context {
    pub fn new(task_defs: Taskdefs) -> Self {
        Self { task_defs }
    }
}

impl Context {
    pub fn build(&self, task_name: String) -> crate::error::CmdResult<crate::task::Task> {
        self.task_defs.build(task_name, crate::task::Agent::Task)
    }
}
