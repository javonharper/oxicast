use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let _args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
    //
    let root_dir = "/Users/javon/Developer/oxicast/example_root_dir/";

    feed_generator::generate_feeds(root_dir);
    server::serve(root_dir);
}

mod feed_generator;
mod server;
