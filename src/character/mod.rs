use std::borrow::Borrow;

use self::{character_name::CharacterName, level::Level};

pub mod character_name;
pub mod level;

#[derive(Debug, PartialEq)]
pub enum CharacterErrors {
    NameAlreadyExists,
}

pub trait CharacterService {
    fn check_name_existance(&self, name: &str) -> Result<bool, CharacterErrors>;
    fn lock_name(&self, name: &str) -> Result<(), CharacterErrors>;
}

pub struct Character<'a> {
    id: String,
    name: CharacterName,
    level: Level,
    service: &'a dyn CharacterService,
}

impl Character<'_> {
    pub fn new(service: &dyn CharacterService) -> Result<Character, CharacterErrors> {
        Ok(Character {
            service,
            level: Level::default(),
            id: String::default(),
            name: CharacterName::default(),
        })
    }

    pub fn create<'a>(&mut self, name: &str) -> Result<(), CharacterErrors> {
        self.id = String::from("");
        self.name = CharacterName::new(name, self.service)?;
        self.level = Level::from(0);

        Ok(())
    }

    pub fn get_id(self) -> String {
        self.id
    }

    pub fn get_level(&self) -> u64 {
        self.level.get_current_level()
    }
}
