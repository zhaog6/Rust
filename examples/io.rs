/***********************************************************************
	> File Name: io.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/08 星期二 12时15分59秒
 ***********************************************************************/

use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	// 读取命令行参数
	let args = std::env::args();
	println!("{:?}", args);
	for arg in args {
		println!("{}", arg);
	}
	println!();

	// 文件读取（一次性读取）
    let f1 = std::fs::read_to_string("hello.txt").unwrap();
	println!("{}", f1);

	let f2 = std::fs::read("hello.txt").unwrap();
	println!("{:?}", f2);

	// 文件读取（流式读取）
	let mut buffer = [0; 5];
	let mut file = std::fs::File::open("hello.txt").unwrap();
	file.read(&mut buffer).unwrap();
	println!("{:?}", buffer);
	file.read(&mut buffer).unwrap();
	println!("{:?}", buffer);

	// 文件写入（一次性写入）
	//std::fs::write("log.txt", "Rust Programming\n").unwrap();

	// 文件写入（流式写入）
	let mut file = std::fs::File::create("log.txt").unwrap();
	file.write(b"FreeFEM is a FEM software\n").unwrap();
	file.write(b"HPDDM is a linear solver based on DDM\n").unwrap();

	// 追加写入
	let mut file = std::fs::OpenOptions::new().append(true).open("log.txt")?;  //也可加.unwrap()
	file.write(b"APPEND: PETSc & SLEPc\n")?;  //也可加.unwrap()

	Ok(())
}
