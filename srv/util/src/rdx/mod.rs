use redis::{Commands, pipe};
use redis::cluster::ClusterClient;

pub fn init(nodes: Vec<String>) -> Result<T, E> {
	let cli = ClusterClient::open(nodes).unwrap();
	let mut conn = cli.get_connection().unwrap();
	return conn;
}

pub fn get(_key: String) {}

pub fn set(_key: String, _val: OxObject) {}

pub fn setex() {}

pub fn setnx() {}

pub fn keys(_key: String) {}

pub fn del(_key: String) {}

pub fn hset(_key: String, _field: String, _val: OxObject) {}

pub fn hget(_key: String, _field: String) {}
