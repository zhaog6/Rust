/***********************************************************************
	> File Name: vector.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/08 星期二 15时28分48秒
 ***********************************************************************/

fn main()
{
	// 创建空向量u
	let u: Vec<i32> = Vec::new();
	println!("Length of u: {}\n", u.len());

	// 通过数组创建
	let mut u = vec![1, 2, 4];
	u.push(8);
	u.push(10);
	println!("u.len() = {}\nu = {:?}\n", u.len(), u);

	let mut v = vec![3, 5, 7, 9];
	u.append(&mut v);
	println!("After append: u = {:?}", u);
	println!("After append: v.len() = {}, v = {:?}\n", v.len(), v);

	println!("{}\n", match u.get(9) {
		Some(val) => val.to_string(),
		None => "None".to_string()
	});

	for val in &u { //if remain u, need to &u rather than u due to the implicit call to ".into_iter()"
		println!("{}", val);
	}
	println!();

	for val in &mut u {
		*val += 20;
	}
	println!("u.len() = {}\nu = {:?}", u.len(), u);
}
