use crate::algorithms::task::Task;

// There processes will be returned rather than be switched
// there will be returned only clones of the processes rather than shift them at all 
pub trait Scheduler {
    type Task: Task;

    fn get_runnable_tasks<'a>(&'a self) 
        -> impl Iterator<Item = &'a Self::Task> 
            where Self: 'a;

    fn schedule(&mut self) -> Option<&Self::Task>;
    fn add_task(&mut self, t: Self::Task);

}

