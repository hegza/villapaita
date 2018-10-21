// Tells what this object is.
pub trait Subject {
    fn subject(&self) -> &FinnishNoun;
}
// Describes what this object is like.
pub trait Describe {
    fn describe<'a>(&'a self) -> &'a str;
}

pub struct FinnishNoun {
    pub nominative: String,
    pub illative: String,
    pub inessive: String,
}

pub struct Dictionary(Vec<FinnishNoun>);

impl Dictionary {
    pub fn get(&self, nominative: &str) -> Option<&FinnishNoun> {
        self.0.iter().find(|w| w.nominative == nominative)
    }
}

lazy_static! {
    pub static ref DICTIONARY: Dictionary = Dictionary(vec![
        FinnishNoun {nominative: "työhuone".to_string(), illative: "työhuoneeseen".to_string(), inessive: "työhuoneessa".to_string()},
        FinnishNoun {nominative: "asunto".to_string(), illative: "asuntoon".to_string(), inessive: "asunnossa".to_string()},
        FinnishNoun {nominative: "hanki".to_string(), illative: "hankeen".to_string(), inessive: "hangessa".to_string()},
        FinnishNoun {nominative: "lepohuone".to_string(), illative: "lepohuoneeseen".to_string(), inessive: "lepohuoneessa".to_string()},
    ]);
}
