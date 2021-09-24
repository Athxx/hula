use lazy_static::lazy_static;
use dotenv::dotenv;

lazy_static! {
	dotenv().ok();
	pub static ref HTTP_PORT : String = dotenv::var("ADDRESS").expect("Expected ADDRESS to be set in env!");
	pub static ref REDIS : String = dotenv::var("ADDRESS").expect("Expected ADDRESS to be set in env!");
	pub static ref EMAIL : Email  = dotenv::var("ADDRESS").expect("Expected ADDRESS to be set in env!");


}





pub struct EmailCfg {
	ak: str,
	sk: str,
	smtp: str,
	port: str,
}

pub struct SMS {
	url: String,
	ak: String,
	sk: String,
}

pub struct Oss {
	ak: String,
	sk: String,
	endpoint: String,
	bucket: String,
	ttl: i8,
}

pub struct Database {
	addr: str,
	auth: str,
	name: str,
	show_log: bool,
	lifetime: i8,
	open_conns: i8,
	idle_conns: i8,
}

pub struct Map {
	google_ak: str,
	google_sk: str,
	amap_ak: str,
	amap_sk: str,
	baidu_ak: str,
	baidu_sk: str,
}

