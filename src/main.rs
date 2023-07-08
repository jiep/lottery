use clap::{Args, Parser, Subcommand};
use lottery::{checker::check::check, finder::lottery::Lottery};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check(Check),
    Find(Find),
}

#[derive(Args)]

/// Find where a lottery number is located
struct Find {
    /// Number to find
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(0..99999))]
    number: u32,

    /// Set url to download locations from
    #[arg(short, long, default_value_t = String::from("https://www.loteriasyapuestas.es/new-geo-web/JsonGenerationServlet/exportPois.txt?drawId=1222809102&number="))]
    url: String,

    /// Return as json
    #[arg(short, long, action)]
    json: bool,
}

#[derive(Args)]
/// Check if a lottery number is prizewinning
struct Check {
    #[arg(short, long, num_args(1..), value_parser = clap::value_parser!(u32).range(0..99999))]
    numbers: Vec<u32>,

    /// Set url to download prizes from
    #[arg(short, long, default_value_t = String::from("https://www.loteriasyapuestas.es/servicios/premioDecimoWeb?idsorteo=1186309102"))]
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Find(args) => {
            let lottery = Lottery::load_from_url(args.url.as_ref(), args.number).unwrap();

            println!(
                "{}",
                if args.json {
                    lottery.parse_to_json().unwrap()
                } else {
                    lottery.to_string()
                }
            );
        }
        Commands::Check(args) => {
            let to_check: Vec<String> = args.numbers.iter().map(|x| x.to_string()).collect();
            if let Ok(prizes) = check(args.url.as_str(), to_check) {
                let mut sum = 0_u32;
                prizes.iter().for_each(|(number, prize)| {
                    let prize = prize / 100;
                    sum += prize;
                    println!(
                        "{:0>5} => {} euros {}",
                        number,
                        prize,
                        if prize > 0_u32 { "🎉" } else { "😭" }
                    );
                });
                println!("\nTotal: {sum} euros");
            }
        }
    }

    Ok(())
}
