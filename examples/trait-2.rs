/***********************************************************************
	> File Name: trait-2.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/08 星期二 09时08分02秒
 ***********************************************************************/

trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

trait Character {
	fn character(&self) -> String {
		String::from("[Character]")
	}
}

struct Person {
    name: String,
    age: u8
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

impl Character for Person {
	fn character(&self) -> String {
		format!("character: {} {}", self.name, self.name)
	}
}

/*
fn marvel(object: impl Descriptive + Character) -> i32 {
	let a = (object.describe()).len() + (object.character()).len();
	return a as i32;
}
*/
/*
fn marvel<T: Descriptive + Character>(object: T) -> i32 {
	let a = (object.describe()).len() + (object.character()).len();
	return a as i32;
}
*/
fn marvel<T>(object: T) -> i32
where T: Descriptive + Character
{
	let a = (object.describe()).len() + (object.character()).len();
	return a as i32;
}

fn main()
{
    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());
	println!("marvel(object): {}", marvel(cali));
}
