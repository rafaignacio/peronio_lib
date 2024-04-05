use crate::character::{Character, CharacterErrors};

use super::MockedCharService;

#[test]
fn should_create_char() -> Result<(), CharacterErrors> {
    let service = MockedCharService {
        name_should_exist: false,
    };
    let mut c = Character::new(&service)?;

    c.create("Rafael")?;
    assert_eq!(c.get_level(), 1);
    assert_ne!(c.get_id(), String::default());

    Ok(())
}
