use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use env_logger;
use api::hdl;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// todo read config
	let port: &str = "0.0.0.0:8080";

	env_logger::init();

	println!("Server running on {} ", port);
	HttpServer::new(move || App::new()
		.wrap(middleware::Logger::default())
		.wrap(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))
		.wrap(Cors::default()
			.allowed_methods(vec!["POST", "GET"])
			.allow_any_origin()
			.supports_credentials()
			.max_age(3600)
		)
		// *********************** graphql ***********************
		.route(
			"/api",
			web::to(hdl::demo),
		)
		// *********************** websocket ***********************
		.route(
			"/ws",
			web::get().to(hdl::demo),
		)
		// *********************** restful ***********************
		.route(
			"/ping",
			web::get().to(hdl::ping),
		)

		// 404 not found
		.default_service(web::to(hdl::not_found))
	) // Graceful shutdown
		.bind(port)?
		.run()
		.await
}
