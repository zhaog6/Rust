/***********************************************************************
	> File Name: thread.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/10 星期四 08时41分52秒
 ***********************************************************************/

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main()
{
	// spawn和join的基本用法
	let handle = thread::spawn(|| {
		for i in 0..5 {
			println!("spawned thread print {}", i);
			thread::sleep(Duration::from_millis(1));
		}

	});

	for i in 0..3 {
		println!("main thread print {}", i);
		thread::sleep(Duration::from_millis(1));
	}

	handle.join().unwrap();
	println!();

	// 使用闭包的move关键字
	let s = "hello";
	let handle = thread::spawn(move || {
		println!("{}", s);
	});
	handle.join().unwrap();
	println!();

	// 利用channel实现消息传递并发
	let (tx, rx) = mpsc::channel();
	thread::spawn(move || {
		let val = 1.5;
		tx.send(val).unwrap();
	});
	let recv = rx.recv().unwrap();
	println!("Get: {}", recv);
}
