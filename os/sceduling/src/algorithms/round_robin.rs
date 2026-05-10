use std::collections::VecDeque;

use crate::algorithms::{real_task::RealTask, scheduler::Scheduler};

struct RoundRobin{
    processes: VecDeque<RealTask>
}

impl Scheduler for RoundRobin {
    type Task = RealTask;
    
    fn get_runnable_tasks<'a>(&'a self) 
        -> impl Iterator<Item = &'a Self::Task> 
            where Self: 'a {
        self.processes.iter()
    }
    
    fn schedule(& mut self) -> Option<&Self::Task> {
        let mut task = self.processes.pop_front()?;
        
        Some(&mut task)
    }
    
    fn add_task(&mut self, t: Self::Task) {
        todo!()
    }
    
}