/***********************************************************************
	> File Name: trait.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/04 星期五 09时38分48秒
 ***********************************************************************/

trait Area {
	fn area(&self) -> f64;
}

struct Circle {
	r: f64
}

impl Area for Circle {
	fn area(&self) -> f64 {
		3.1415926 * self.r
	}
}

fn main()
{
	let r = Circle {r: 10.5};
	println!("area = {}", r.area());
}
