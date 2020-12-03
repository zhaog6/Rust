/***********************************************************************
	> File Name: option_enum.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/03 星期四 15时11分34秒
 ***********************************************************************/

fn main()
{
	let opt = Some("Hello");  //or Option::Some("Hello")
	//let opt: Option<&str> = None;  //or Option::None
	match opt {
		Some(something) => {
			println!("{}", something);
		},
		None => {
			println!("opt is nothing");
		}
	}
}
