extern crate clap;
extern crate reqwest;
extern crate ansi_term;
extern crate itertools;
extern crate scraper;
extern crate serde_json;
extern crate serde;
extern crate rand;

use clap::{App, SubCommand, Arg};
use rand::{thread_rng};
use rand::seq::SliceRandom;
use scraper::Html;
use tempfile::NamedTempFile;
use serde::Deserialize;
use std::process::Command;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::error::Error;

mod dict;
mod util;
mod parser;

use crate::dict::*;
use crate::parser::*;

struct Pair {
    key: String,
    val: String,
}

fn main() {
    let matches = App::new("enofa")
        .version("0.0.1")
        .author("neverfelly. neverfelly@gmail.com")
        .about("A one for all tool for english learning.")
        .subcommand(SubCommand::with_name("recite")
                    .about("Reciate a list of dict.")
                    .arg(Arg::with_name("begin")
                         .short("a")
                         .long("begin")
                         .takes_value(true)
                         .help("set a begin."))
                    .arg(Arg::with_name("end")
                         .short("b")
                         .long("end")
                         .takes_value(true)
                         .help("set a end."))
                    .arg(Arg::with_name("FILE")
                         .short("f")
                         .long("file")
                         .takes_value(true)
                         .help("choose a file"))
                    .arg(Arg::with_name("wrong")
                         .short("o")
                         .long("output")
                         .help("wrong answer output.")))
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
        .get_matches();

    match matches.subcommand() {
        ("recite", Some(recite_matches)) => {
            let mut a = 0;
            let mut b = 0;
            let mut dict_file = "resources/all.json";
            let mut wrong_file = "wrong.json";
            if let Some(begin) = recite_matches.value_of("begin") {
                a = begin.parse().unwrap();
            }
            if let Some(end) = recite_matches.value_of("end") {
                b = end.parse().unwrap();
            }
            if let Some(arg_file) = recite_matches.value_of("FILE") {
                dict_file = arg_file;
            }
            match read_file(String::from(dict_file)) {
                Ok(data) => {
                    let mut vec = Vec::new();
                    for pair in &data {
                        for (key, val) in pair {
                            vec.push((key, val))
                        }
                    }
                    if b == 0 {
                        b = vec.len();
                    }
                    &vec[a..b].shuffle(&mut thread_rng());
                    for index in (a..b) {
                        println!("{}", vec[index].0);
                        let mut input = String::new();
                        match io::stdin().read_line(&mut input) {
                            Ok(n) => {
                                println!("");
                            }
                            Err(e) => {
                                println!("error: {}", e);
                            }
                        }
                        println!("{}", vec[index].1); 
                        match io::stdin().read_line(&mut input) {
                            Ok(n) => {
                                println!("");
                            }
                            Err(e) => {
                                println!("error: {}", e);
                            }
                        } 
                    }
                },
                Err(e) => {
                    panic!("error: {}", e);
                },
            }
        }
        ("dict", Some(dict_matches)) => {
            let mut voice = false;
            let mut more = false;
            let mut accent = 1;
            if dict_matches.is_present("voice") {
                voice = true;
            }
            if dict_matches.is_present("more") {
                more = true;
            }
            if let Some(accent_type) = dict_matches.value_of("accent") {
                accent = accent_type.parse().unwrap();
            }
            let mut words = Vec::new();
            for word in dict_matches.values_of("word").unwrap() {
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

fn read_file(path: String) -> Result<Vec<HashMap<String, String>>, Box<Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u: Vec<HashMap<String, String>> = serde_json::from_reader(reader)?;
    Ok(u)
}

fn play_sound(dict: &Dict) -> Result<(), String> {
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
