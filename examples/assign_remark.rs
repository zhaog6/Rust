/***********************************************************************
	> File Name: assign_remark.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/02 星期三 09时45分30秒
 ***********************************************************************/

fn main()
{
	// YES
	let _x = 1;
	let _x = 10;
	println!("_x = {}", _x);

	/*
	// NO
	let x = 1;
	let x = 10;
	println!("x = {}", x);
	*/

	// YES
	let mut a = 10;
	a *= 8;
	println!("a = {}", a);

	/*
	// NO
	let mut b = 10;
	b = 8;
	println!("b = {}", b);
	*/

	// YES
	let mut _b = 10;
	_b = 8;
	println!("_b = {}", _b);
}
