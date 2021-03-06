use std::collections::HashMap;

// Tells what this object is.
pub trait Subject {
    fn subject(&self) -> &FinnishNoun;
}
// Describes what this object is like.
pub trait Describe {
    fn describe<'a>(&'a self) -> &'a str;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FinnishNoun {
    pub nominative: String,
    pub illative: String,
    pub inessive: String,
}

// TODO: replace data structure with a hashmap
#[derive(Serialize, Deserialize)]
pub struct Dictionary(HashMap<String, FinnishNoun>);

impl Dictionary {
    pub fn get(&self, key: &str) -> Option<&FinnishNoun> {
        self.0.get(key)
    }
}

use ron::de::from_reader;
use std::fs::File;
lazy_static! {
    pub static ref DICTIONARY: Dictionary = {
        let input_path = format!("{}/data/finnish_dictionary.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        let dictionary: Dictionary = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load dictionary: {}", e);

                ::std::process::exit(1);
            }
        };
        dictionary
    };
}
