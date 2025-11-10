use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to serve
    #[arg(short, long)]
    dir: String,
}

fn main() {
    let args = Args::parse();

    let root_dir = args.dir;

    feed_generator::generate_feeds(root_dir.as_str());
    server::serve(root_dir.as_str());
}

mod feed_generator;
mod network;
mod server;
