extern crate clap;
extern crate reqwest;

use clap::{App, SubCommand, Arg};

mod dict;

use crate::dict::*;

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
            let mut accent = 1;
            if dict_matches.is_present("voice") {
                println!("allows voice.");
                voice = true;
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
            let dict = Dict::new(words, voice, accent);
            let mut body = reqwest::get(dict.query_url().as_str())
                .unwrap()
                .text()
                .unwrap();
            println!("{}", body);
        },
        ("", None) => println!("No subcommand was uesd"),
        _ => unreachable!(),
    }
}
