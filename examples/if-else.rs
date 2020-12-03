/***********************************************************************
	> File Name: if-else.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/02 星期三 11时52分31秒
 ***********************************************************************/

fn main()
{
	let a = 12;
	let b;
	if a > 10
	{
		b = 1;
	}
	else if a < 0
	{
		b = -1;
	}
	else
	{
		b = 0;
	}
	println!("a = {}, b = {}", a, b);
}
