#[derive(Debug, PartialEq)]
pub enum CharacterNameServiceErrors {
    NameAlreadyExists
}

pub trait CharacterNameService {
    fn name_exists(&self, name: &str) -> Result<bool, CharacterNameServiceErrors>;
    fn lock_name(&self, name: &str) -> Result<(), CharacterNameServiceErrors>;
}

#[derive(Debug, PartialEq)]
pub struct CharacterName(String);

impl CharacterName {
    pub fn new(name: &str, service: &impl CharacterNameService)
        -> Result<CharacterName, CharacterNameServiceErrors> {
        if let Ok(false) = service.name_exists(name) {
            service.lock_name(name)?;
            return Ok(CharacterName(name.to_string()));
        } else {
           Err(CharacterNameServiceErrors::NameAlreadyExists)
        }
    }
}
