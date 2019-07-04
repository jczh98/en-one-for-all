extern crate clap;
extern crate reqwest;
extern crate ansi_term;
extern crate itertools;
extern crate scraper;

use clap::{App, SubCommand, Arg};
use scraper::Html;
use tempfile::NamedTempFile;
use std::process::Command;
use std::io::prelude::*;

mod dict;
mod util;
mod parser;

use crate::dict::*;
use crate::parser::*;

fn main() {
    let matches = App::new("enofa")
        .version("0.0.1")
        .author("neverfelly. neverfelly@gmail.com")
        .about("A one for all tool for english learning.")
        .subcommand(SubCommand::with_name("dict")
                    .about("Query a word from dict")
                    .arg(Arg::with_name("voice")
                         .short("v")
                         .long("voice")
                         .help("show voice."))
                    .arg(Arg::with_name("more")
                         .short("m")
                         .long("more")
                         .help("show more."))
                    .arg(Arg::with_name("accent")
                         .short("a")
                         .long("accent")
                         .takes_value(true)
                         .help("decide accent, 1 for UK and 2 for US."))
                    .arg(Arg::with_name("word") .help("query a word from network.")
                         .multiple(true)))
        .arg_from_usage("-q, --query=[WORD] 'query a word from network'")
        .get_matches();

    if let Some(word) = matches.value_of("query") {
        println!("word is {}", word);
    }

    match matches.subcommand() {
        ("dict", Some(dict_matches)) => {
            let mut voice = false;
            let mut more = false;
            let mut accent = 1;
            if dict_matches.is_present("voice") {
                println!("allows voice.");
                voice = true;
            }
            if dict_matches.is_present("more") {
                println!("allows more.");
                more = true;
            }
            if let Some(accent_type) = dict_matches.value_of("accent") {
                println!("accent is {}", accent_type);
                accent = accent_type.parse().unwrap();
            }
            let mut words = Vec::new();
            for word in dict_matches.values_of("word").unwrap() {
                println!("Cloning {}", word);
                words.push(String::from(word));
            }
            let dict = Dict::new(words, voice, accent, more);
            let body = reqwest::get(dict.query_url().as_str())
                .unwrap()
                .text()
                .unwrap();
            let document = Html::parse_document(&body);
            if let Err(e) = parse_and_print(&document, &dict.query_string(), dict.words.len() > 1) {
                panic!(format!("{:?}", e));
            }
            if dict.is_voice() {
                if let Err(e) =  play_sound(&dict) {
                    panic!(format!("{:?}", e));
                }
            }
        },
        ("", None) => println!("No subcommand was uesd"),
        _ => unreachable!(),
    }
}

fn play_sound(dict: &Dict) -> Result<(), String> {
    println!("voice url is {}", dict.voice_url());
    let mut voice_response = reqwest::get(dict.voice_url().as_str())
        .map_err(|e| format!("{}", e))?;
    let mut buf: Vec<u8> = vec![];
    voice_response.copy_to(&mut buf)
        .map_err(|e| format!("{}", e))?;
    let mut file = NamedTempFile::new()
        .map_err(|e| format!("{}", e))?;
    file.write_all(&buf)
        .map_err(|e| format!("{}", e))?;
    Command::new("mpg123")
        .arg(file.path().as_os_str())
        .output()
        .map_err(|e| format!("{}", e))?;
    Ok(())
}
