use self::{character_name::CharacterName, level::Level};

pub mod character_name;
pub mod level;

pub enum CharacterErrors {}

pub trait CharacterService {}

pub struct Character {
    id: String,
    name: CharacterName,
    level: Level,
}

impl Character {
    pub fn new(name: CharacterName, xp_point: u128) -> Result<Character, CharacterErrors> {
        todo!()
    }
}
