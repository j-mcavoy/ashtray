use log::debug;
use serde_derive::Deserialize;
use serde_derive::Serialize;

const WEBSTER_API_KEY: &str = "API_KEY_HERE";

// https://dictionaryapi.com/products/json
pub fn is_webseter_word(word: String) -> bool {
    if let Ok(body) = reqwest::blocking::get(format!(
        "https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}",
        word, WEBSTER_API_KEY
    ))
    .unwrap()
    .json::<Root>()
    {
        let fls: Vec<String> = body.iter().map(|entry| entry.fl.clone()).collect();
        for fl in fls {
            if fl == "noun" || fl == "verb" || fl == "adjective" {
                debug!("{} is good {}", word, fl);
                return true;
            } else {
                continue;
            }
        }
        debug!("{} is bad", word);
        return false;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_word() {
        assert!(!is_webseter_word("wef".to_string()));
        assert!(!is_webseter_word("rd".to_string()));
    }
    #[test]
    fn test_good_word() {
        assert!(is_webseter_word("sway".to_string()));
        assert!(is_webseter_word("panda".to_string()));
    }
}

pub type Root = Vec<Entry>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub fl: String,
}
