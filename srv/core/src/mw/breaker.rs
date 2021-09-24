use std::thread;

fn _breaker() {}

// 进行断流检测
fn _breaker_gc() {
	loop {
		thread::sleep_ms(2000);
	}
}
