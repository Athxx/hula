use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let schema = build_schema().await;

	println!("GraphQL UI: http://127.0.0.1:8080");

	HttpServer::new(move || {
		App::new()
			.data(schema.clone())
			.service(web::resource("/graphql").guard(guard::Post()).to(graphql))
			.service(web::resource("/graphiql").guard(guard::Get()).to(graphiql))
	})
		.bind("0.0.0.0:8080")?
		.run()
		.await
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
	async fn add(&self, a: i32, b: i32) -> i32 {
		a + b
	}

	async fn dec(&self, a: i32, b: i32) -> i32 {
		a - b
	}
}

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};

// `ActixSchema` 类型定义，项目中可以统一放置在一个共用文件中。
// 但和 `actix-web` 和 `tide` 框架不同，无需放入应用程序`状态（State）`
// 所以此 `Schema` 类型仅是为了代码清晰易读，使用位置并不多，我们直接和构建函数一起定义。
// 或者，不做此类型定义，直接作为构建函数的返回类型。
type ActixSchema = Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub async fn build_schema() -> ActixSchema {
	// The root object for the query and Mutatio, and use EmptySubscription.
	// Add global sql datasource  in the schema object.
	Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: Request) -> Response {
	schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Result<HttpResponse> {
	Ok(HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(playground_source(GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"))))
}
