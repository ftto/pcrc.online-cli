mod api;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
}

fn main() {
    let args = Cli::from_args();

    match args.pattern.as_str() {
        "status" => {
            api::fetch_status();
        }
        _ => println!("nothing"),
    }
}
