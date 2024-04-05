use crate::character::{Character, CharacterErrors};

use super::MockedCharService;

#[test]
fn should_create_char() -> Result<(), CharacterErrors> {
    let service = MockedCharService {
        name_should_exist: false,
    };
    let _ = Character::new(service)?;

    Ok(())
}
