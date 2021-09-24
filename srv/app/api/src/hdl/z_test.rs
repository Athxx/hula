async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}
