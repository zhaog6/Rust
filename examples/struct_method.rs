/***********************************************************************
	> File Name: struct_method.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/03 星期四 08时28分45秒
 ***********************************************************************/

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

#[derive(Debug)]
struct Ball {
	radius: u32
}

impl Rectangle {
	// define struct method (with the "&self" parameter)
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn wider(&self, rect: &Rectangle) -> bool {
		self.width > rect.width
	}
}

impl Ball {
	// define struct correlation function (without the "&self" parameter)
	fn create(radius: u32) -> Ball {
		Ball {radius}
	}
}

fn main()
{
	let rect = Rectangle {width: 20, height: 50};
	println!("rect: {:?}", rect);
	println!("");
	println!("rect: {:#?}", rect);
	println!("");
	println!("Area of the Rectangle: {}", rect.area());

	let rect2 = Rectangle {width: 60, height: 12};
	println!("If wider?: {}", rect.wider(&rect2));  //结构体方法的调用
	println!("");

	let ball = Ball::create(10);  //结构体函数的调用
	println!("ball: {:#?}", ball);
	println!("");

	// 单元结构体 (Unit Struct)
	#[derive(Debug)]
	struct UnitStruct;
	let unit = UnitStruct;
	println!("unit struct: {:?}", unit);
}
