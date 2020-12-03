/***********************************************************************
	> File Name: slice-array.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/03 星期四 09时09分25秒
 ***********************************************************************/

fn main()
{
	let a = [1, 2, 3, 4, 5];
	let part = &a[2..];
	for i in part.iter() {
		println!("{}", i);
	}
}
