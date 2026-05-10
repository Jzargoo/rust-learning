use crate::algorithms::{predictable_task::PredictableTask, scheduler::Scheduler};

pub struct Srtf{
    storage: Vec<PredictableTask>
}

impl Scheduler for Srtf {
    type Task=PredictableTask;


    fn get_runnable_tasks<'a>(&'a self) -> impl Iterator<Item = &'a PredictableTask> where Self: 'a {
        self.storage.iter()
    }

    fn schedule(&mut self) -> Option<&mut Self::Task> {
        if self.storage.is_empty() { return None; }
        self.storage.sort_by_key(|k| k.get_remaining_time());
        self.storage.get_mut(0)
    }

    fn add_task(&mut self, t: Self::Task) {
        self.storage.push(t);
    }
}