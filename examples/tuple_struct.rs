/***********************************************************************
	> File Name: tuple_struct.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/03 星期四 10时13分18秒
 ***********************************************************************/

struct Color(u8, u8, u8);
struct Point(f64, f64, f64);

fn main()
{
	let red = Color(1, 0, 0);
	let origin = Point(0., 0., 0.);
	println!("red = ({}, {}, {})", red.0, red.1, red.2);
	println!("origin = ({}, {}, {})", origin.0, origin.1, origin.2);
}
