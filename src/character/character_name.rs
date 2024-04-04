pub trait CharacterNameService {
    fn name_exists(name: String) -> Option<bool>;
}

pub struct CharacterName(String);

impl CharacterName {
    pub fn new(name: String, service: impl CharacterNameService)
        -> CharacterName {
        todo!()
    }
}
