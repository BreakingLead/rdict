//! Reader of dict offline archives.

use std::io::Read;

use anyhow::{anyhow, Result};
use colored::Colorize;
use flate2::read::ZlibDecoder;
use serde::Deserialize;
use serde_json::Value;

const EN_INDEX_BUF: &str = include_str!("../assets/en.ind");
const ZH_INDEX_BUF: &str = include_str!("../assets/zh.ind");
const EN_DICT_BUF: &[u8] = include_bytes!("../assets/en.z");
// const ZH_DICT_BUF: &[u8] = include_bytes!("../assets/zh.z");

#[derive(Debug)]
pub enum Language {
    English,
    Chinese,
}

#[derive(Debug)]
pub struct Pronounciation {
    pub american: String,
    pub british: String,
    pub other: String,
}

#[derive(Deserialize, Debug)]
struct Paraphrase(Vec<String>);

#[derive(Debug)]
struct Example {
    origin: String,
    translation: String,
}

#[derive(Debug)]
struct Usage {
    usage: String,
    word_class: String,
    examples: Vec<Example>,
}

#[derive(Debug)]
pub struct WordInfo {
    pub lang: Language,
    pub word: String,
    pub id: u32,
    pub pronounciation: Pronounciation,
    pub paraphrase: Paraphrase,
    pub rank: String,
    pub pattern: String,
    /// sentences in json
    pub usages: Vec<Usage>,
}

pub fn detect_language(word: &str) -> Language {
    // non-robust implementation, modify later
    match word.is_ascii() {
        true => Language::English,
        false => Language::Chinese,
    }
}

pub fn get_raw_from_cache(word: &str) -> Result<(Language, String)> {
    // Find the index of the word.
    let mut index_buf: &str = "";
    let mut dict_buf_size = 0usize;

    // Find index buffer
    match detect_language(word) {
        Language::English => {
            index_buf = EN_INDEX_BUF;
            dict_buf_size = EN_DICT_BUF.len();
        }
        Language::Chinese => {
            index_buf = ZH_INDEX_BUF;
            // dict_buf_size = ZH_DICT_BUF.len();
            dict_buf_size = 0;
        }
    };

    let (index, next_index) = {
        let n = index_buf
            .lines()
            .position(|i| i.split_once("|").unwrap().0 == word)
            .ok_or(anyhow!("word not found"))?;
        let index = index_buf
            .lines()
            .nth(n)
            .unwrap()
            .split_once("|")
            .unwrap()
            .1
            .parse::<usize>()?;
        let next_index = index_buf
            .lines()
            .nth(n + 1)
            // If you query the last word in index There won't be the (n+1)th item so we should add a pseudo item in order to prevent
            // it happening.
            .unwrap_or(&format!("pseudo|{}", dict_buf_size))
            .split_once("|")
            .unwrap()
            .1
            .parse::<usize>()?;
        (index, next_index)
    };

    let str = match detect_language(word) {
        Language::English => {
            let bytes = EN_DICT_BUF;
            let mut decoder = ZlibDecoder::new(&bytes[index..next_index]);
            let mut str = Default::default();
            decoder.read_to_string(&mut str)?;
            str
        }
        Language::Chinese => {
            // let bytes = ZH_DICT_BUF;
            let bytes = [0u8; 1];
            let mut decoder = ZlibDecoder::new(&bytes[index..next_index]);
            let mut str = Default::default();
            decoder.read_to_string(&mut str)?;
            str
        }
    };

    Ok((detect_language(word), str))
}

pub fn get_word_info(word: &str) -> Result<WordInfo> {
    let (lang, raw) = get_raw_from_cache(word)?;
    dbg!(&raw);
    let mut raw = raw.split("|");

    let info = match lang {
        Language::English => {
            let word = raw.next().unwrap().to_string();
            let id = raw.next().unwrap().parse::<u32>()?;
            let am = raw.next().unwrap().to_string();
            let br = raw.next().unwrap().to_string();
            let other = raw.next().unwrap().to_string();
            let paraphrase = raw.next().unwrap();
            let paraphrase = serde_json::from_str(paraphrase).unwrap();
            let rank = raw.next().unwrap().to_string();
            let pattern = raw.next().unwrap().to_string();
            let usages = raw.next().unwrap();
            let usages: Value = serde_json::from_str(usages).unwrap();

            let usages = usages.as_array().unwrap();
            let usages = usages
                .into_iter()
                .map(|x| {
                    let x = x.as_array().unwrap();
                    let examples: Option<Vec<Example>> = (|| -> Option<Vec<Example>> {
                        Some(
                            (x.get(2)?.as_array().unwrap())
                                .into_iter()
                                .map(|e| {
                                    let e = e.as_array().unwrap();
                                    Example {
                                        origin: e[0].as_str().unwrap().to_string(),
                                        translation: e[1].as_str().unwrap().to_string(),
                                    }
                                })
                                .collect(),
                        )
                    })();

                    Usage {
                        usage: x[0].as_str().unwrap().to_string(),
                        word_class: x[1].as_str().unwrap().to_string(),
                        examples: examples.unwrap_or_default(),
                    }
                })
                .collect();

            let info = WordInfo {
                lang: Language::English,
                word,
                id: id,
                pronounciation: Pronounciation {
                    american: am,
                    british: br,
                    other,
                },
                paraphrase,
                rank,
                pattern,
                usages,
            };
            info
        }
        Language::Chinese => {
            todo!()
        }
    };
    Ok(info)
}

pub fn display_word_info(info: WordInfo) {
    println!();
    println!(
        "{} {} {} {} {}
{}
",
        info.word.bold().yellow(),
        info.pronounciation.american,
        info.pronounciation.british,
        info.pronounciation.other,
        info.rank.bold().green(),
        info.pattern.blue(),
    );

    for i in info.usages {
        println!("{} {}", i.word_class.red(), i.usage.green());
        for j in i.examples {
            println!("\t{}\n\t{}", j.origin.cyan(), j.translation.blue());
        }
        println!();
    }
}
