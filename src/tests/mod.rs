use crate::character::{CharacterErrors, CharacterService};

#[cfg(test)]
mod character;
mod character_name_tests;
mod level_tests;

struct MockedCharService {
    name_should_exist: bool,
}

impl CharacterService for MockedCharService {
    fn name_exists(&self, _name: &str) -> Result<bool, CharacterErrors> {
        Ok(self.name_should_exist)
    }

    fn lock_name(&self, _name: &str) -> Result<(), CharacterErrors> {
        Ok(())
    }
}
