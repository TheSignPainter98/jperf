use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	/// Application name
	#[arg(long)]
	app: String,
	
	/// Most recent time interval in milliseconds
	#[arg(short, long, default_value_t = 1000)]
	interval: u32,
}

