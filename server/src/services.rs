use crate::db::PrismaClient;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCommandRequest {
	name: String,
	action: String,
}

#[post("/api/v1/commands")]
pub async fn receive_command(client: web::Data<PrismaClient>, body: web::Json<CreateCommandRequest>) -> impl Responder {
	let command = client.command().create(body.name.to_owned(), body.action.to_owned(), vec![]).exec().await.unwrap();

	HttpResponse::Ok().json(command)
}

#[get("/api/v1/commands")]
pub async fn send_command(client: web::Data<PrismaClient>) -> impl Responder {
	let commands = client.command().find_many(vec![]).exec().await.unwrap();

	HttpResponse::Ok().json(commands)
}
