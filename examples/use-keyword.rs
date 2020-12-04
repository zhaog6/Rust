/***********************************************************************
	> File Name: use-keyword.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/04 星期五 10时51分30秒
 ***********************************************************************/

use std::f64::consts::PI;

mod nation {
	pub mod government {
		pub fn govern() { println!("government -> govern"); }
	}
	pub fn govern() { println!("nation -> govern"); }
}

use crate::nation::government::govern;
use crate::nation::govern as nation_govern;

fn main()
{
	govern();
	nation_govern();
	println!("");
	println!("sin(pi/2) = {}", (PI / 2.).sin());
}
