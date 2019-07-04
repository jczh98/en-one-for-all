use itertools::join;
use scraper::element_ref::Select;
use scraper::{Html, Selector};
use ansi_term::Colour;
use crate::util::*;

#[derive(Debug)]
pub enum ParseError {
    CssParseError(&'static str),
    NilError,
}

pub fn parse_and_print(fragment: &Html, query: &str, is_multi: bool) -> Result<(), ParseError> {
    if is_chinese(query) {
        basic_query_chn(&fragment)?;
    } else {
        if hint_eng(&fragment, query)? {
            return Ok(());
        }
        basic_query_eng(&fragment, is_multi)?;
    }
    Ok(())
}

pub fn query_sentences(fragment: &Html, is_more: bool) -> Result<(), ParseError> {
    let sentences_selector = Selector::parse("div#bilingual > ul > li")
        .map_err(|_| ParseError::CssParseError("parse div#bilingual > ul > li"))?;
    let sentences = fragment.select(&sentences_selector);
    let mut count = 1;
    println!("");
    for s in sentences {
        let p_selector = Selector::parse("p").map_err(|_| ParseError::CssParseError("parse p"))?;
        let ps = s.select(&p_selector);
        for (i, p) in ps.enumerate() {
            if i == 0 {
                print!(
                    "{}",
                    Colour::Green.paint(format!(
                        "  {}. {}",
                        count,
                        join(&p.text().collect::<Vec<_>>(), "").trim()
                    ))
                )
            }
            if i == 1 {
                print!(
                    "     {}",
                    Colour::Blue.paint(join(&p.text().collect::<Vec<_>>(), "").trim())
                );
                count += 1
            }
            println!("");
            if count == 4 && !is_more {
                println!("");
                return Ok(());
            }
        }
    }
    println!("");
    Ok(())
}

fn hint_eng(fragment: &Html, query: &str) -> Result<bool, ParseError> {
    let typo_selector =
        Selector::parse(".typo-rel").map_err(|_| ParseError::CssParseError("parse .typo-rel"))?;
    let typos = fragment.select(&typo_selector).collect::<Vec<_>>();
    if typos.len() == 0 {
        return Ok(false);
    }

    println!("");
    println!(
        "{}",
        Colour::Blue.paint(format!("     word(s) '{}' not found, do you mean?", query))
    );
    println!("");

    for t in typos {
        let word_selector = Selector::parse("a").map_err(|_| ParseError::CssParseError("parse a"))?;
        let words = match t.select(&word_selector).next() {
            Some(w) => w,
            None => return Ok(false),
        };
        println!(   
            "     {}",
            Colour::Green.paint(words.text().next().ok_or(ParseError::NilError)?)
        );
        println!(
            "     {}",
            Colour::Yellow.paint(t.text().last().ok_or(ParseError::NilError)?.trim())
        );
    }
    Ok(true)
}

fn basic_query_chn(fragment: &Html) -> Result<(), ParseError> {
    println!("");
    let chn_selector = Selector::parse(".trans-container > ul > p")
        .map_err(|_| ParseError::CssParseError("parse trans-container > ul > p"))?;
    let chn = fragment.select(&chn_selector);
    let mut meanings = Vec::new();
    for c in chn {
        let search_seletor = Selector::parse(".contentTitle > .search-js")
            .map_err(|_| ParseError::CssParseError("parse .contentTitle > .search-js"))?;
        let search_content = c.select(&search_seletor);
        for s in search_content {
            let t = s.text();
            t.for_each(|s| meanings.push(s));
        }
        print!(
            "       {}",
            Colour::Blue.paint(c.text().skip(1).next().ok_or(ParseError::NilError)?.trim())
        );
        let joined_meaning = join(&meanings, ";");
        print!("  {}", Colour::Yellow.paint(joined_meaning));
    }

    Ok(())
}

fn basic_query_eng(fragment: &Html, is_multi: bool) -> Result<(), ParseError> {
    println!("");
    if !is_multi {
        let pronounce_selector = Selector::parse("div.baav > span.pronounce")
            .map_err(|_| ParseError::CssParseError("parse div.baav > span.pronounce"))?;
        let pronounce = fragment.select(&pronounce_selector);
        for (i, n) in pronounce.enumerate() {
            let phonetic_selector = Selector::parse("span.phonetic")
                .map_err(|_| ParseError::CssParseError("parse span.phonetic"))?;
            let phonetic = n.select(&phonetic_selector);
            if i == 0 {
                print!("    {} ", Colour::Yellow.bold().paint("英："));
                pronounce_output_select(phonetic)?;
            } else {
                print!("{} ", Colour::Blue.bold().paint("美："));
                pronounce_output_select(phonetic)?;
            }
        }
    }

    println!("");
    println!("");
    //means
    let means_selector = Selector::parse("div#phrsListTab > div.trans-container > ul").map_err(
        |_| ParseError::CssParseError("parse div#phrsListTab > div.trans-container > ul"),
    )?;
    let means = fragment.select(&means_selector);
    for m in means {
        println!(
            "  {}",
            Colour::Blue.paint(join(&m.text().collect::<Vec<_>>(), ""))
        );
    }
    Ok(())
}

fn pronounce_output_select(select: Select) -> Result<(), ParseError> {
    let t_vec = select
        .collect::<Vec<scraper::ElementRef>>()
        .first()
        .ok_or(ParseError::NilError)?
        .text()
        .collect::<Vec<_>>();

    let text = t_vec.first().ok_or(ParseError::NilError)?;

    print!("{}   ", Colour::Blue.paint(*text));
    Ok(())
}
