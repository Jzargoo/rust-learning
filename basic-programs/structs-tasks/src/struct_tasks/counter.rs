pub struct Counter {
    value : i32, 
    step: i32
}

impl Counter {
    pub fn new () -> Self {
        Self {
            value: 0,
            step: 1
        }
    }

    pub fn increment(&mut self) {
        self.value += self.step;
    }
    pub fn decrement(&mut self) {
        self.value -= self.step;
    }
    pub fn reset(&mut self) {
        self.value = 0;
    }
    pub fn get(&self) -> i32 {
        self.value
    }

}