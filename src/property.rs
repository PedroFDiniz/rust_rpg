use std::fmt::{Display, Formatter, Result};

pub struct Property {
    actual: u32,
    maximum: u32
}

impl Property {
    #![allow(unused)]
    pub fn new(max:u32) -> Self {
        return Property {
            actual: max,
            maximum: max
        };
    }
    pub fn get(&self) -> u32 {
        return self.actual;
    }

    pub fn get_max(&self) -> u32 {
        return self.maximum;
    }

    pub fn add(&mut self, amount:u32) {
        if self.actual + amount > self.maximum { self.actual = self.maximum; }
        else { self.actual += amount; }
    }

    pub fn subtract(&mut self, amount:u32) {
        if self.actual < amount { self.actual = 0; }
        else { self.actual -= amount; }
    }

    pub fn set(&mut self, value: u32) {
        self.actual = if value <= self.maximum { value } else { self.maximum };
    }

    pub fn set_max(&mut self, value: u32) {
        self.maximum = if value < 1 { 1 } else { value };
        self.update();
    }

    pub fn fill(&mut self) {
        self.actual = self.maximum;
    }

    pub fn is_empty(&self) -> bool {
        return self.actual == 0;
    }

    pub fn is_full(&self) -> bool {
        return self.actual == self.maximum;
    }

    fn update(&mut self) {
        if self.actual > self.maximum { self.actual = self.maximum; }
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}/{}]", self.get(), self.get_max())
    }
}

impl Default for Property {
    fn default() -> Self {
        Self {
            actual: 100,
            maximum: 100,
        }
    }
}