/***********************************************************************
	> File Name: enum.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/03 星期四 13时48分25秒
 ***********************************************************************/

#[derive(Debug)]

enum Book {
	Papery {index: u32},
	Electronic {url: String}
}

fn main()
{
	let book = Book::Papery {index: 100};
	let ebook = Book::Electronic {url: String::from("url://www.freefem.org")};
	println!("{:?}", book);
	println!("{:?}", ebook);

	match book {
		Book::Papery {index} => {
			println!("Paper book {}", index);
		},
		Book::Electronic {url} => {
			println!("E-book {}", url);
		}
	}

	let a = 10;
	match a {
		10 => println!("It is ten"),
		_ => {}  //println!("It is not ten")
	}
}
