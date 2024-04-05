use super::{CharacterErrors, CharacterService};

#[derive(Debug, PartialEq, Default)]
pub struct CharacterName(String);

impl CharacterName {
    pub fn new(
        name: &str,
        service: &impl CharacterService,
    ) -> Result<CharacterName, CharacterErrors> {
        if let Ok(false) = service.name_exists(name) {
            service.lock_name(name)?;

            return Ok(CharacterName(name.to_string()));
        }

        eprintln!("Name {name} already exists on database.");
        Err(CharacterErrors::NameAlreadyExists)
    }
}
