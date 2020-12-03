/***********************************************************************
	> File Name: function.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/02 星期三 11时04分05秒
 ***********************************************************************/

fn add(a: i32, b: i32) -> i32
{
	return a + b;
}

fn main()
{
	let sum = add(5, 6);
	println!("a + b = {}", sum);

	let x = 10;
	let y = {
		let x = 100;
		x + 80
	};
	println!("x = {}, y = {}", x, y);
}
