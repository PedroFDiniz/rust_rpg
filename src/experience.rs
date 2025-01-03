pub struct Experience {
    actual: u64,
    maximum: u64,
}

impl Experience {
    pub fn new(maximum_xp: u64) -> Self {
        return Self {
            actual: 0,
            maximum: maximum_xp,
        }
    }

    pub fn get(&self) -> u64 {
        return self.actual;
    }

    pub fn get_max(&self) -> u64 {
        return self.maximum;
    }

    pub fn add(&mut self, amount: u64) {
        if self.actual + amount > self.maximum { self.actual = self.maximum; }
        else { self.actual += amount; }
    }

    pub fn subtract(&mut self, amount: u64) {
        if self.actual < amount { self.actual = 0; }
        else { self.actual -= amount; }
    }

    pub fn set(&mut self, value: u64) {
        if value < self.maximum { self.actual = value; }
        else { self.actual = self.maximum; }
    }

    pub fn set_max(&mut self, value: u64) {
        if value < 1 { self.maximum = 1; }
        else { self.maximum = value; }
        self.update();
    }

    fn update(&mut self) {
        if self.actual > self.maximum { self.actual = self.maximum };
    }

}

impl Default for Experience {
    fn default() -> Self {
        Self {
            actual: 0,
            maximum: 10000,
        }
    }
}