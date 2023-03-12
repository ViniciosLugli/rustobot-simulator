extern crate log;

use chrono::Local;
use dotenvy::dotenv;
use env_logger::Builder;
use log::LevelFilter;
use std::io::{self, Write};

#[actix_web::main]
async fn main() -> io::Result<()> {
	dotenv().ok();

	Builderd::new()
		.format(|buf, record| {
			writeln!(buf, "{} [{}] - {}", Local::now().format("%H:%M:%S |"), record.level(), record.args())
		})
		.filter(None, LevelFilter::Info)
		.init();

	Ok(())
}
