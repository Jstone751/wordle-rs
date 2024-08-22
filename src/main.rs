use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use dialoguer::Input;
use reqwest::blocking::Client;

fn no_color_wordle_check(guess: &str, answer: &str) -> String {
    let mut result = String::new();
    for (i, l) in guess.chars().enumerate() {
        if l == answer.chars().nth(i).unwrap() {
            result.push('G');
        } else if answer.contains(l) {
            result.push('Y');
        } else {
            result.push('B');
        }
    }
    result
}

fn wordle_check(guess: &str, answer: &str) -> String {
    let mut result = String::new();
    for (i, l) in guess.chars().enumerate() {
        if l == answer.chars().nth(i).unwrap() {
            result.push_str(format!("{}", l.to_string().bright_green()).as_str());
        } else if answer.contains(l) {
            result.push_str(format!("{}", l.to_string().bright_yellow()).as_str());
        } else {
            result.push_str(format!("{}", l.to_string().bright_white()).as_str());
        }
    }
    result
}

fn get_wordle_answer_from_api() -> String {
    let client = Client::new();
    let response = client
        .get("https://random-word-api.herokuapp.com/word?length=5")
        .send();
    match response {
        Ok(res) => res.text().unwrap().replace(['[', ']', ' ', '"'], ""),
        Err(err) => panic!("Problem getting word! Failed with error: {:?}", err),
    }
}

fn main() {
    let matches = Command::new("Wordle")
        .arg(
            Arg::new("no-color")
                .short('n')
                .long("no-color")
                .help("Disables colored output")
                .action(ArgAction::SetTrue),
        )
        .about("A wordle clone written in rust")
        .arg(
            Arg::new("begin")
                .default_value("true")
                .short('b')
                .help("Begins the game")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    let no_color = matches.get_flag("no-color");
    if !matches.get_flag("begin") {
        panic!("Please begin the game!")
    }
    let answer = get_wordle_answer_from_api();
    let mut tries: i32 = 0;
    for _ in 0..6 {
        let guess = Input::<String>::new()
            .report(false)
            .allow_empty(false)
            .validate_with(|word: &String| -> Result<(), String> {
                if word.chars().count() != 5 {
                    Err("Word must be five letters long".to_string())
                } else {
                    Ok(())
                }
            })
            .with_prompt("Guess a five letter word")
            .interact_text()
            .unwrap();
        if no_color {
            let result = no_color_wordle_check(guess.as_str(), answer.as_str());
            println!("{}", result)
        }
        let result = wordle_check(guess.as_str(), answer.as_str());
        println!("{}", result.bold());
        if guess == answer {
            println!("{}", "You win!".green().bold());
            break;
        } else {
            tries += 1
        }
        if tries == 6 {
            println!(
                "{}{}",
                "You lose!".red().bold(),
                format!(" The word was {}", answer.bold().underline()).as_str()
            );
            break;
        }
    }
}
