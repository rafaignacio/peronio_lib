use crate::character::*;

use self::character_name::{CharacterName, CharacterNameService, CharacterNameServiceErrors};

#[test]
fn should_create_char_name() {
    let service = MockedCharNameService {
        name_should_exist: false,
    };
    match CharacterName::new("Rafael", &service) {
        Ok(_) => (),
        Err(_) => panic!("failed"),
    }
}

#[test]
fn should_fail_when_creating_char_name() {
    let service = MockedCharNameService {
        name_should_exist: true,
    };
    match CharacterName::new("Rafael", &service) {
        Err(CharacterNameServiceErrors::NameAlreadyExists) => (),
        _ => panic!("failed"),
    }
}

struct MockedCharNameService {
    name_should_exist: bool,
}

impl CharacterNameService for MockedCharNameService {
    fn name_exists(&self, name: &str) -> Result<bool, character_name::CharacterNameServiceErrors> {
        Ok(self.name_should_exist)
    }

    fn lock_name(&self, name: &str) -> Result<(), character_name::CharacterNameServiceErrors> {
        Ok(())
    }
}
