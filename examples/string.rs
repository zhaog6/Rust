/***********************************************************************
	> File Name: string.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/08 星期二 17时07分14秒
 ***********************************************************************/

fn main()
{
	// 将基本数据类型转换为字符串
	let one = 1.to_string();
	let float = 1.5.to_string();
	let slice = "slice".to_string();
	println!("one = {},\nfloat = {},\nslice = {}\n", one, float, slice);

	// 支持UTF-8中文字符串输出
	let hello = String::from("你好！");
	println!("hello -> {}\n", hello);

	// 追加字符串和字符
	let mut hello = String::from("FreeFEM ");
	hello.push_str("HPDDM PETS");
	hello.push('c');
	println!("{}", hello);

	// 合并字符串
	let mut s1 = String::from("FreeFEM ");
	let s2 = String::from("HPDDM PETSc 数值计算软件");
	s1 += &s2;
	println!("s1.len() = {}", s1.len()); //注：比s1.chars().count()快得多
	println!("{}", s1);
	println!("{}\n", s2);

	// 遍历字符串
	for c in s1.chars() {
		println!("{}", c);
	}
	println!();

	// 从字符串中取出单个字符（如果是UTF-8字符串，不要在遍历中使用，因为UTF-8每个字符的长度不一定相等！）
	println!("{:?}\n", s1.chars().nth(20));

	// 截取字符子串
	let mut substr = &s1[0..20];
	println!("Substring 1: {}", substr);
	substr = &s1[20..];
	println!("Substring 2: {}", substr);
	/* 注：运行错误，下面操作可能会分解一个UTF-8字符
	substr = &s1[18..21];
	println!("Substring 3: {}", substr);
	*/
}
