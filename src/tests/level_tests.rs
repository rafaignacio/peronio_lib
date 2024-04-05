use crate::character::level::Level;

#[test]
fn should_convert_level_properly() {
    let level = Level::from(0);
    assert_eq!(0, level.get_current_level());

    println!("{level}");

    let level = Level::from(1_000_000);
    println!("{level}");
    assert_eq!(41, level.get_current_level());

    let level = Level::from(1_000_000_000);
    println!("{level}");
    assert_eq!(400, level.get_current_level());
}
