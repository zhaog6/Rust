/***********************************************************************
	> File Name: loop.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/02 星期三 13时21分20秒
 ***********************************************************************/

fn main()
{
	let a = [1, 2, 3, 4, 5];
	for i in 0..=4
	{
		println!("a[{}] = {}", i, a[i]);
	}
	let mut i = 0;
	let val = loop {
		if i == 4
		{
			println!("a > 100 !");
			break a[i];
		}
		i += 1;
	};
	println!("val = {}", val);
}
