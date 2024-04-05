use crate::character::level::Level;

#[test]
fn should_convert_level_properly() {
    let mut level = Level::from(0);
    println!("{level}");
    assert_eq!(1, level.get_current_level());

    level = Level::from(101);
    println!("{level}");
    assert_eq!(2, level.get_current_level());

    level = Level::from(1_000_000);
    println!("{level}");
    assert_eq!(41, level.get_current_level());

    level = Level::from(1_000_000_000);
    println!("{level}");
    assert_eq!(393, level.get_current_level());

    level = Level::from(16_567_000_000);
    println!("{level}");
    assert_eq!(1000, level.get_current_level());
}
