
use crate::algorithms::task::Task;

pub struct RealTask {
    id: u32,
    pub rem_time: u32, //time for mlfq in ms 
    pub priority: i8,
    pub p_pcb: *const usize
}

impl Task for RealTask {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl RealTask {
    fn new() -> Self {
        RealTask {
            id: 1, 
            rem_time: 0,
            priority: 1,
            p_pcb: 42 as *const usize,
        }
    }

    pub fn set_rem_time(&mut self, rem_time: u32) {
        self.rem_time = rem_time
    }

    pub fn promote(&mut self) {
        if self.priority > 0{
            self.priority -= 1;
        }
    }

    pub fn demote(&mut self, highest: i8) {
        if self.priority < highest {
            self.priority += 1;
        }
    }
}