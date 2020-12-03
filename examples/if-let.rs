/***********************************************************************
	> File Name: if-let.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/03 星期四 16时00分46秒
 ***********************************************************************/

fn main()
{
	let i = 0;
	// 利用 if let 语法
	if let 0 = i {
		println!("if-let: zero");
	} else if let 1 = i {
		println!("if-let: one");
	} else {
		println!("if-let: neither zero nor one");
	}

	// 利用match
	match i {
		0 => println!("match: zero"),
		1 => println!("match: one"),
		_ => {}
	}
}
