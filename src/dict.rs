use std::fmt;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use crate::util::*;

#[derive(Debug, PartialEq)]
pub struct Dict {
    pub words: Vec<String>,
    config: Config,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    voice: bool,
    accent: i32,
    more:bool,
}

impl Dict {
    pub fn new(words: Vec<String>, voice: bool, accent: i32, more: bool) -> Self {
        Dict {
            words,
            config: Config {
                voice,
                accent,
                more
            },
        }
    }
    
    pub fn voice_url(&self) -> String {
        format!(
            "{}{}{}",
            String::from("https://dict.youdao.com/dictvoice?audio="),
            utf8_percent_encode(&self.words.join("+")[..], DEFAULT_ENCODE_SET).to_string(),
            format!("&type={}", self.config.accent))
    }

    pub fn query_string(&self) -> String {
        self.words.join(" ")
    }
    pub fn query_url(&self) -> String {
        if is_chinese(&self.words.concat()[..]) {
            format!(
                "{}{}",
                String::from("http://dict.youdao.com/w/eng/"),
                utf8_percent_encode(&self.words.join(" ")[..], DEFAULT_ENCODE_SET).to_string())

        } else {
            format!(
                "{}{}",
                String::from("http://dict.youdao.com/w/"),
                utf8_percent_encode(&self.words.join(" ")[..], DEFAULT_ENCODE_SET).to_string())
        }
    }

    pub fn is_voice(&self) -> bool {
        self.config.voice
    }

    pub fn is_more(&self) -> bool {
        self.config.more
    }
}

impl fmt::Display for Dict {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "query words: {:?}, isVoice: {}, accent type: {}, isMore: {}",
            self.words, self.config.voice, self.config.accent, self.config.more)
    }
}
