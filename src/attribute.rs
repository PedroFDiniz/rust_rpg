pub struct Attribute {
    base: u16,
    growth: u16,
    flat_mod: u16,
    multiplier: u16,
}

impl Attribute {
    #![allow(unused)]
    pub fn new(base_attr: u16, growth_attr: u16) -> Self {
        return Attribute {
            base: base_attr,
            growth: growth_attr,
            flat_mod: 0,
            multiplier: 1,
        };
    }

    pub fn total(&self, level: u16) -> u16 {
        return
            (self.base +
            (self.growth * level) +
            self.flat_mod) *
            self.multiplier;
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            base: 10,
            growth: 1,
            flat_mod: 0,
            multiplier: 1,
        }
    }
}