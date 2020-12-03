/***********************************************************************
	> File Name: borrow.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/02 星期三 15时49分45秒
 ***********************************************************************/

fn main()
{
	/*
	let s1 = String::from("hello");
	let mut s2 = &s1;
	let s3 = s1;
	s2 = &s3;
	println!("s2: {}, world!", s2);
	println!("s3: {}, world!", s3);
	*/

	let mut s1 = String::from("hello");
	let s2 = &mut s1;
	s2.push_str(", world!");
	println!("s2: {}", s2);
	println!("s1: {}", s1);
}
