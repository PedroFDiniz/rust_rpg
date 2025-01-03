#[path = "./property.rs"] pub mod property;
#[path = "./experience.rs"] pub mod experience;
#[path = "./attribute.rs"] pub mod attribute;
#[path = "./level.rs"] pub mod level;
use property::Property;
use experience::Experience;
use attribute::Attribute;
use level::Level;
use std::fmt::{Display, Formatter, Result};

pub struct Character {
    /* Properties */
    pub name: String,
    pub level: Level,
    pub experience: Experience,

    /* Attributes */
    pub agility: Attribute,
    pub charisma: Attribute,
    pub constitution: Attribute,
    pub dexterity: Attribute,
    pub intellect: Attribute,
    pub resolve: Attribute,
    pub strength: Attribute,
    pub wisdom: Attribute,

    /* Statistics */
    pub health: Property,
    pub mana: Property,
    pub stamina: Property,
}

impl Character {
    #![allow(unused)]
    pub fn new(character_name:&str) -> Character {
        return Character {
            name: String::from(character_name),
            level: Level::default(),
            experience: Experience::default(),

            agility: Attribute::default(),
            charisma: Attribute::default(),
            constitution: Attribute::default(),
            dexterity: Attribute::default(),
            intellect: Attribute::default(),
            resolve: Attribute::default(),
            strength: Attribute::default(),
            wisdom: Attribute::default(),

            health: Property::new(100),
            mana: Property::new(100),
            stamina: Property::new(100),
        }
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
        let con = u32::from(self.constitution.total(self.level.get()));
        return con * 10 * u32::from(self.level.get());
    }

    fn calc_max_mana(&self) -> u32 {
        let int = u32::from(self.intellect.total(self.level.get()));
        return int * 10 * u32::from(self.level.get());
    }

    fn calc_max_stamina(&self) -> u32 {
        let con = u32::from(self.constitution.total(self.level.get()));
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
            constitution: Default::default(),
            dexterity: Default::default(),
            intellect: Default::default(),
            resolve: Default::default(),
            strength: Default::default(),
            wisdom: Default::default(),
            experience: Default::default(),
        }
    }
}
