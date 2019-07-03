use std::fmt;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

#[derive(Debug, PartialEq)]
pub struct Dict {
    pub words: Vec<String>,
    config: Config,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    voice: bool,
    accent: i32,
}

impl Dict {
    pub fn new(words: Vec<String>, voice: bool, accent: i32) -> Self {
        Dict {
            words,
            config: Config {
                voice,
                accent
            },
        }
    }
    
    pub fn query_url(&self) -> String {
        format!(
            "{}{}",
            String::from("https://dict.youdao.com/fsearch?q="),
            utf8_percent_encode(&self.words.join(" ")[..], DEFAULT_ENCODE_SET).to_string())
    }
}

impl fmt::Display for Dict {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "query words: {:?}, isVoice: {}, accent type: {}",
            self.words, self.config.voice, self.config.accent)
    }
}
