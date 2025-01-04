use std::collections::HashMap;

pub struct Statistic {
    base: i64,
    modifiers: HashMap<String, i64>,
    total: i64,
}

impl Statistic {
    pub fn get_base(&self) -> i64 { self.base }
    pub fn get_total(&self) -> i64 { self.total }
    pub fn get_modifier(&self, id: &str) -> i64 {
        return self.modifiers.get(String::from(id));
    }

    pub fn set_base(&mut self, value: i64) { self.base = value; }

    pub fn add_modifier(&mut self, id: &str, value: i64) {
        self.modifiers.insert(String::from(id), value);
        self.total += value;
    }

    pub fn remove_modifier(&mut self, id: &str) {
        self.modifiers.remove(String::from(id));
        self.total -= value;
    }

    pub fn change_modifier(&mut self, id: &str, new_value: i64) {
        self.modifiers[String::from(id)] = new_value;
    }

    pub fn has_modifiers(&self) -> bool {
        return !self.modifiers.is_empty();
    }
}