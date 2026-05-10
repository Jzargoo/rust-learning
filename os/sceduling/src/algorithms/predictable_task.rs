use crate::algorithms::task::Task;

pub struct PredictableTask{
    id: u32,
    remaining_time: u32 //in ms
}

impl Task for PredictableTask{
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl PredictableTask {
    pub fn new(nid: u32, rmt:u32 ) -> Self{
        PredictableTask{
            id:nid,
            remaining_time: rmt
        }
    }
    pub fn get_remaining_time(&self) -> u32 {
           self.remaining_time
    }

    pub fn update_remaining_time(&mut self,time: u32) {
        self.remaining_time -= time;
    }
}
