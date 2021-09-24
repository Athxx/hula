mod model;
mod user_info;

use rbatis::core::db::DBPoolOptions;
use rbatis::rbatis::Rbatis;

// 对于常量，应当统一放置
// 下一篇重构中，我们再讨论不同的方式
pub const MYSQL_URL: &'static str = "mysql://root:mysql@localhost:3306/budshome";

pub async fn db_pool() -> Rbatis {
	let rb = Rbatis::new();

	let mut opts = DBPoolOptions::new();
	opts.max_connections = 100;

	rb.link_opt(MYSQL_URL, &opts).await.unwrap();

	rb
}
