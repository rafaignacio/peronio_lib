use crate::character::{character_name::*, CharacterErrors, CharacterService};

#[test]
fn should_create_char_name() {
    let service = MockedCharService {
        name_should_exist: false,
    };
    match CharacterName::new("Rafael", &service) {
        Ok(_) => (),
        Err(_) => panic!("failed"),
    }
}

#[test]
fn should_fail_when_creating_char_name() {
    let service = MockedCharService {
        name_should_exist: true,
    };
    match CharacterName::new("Rafael", &service) {
        Err(CharacterErrors::NameAlreadyExists) => (),
        _ => panic!("failed"),
    }
}

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
