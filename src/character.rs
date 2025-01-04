#[path = "./level.rs"] pub mod level;
#[path = "./property.rs"] pub mod property;
#[path = "./statistic.rs"] pub mod statistic;
#[path = "./attribute.rs"] pub mod attribute;
#[path = "./experience.rs"] pub mod experience;
use level::Level;
use property::Property;
use statistic::Statistic;
use attribute::Attribute;
use experience::Experience;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
enum Attributes {
    AGI,
    END,
    DEX,
    STR,
    CHA,
    INT,
    RES,
    WIS,
}

pub struct Character {
    /* Properties */
    pub name: String,
    pub level: Level,
    pub experience: Experience,

    /* Attributes */
    pub agility: Attribute,
    pub endurance: Attribute,
    pub dexterity: Attribute,
    pub strength: Attribute,
    pub charisma: Attribute,
    pub intellect: Attribute,
    pub resolve: Attribute,
    pub wisdom: Attribute,

    /* Statistics */
    pub health: Property,
    pub mana: Property,
    pub stamina: Property,
    perception: Statistic,
    initiative: Statistic,
    attack_power: Statistic,
    magic_power: Statistic,
    fortitude: Statistic,
    reflexes: Statistic,
    willpower: Statistic,
    charm: Statistic,
    critical_strike_chance: Statistic,
    critical_strike_power: Statistic,
    miracle_chance: Statistic,
    miracle_power: Statistic,

}

impl Character {
    #![allow(unused)]
    pub fn new(character_name:&str) -> Character {
        let mut result = Character {
            name: String::from(character_name),
            level: Level::default(),
            experience: Experience::default(),

            agility: Attribute::default(),
            charisma: Attribute::default(),
            endurance: Attribute::default(),
            dexterity: Attribute::default(),
            intellect: Attribute::default(),
            resolve: Attribute::default(),
            strength: Attribute::default(),
            wisdom: Attribute::default(),

            health: Property::default(),
            mana: Property::default(),
            stamina: Property::default(),
        };
        result.update();
        return result;
    }

    pub fn update(&mut self) {
        self.recalculate_level();
        self.health.set_max(self.calc_max_health());
        self.mana.set_max(self.calc_max_mana());
        self.stamina.set_max(self.calc_max_stamina());
    }

    fn recalculate_level(&mut self) {
        let factor = self.experience.get_max() / u64::from(self.level.get_max());
        self.level.set((self.experience.get() / factor) as u16);
    }

    fn calc_max_health(&self) -> u32 {
        let con = u32::from(self.endurance.total(self.level.get()));
        return con * 10 * u32::from(self.level.get());
    }

    fn calc_max_mana(&self) -> u32 {
        let int = u32::from(self.wisdom.total(self.level.get()));
        return int * 10 * u32::from(self.level.get());
    }

    fn calc_max_stamina(&self) -> u32 {
        let con = u32::from(self.endurance.total(self.level.get()));
        let str = u32::from(self.strength.total(self.level.get()));
        let agi = u32::from(self.agility.total(self.level.get()));
        let dex = u32::from(self.dexterity.total(self.level.get()));
        let lvl = u32::from(self.level.get());
        return ((agi + con + dex + str) * 10 * lvl) / 4;
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,
            "Name: {}\nHealth: {}\nMana: {}\nStamina: {}",
            self.name,
            self.health,
            self.mana,
            self.stamina)
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::from("João da Silva"),
            level: Default::default(),
            health: Default::default(),
            mana: Default::default(),
            stamina: Default::default(),
            agility: Default::default(),
            charisma: Default::default(),
            endurance: Default::default(),
            dexterity: Default::default(),
            intellect: Default::default(),
            resolve: Default::default(),
            strength: Default::default(),
            wisdom: Default::default(),
            experience: Default::default(),
        }
    }
}
