/***********************************************************************
	> File Name: map.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/08 星期二 18时12分57秒
 ***********************************************************************/

use std::collections::HashMap;

fn main()
{
	// insert方法和get方法是映射表最常用的两种方法
	let mut map = HashMap::new();
	map.insert("color", "red");
	map.insert("size", "10");
	println!("{}\n", map.get("color").unwrap());
	for ele in map.iter() {
		println!("{:?}", ele);
	}
	println!();

	map.entry("direction").or_insert("flexible");
	map.entry("color").or_insert("blue");
	for ele in map.iter() {
		println!("{:?}", ele);
	}
	println!();

	if let Some(x) = map.get_mut("size") {
		*x = "16";
	}
	for ele in map.iter() {
		println!("{:?}", ele);
	}
}
