/***********************************************************************
	> File Name: slice-string.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/02 星期三 16时27分48秒
 ***********************************************************************/

fn main()
{
	let s = String::from("broadcast");
	let part1 = &s[0..5];
	let part2 = &s[5..];
	println!("{} = {} + {}", s, part1, part2);

	let st = "helloworld";
	let st_part1 = &st[0..5];
	let st_part2 = &st[5..];
	println!("{} = {} + {}", st, st_part1, st_part2);
}
