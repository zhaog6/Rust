/***********************************************************************
	> File Name: module.rs
	> Author: Zhao Gang
	> Mail: zhaog6@lsec.cc.ac.cn
	> Created Time: 2020/12/04 星期五 08时47分19秒
 ***********************************************************************/

mod nation {
	pub mod government {
		pub fn govern() { println!("government -> govern"); }
	}
	pub mod congress {
		pub fn legislate() { println!("congress -> legislate"); }
	}
	pub mod court {
		pub fn judicial() {
			super::government::govern();
			println!("court -> judicial");
		}
	}
}

fn main()
{
	nation::government::govern();
	nation::congress::legislate();
	nation::court::judicial();
}
