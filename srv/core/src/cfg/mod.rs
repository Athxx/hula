use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref HTTP_PORT: String = env::var("ADDRESS").expect("Expected ADDRESS to be set in env!");
    pub static ref REDIS: String = env::var("ADDRESS").expect("Expected ADDRESS to be set in env!");
    pub static ref EMAIL: String = env::var("ADDRESS").expect("Expected ADDRESS to be set in env!");
}

pub struct Base {
	pub mode: i8, // 0.prd  , 1.dev , 2.test
	pub mode_curl : bool,
}

pub struct Logger {
	pub level : String,
	pub stdout : bool,
	pub stderr : bool,
	pub format : String, // json, console
}

pub struct Email {
	pub ak: String,
	pub sk: String,
	pub smtp: String,
	pub port: String,
}

pub struct SMS {
	pub url: String,
	pub ak: String,
	pub sk: String,
}

pub struct Oss {
	pub ak: String,
	pub sk: String,
	pub endpoint: String,
	pub bucket: String,
	pub ttl: i8,
	pub thumbnail: String,
	pub callback_url : String
}

pub struct Database {
	pub addr: String,
	pub auth: String,
	pub name: String,
	pub show_log: bool,
	pub lifetime: i8,
	pub open_conns: i8,
	pub idle_conns: i8,
}

pub struct Map {
	pub google_ak: String,
	pub google_sk: String,
	pub amap_ak: String,
	pub amap_sk: String,
	pub baidu_ak: String,
	pub baidu_sk: String,
}
