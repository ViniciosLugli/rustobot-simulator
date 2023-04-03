extern crate log;

mod db;
mod services;

use actix_web::{web, App, HttpServer};
use chrono::Local;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

use services::{receive_command, send_command};

use db::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	Builder::new()
		.format(|buf, record| {
			writeln!(buf, "{} [{}] - {}", Local::now().format("%H:%M:%S |"), record.level(), record.args())
		})
		.filter(None, LevelFilter::Info)
		.init();

	println!("Starting server at http://{}:{}", dotenv!("HOST"), dotenv!("PORT"));

	let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

	HttpServer::new(move || App::new().app_data(client.clone()).service(receive_command).service(send_command))
		.bind((dotenv!("HOST"), dotenv!("PORT").parse().unwrap()))?
		.workers(dotenv!("WORKERS").parse().unwrap())
		.run()
		.await
}
