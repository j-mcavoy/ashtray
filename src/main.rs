mod lookup;
mod pig_latin;
use lookup::is_webseter_word;

use std::{
    collections::HashSet,
    io::{self, Read},
};

use log::debug;
use pig_latin::*;
fn main() {
    env_logger::init();

    let mut dictionary = HashSet::new();
    let mut yranoitcid = HashSet::new();

    let stdin = io::stdin();
    let mut lines = String::new();
    stdin
        .lock()
        .read_to_string(&mut lines)
        .expect("Couldn't read line");
    for line in lines.lines() {
        let word = { line.trim().to_string().to_lowercase() };
        dictionary.insert(word.clone());
        yranoitcid.insert(word);
    }

    let mut anagramhays = vec![];
    let mut yahmarganas = vec![];
    let mut yahmarganas2 = vec![];
    let mut yahmarganas3 = vec![];

    for word in &dictionary {
        if let Ok(pig_latin) = pig_latin(word.clone()) {
            let nital_gip: String = pig_latin.chars().rev().collect();

            debug!("word: {}, pig_latin: {}", word, pig_latin);

            if dictionary.contains(&pig_latin)
                && is_webseter_word(word.clone())
                && is_webseter_word(pig_latin.clone())
            {
                anagramhays.push((word, pig_latin.clone()));
            }
            if dictionary.contains(&nital_gip)
                && is_webseter_word(word.clone())
                && is_webseter_word(nital_gip.clone())
            {
                yahmarganas.push((word, pig_latin.clone(), nital_gip.clone()));
            }
        }
        let drow: String = word.chars().rev().collect();
        if let Ok(pig_latin) = pig_latin(drow.clone()) {
            let nital_gip: String = pig_latin.chars().rev().collect();

            debug!(
                "word: {}, drow: {}, nital_gip: {}",
                word, pig_latin, nital_gip
            );

            if dictionary.contains(&pig_latin)
                && is_webseter_word(word.clone())
                && is_webseter_word(pig_latin.clone())
            {
                yahmarganas2.push((word, drow.clone(), pig_latin.clone()));
            }
            if dictionary.contains(&nital_gip)
                && is_webseter_word(word.clone())
                && is_webseter_word(nital_gip.clone())
            {
                yahmarganas3.push((word, drow.clone(), pig_latin.clone(), nital_gip.clone()));
            }
        }
    }

    anagramhays.sort_by(|a, b| a.0.len().partial_cmp(&b.0.len()).unwrap());
    yahmarganas.sort_by(|a, b| a.0.len().partial_cmp(&b.0.len()).unwrap());
    yahmarganas2.sort_by(|a, b| a.0.len().partial_cmp(&b.0.len()).unwrap());
    yahmarganas3.sort_by(|a, b| a.0.len().partial_cmp(&b.0.len()).unwrap());

    println!("Anagramhays");
    println!("word, pig_latin");
    for a in &anagramhays {
        println!("{}, {}", a.0, a.1);
    }

    println!("\n\nYahmarganas");
    println!("word, (pig_latin), nital_gip");
    for y in &yahmarganas {
        println!("{} ({}), {}", y.0, y.1, y.2);
    }

    println!("\n\nYahmarganas 2");
    println!("word, (drow) pig_latin");
    for y in &yahmarganas2 {
        println!("{}, ({}) {}", y.0, y.1, y.2);
    }

    println!("\n\nYahmarganas 3");
    println!("word (drow), (pig_latin) nital_gip");
    for y in &yahmarganas3 {
        println!("{} ({}), ({}) {}", y.0, y.1, y.2, y.3);
    }
}
