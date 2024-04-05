use self::{character_name::CharacterName, level::Level};

pub mod character_name;
pub mod level;

#[derive(Debug, PartialEq)]
pub enum CharacterErrors {
    NameAlreadyExists,
}

pub trait CharacterService {
    fn name_exists(&self, name: &str) -> Result<bool, CharacterErrors>;
    fn lock_name(&self, name: &str) -> Result<(), CharacterErrors>;
}

pub struct Character {
    id: String,
    name: CharacterName,
    level: Level,
    service: Box<dyn CharacterService>,
}

impl Character {
    pub fn new(service: impl CharacterService + 'static) -> Result<Character, CharacterErrors> {
        Ok(Character {
            service: Box::new(service),
            level: Level::default(),
            id: String::default(),
            name: CharacterName::default(),
        })
    }
}
