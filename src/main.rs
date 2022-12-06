use clap::{arg, Parser};
use lottery::finder::lottery::Lottery;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number to find
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(0..99999))]
    number: u32,

    /// Set url to download locations from
    #[arg(short, long, default_value_t = String::from("https://www.loteriasyapuestas.es/new-geo-web/JsonGenerationServlet/exportPois.txt?drawId=1186309102&number="))]
    url: String,

    /// Return as json
    #[arg(short, long, action)]
    json: bool,
}

fn main() {
    let args = Args::parse();

    let lottery = Lottery::load_from_url(args.url, args.number).unwrap();

    println!(
        "{}",
        if args.json {
            lottery.parse_to_json().unwrap()
        } else {
            lottery.to_string()
        }
    );
}
