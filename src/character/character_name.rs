use super::{CharacterErrors, CharacterService};

#[derive(Debug, PartialEq, Default)]
pub struct CharacterName(String);

impl CharacterName {
    pub fn new(
        name: &str,
        service: &dyn CharacterService,
    ) -> Result<CharacterName, CharacterErrors> {
        if let Ok(false) = service.check_name_existance(name) {
            service.lock_name(name)?;

            return Ok(CharacterName(name.to_string()));
        }

        eprintln!("Name {name} already exists on database.");
        Err(CharacterErrors::NameAlreadyExists)
    }
}
