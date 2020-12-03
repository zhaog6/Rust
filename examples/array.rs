/***********************************************************************
	> File Name: array.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/02 星期三 10时24分41秒
 ***********************************************************************/

fn main()
{
	// Tuple
	let tup: (i32, f64, u8) = (100, 8.5, 12);
	let (a, b, c) = tup;
	println!("a = {}, b = {}, c = {}", a, b, c);

	// Array
	let a = [1, 2, 3, 4, 5];
	println!("a[0] = {}, a[1] = {}", a[0], a[1]);
	let c: [i32; 5] = [10, 20, 30, 40, 50];
	println!("c[0] = {}", c[0]);
	let d = [3; 5];
	let d_len = d.len();
	println!("The size of d = {}", d_len);

	let mut d = [1, 2, 3];
	d[0] = 4;
	println!("{}, {}, {}", d[0], d[1], d[2]);
}
