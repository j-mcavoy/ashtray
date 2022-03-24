use std::char;

use log::debug;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyString,
}
trait CharClassification {
    fn is_vowel(&self) -> bool;
    fn is_constanant(&self) -> bool {
        !self.is_vowel()
    }
}

impl CharClassification for char {
    fn is_vowel(&self) -> bool {
        matches!(self, 'a' | 'e' | 'i' | 'o' | 'u')
    }
}

fn consonant_cluster(input: String) -> (String, String) {
    let mut iter = input.chars().peekable();

    let mut cluster = vec![];

    while iter.peek().is_some() && iter.peek().unwrap().is_constanant() {
        cluster.push(iter.next().unwrap());
    }

    let cluster = cluster.iter().collect();
    let and_the_rest = iter.collect();

    debug!("cluster = {:?}", cluster);
    debug!("and_the_rest = {:?}", and_the_rest);
    (cluster, and_the_rest)
}

pub fn pig_latin(input: String) -> Result<String, Error> {
    let mut chars = input.chars().peekable();
    chars.peek().ok_or(Error::EmptyString)?;

    match chars.peek() {
        None => Ok(format!("{}ay", chars.collect::<String>())),
        Some(first) => {
            if first.is_vowel() {
                debug!("first is a vowel: {}", first);
                Ok(format!("{}hay", input))
            } else {
                debug!("first is consonant: {}", first);
                let (cluster, rest) = consonant_cluster(input.to_string());
                Ok(format!("{}{}ay", rest, cluster))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zero_length() {
        assert_eq!(pig_latin("".to_string()).unwrap_err(), Error::EmptyString);
    }

    #[test]
    fn test_pig_latin_single_char() {
        assert_eq!(pig_latin("f".to_string()).unwrap(), "fay".to_string());
    }

    #[test]
    fn test_single_constanant() {
        assert_eq!(pig_latin("pig".to_string()).unwrap(), "igpay".to_string());
        assert_eq!(
            pig_latin("latin".to_string()).unwrap(),
            "atinlay".to_string()
        );
    }

    #[test]
    fn test_consonent_clusters() {
        assert_eq!(
            pig_latin("trash".to_string()).unwrap(),
            "ashtray".to_string()
        );
        assert_eq!(
            pig_latin("sprant".to_string()).unwrap(),
            "antspray".to_string()
        );
    }
}
