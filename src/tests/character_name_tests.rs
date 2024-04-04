use crate::character::*;

use self::character_name::CharacterNameService;

#[test]
fn should_create_char_name() {
    let service = MockedCharNameService{
        name_should_exist: false
    };
    match character_name::CharacterName::new("Rafael",
        &service) {
        Ok(_) => (),
        Err(_) => panic!("failed")
    }
}

struct MockedCharNameService {
    name_should_exist: bool,
}

impl CharacterNameService for MockedCharNameService {
    fn name_exists(&self, name: &str) ->
        Result<bool, character_name::CharacterNameServiceErrors> {
        Ok(self.name_should_exist)
    }

    fn lock_name(&self, name: &str) -> Result<(), character_name::CharacterNameServiceErrors> {
        Ok(())
    }
}
