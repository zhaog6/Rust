/***********************************************************************
	> File Name: struct.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/03 星期四 09时21分59秒
 ***********************************************************************/

struct Site {
	domain: String,
	name: String,
	nation: String,
	found: u32
}

fn main()
{
	let	domain = String::from("www.freefem.org");
	let	name = String::from("FreeFEM");
	let freefem = Site {
		domain,  //be identificial to "domain: domain,"
		name,  //be identificial to "name: name,"
		nation: String::from("France"),
		found: 1987
	};
	println!("domain: {}", freefem.domain);
	println!("name: {}", freefem.name);
	println!("nation: {}", freefem.nation);
	println!("found: {}", freefem.found);
	println!("");

	let hpddm = Site {
		name: String::from("HPDDM"),
		found: 2013,
		..freefem
	};
	println!("domain: {}", hpddm.domain);
	println!("name: {}", hpddm.name);
	println!("nation: {}", hpddm.nation);
	println!("found: {}", hpddm.found);
}
