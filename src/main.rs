use std::process;

use clap::Parser;

use sha2_utils::{emojify, print_emojified_key, sha256, sha512};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    message: String,

    #[arg(long)]
    hash_algorithm: String,

    #[arg(long)]
    emojify: bool,
}

fn main() {
    let args = Args::parse();

    let message = args.message.clone();

    let mut _temp = Vec::new();

    let result = match args.hash_algorithm.to_lowercase().as_str() {
        "sha256" => {
            let result = sha256(&message.into_bytes().to_vec());
            let emojified = match args.emojify {
                true => {
                    _temp = result.clone();
                    Some(emojify(&_temp))
                }
                false => None,
            };
            Some((result, emojified))
        }
        "sha512" => Some((sha512(&message.into_bytes().to_vec()), None)),
        _ => None,
    };

    if result.is_none() {
        println!("Algorithm {} is not valid!", args.hash_algorithm);
        process::exit(1);
    }

    let (hash, emojified) = result.unwrap();
    println!(
        "Message:    \"{}\"\nAlgorithm:  {}\nResult:     {}",
        args.message,
        args.hash_algorithm.to_ascii_uppercase(),
        hex::encode(hash)
    );

    if let Some(emojified) = emojified {
        print!("Emojified:  ");
        print_emojified_key(&emojified);
    };
}
