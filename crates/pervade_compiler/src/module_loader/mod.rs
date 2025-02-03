mod module_loader;
mod module_task;
mod task_result;

pub use module_loader::ModuleLoader;

use task_result::TaskResult;

pub enum Msg {
    Done(TaskResult),
}
