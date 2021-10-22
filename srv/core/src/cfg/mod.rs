use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref HTTP_PORT: String = env::var("ADDRESS").expect("Expected ADDRESS to be set in env!");
    pub static ref REDIS: String = env::var("ADDRESS").expect("Expected ADDRESS to be set in env!");
    pub static ref EMAIL: String = env::var("ADDRESS").expect("Expected ADDRESS to be set in env!");
}

pub struct EmailCfg {
	ak: String,
	sk: String,
	smtp: String,
	port: String,
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
	addr: String,
	auth: String,
	name: String,
	show_log: bool,
	lifetime: i8,
	open_conns: i8,
	idle_conns: i8,
}

pub struct Map {
	google_ak: String,
	google_sk: String,
	amap_ak: String,
	amap_sk: String,
	baidu_ak: String,
	baidu_sk: String,
}
