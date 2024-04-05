use crate::character::character_name::*;

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
    fn name_exists(&self, _name: &str) -> Result<bool, CharacterNameServiceErrors> {
        Ok(self.name_should_exist)
    }

    fn lock_name(&self, _name: &str) -> Result<(), CharacterNameServiceErrors> {
        Ok(())
    }
}
