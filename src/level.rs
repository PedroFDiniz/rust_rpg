pub struct Level {
    actual: u16,
    maximum: u16,
}

impl Level {
    pub fn init(max: u16) -> Self {
        return Self {
            actual: 1,
            maximum: max,
        }
    }
    pub fn get(&self) -> u16 {
        return self.actual;
    }

    pub fn get_max(&self) -> u16 {
        return self.maximum;
    }

    pub fn set(&mut self, value: u16) {
        self.actual = if value <= self.maximum { value } else { self.maximum };
    }

    pub fn set_max(&mut self, value: u16) {
        self.maximum = if value == 0 { 1 } else { value };
    }

    fn update(&mut self) {
        if self.actual > self.maximum { self.actual = self.maximum; }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self {
            actual: 1,
            maximum: 50
        }
    }
}