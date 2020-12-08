/***********************************************************************
	> File Name: generics.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/08 星期二 11时32分13秒
 ***********************************************************************/

struct Book<T> {
	x: T,
	y: T
}

impl<T> Book<T> {
	fn output(&self, object: T) -> T {
		object
	}
}

fn main()
{
	let book = Book {x: 1, y: 2};
	println!("# of books: {}", book.output(100));
}
