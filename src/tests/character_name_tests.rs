use crate::character::{character_name::*, CharacterErrors};

use super::MockedCharService;

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
