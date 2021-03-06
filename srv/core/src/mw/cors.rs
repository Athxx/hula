use actix_cors::Cors;

fn _cors() {
	let cors = Cors::default()
		.allowed_origin("https://www.example.com")
		.allowed_methods(vec!["GET", "POST"])
		.allow_any_header()
		.max_age(3600);
	// return actix_cors::Cors::default()
	//     .allowed_origin("https://www.rust-lang.org/")
	//     .allowed_origin_fn(|origin, _req_head| {
	//         origin.as_bytes().ends_with(b".rust-lang.org")
	//     })
	//     .allowed_methods(vec!["GET", "POST"])
	//     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
	//     .allowed_header(http::header::CONTENT_TYPE)
	//     .max_age(3600);
}
