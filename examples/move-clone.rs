/***********************************************************************
	> File Name: move-clone.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/02 星期三 14时35分27秒
 ***********************************************************************/

fn main()
{
	let s1 = String::from("hello");
	let mut s2 = s1.clone();
	println!("s1: {}, world!", s1);
	println!("s2: {}, world!", s2);
	println!("-------------------");
	s2 = s1;
	//println!("s1: {}, world!", s1); //Error: s1 is ineffective
	println!("s2: {}, world!", s2);
}
