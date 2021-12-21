extern crate time;

fn main() {
    let	d = time::now();
	println!("Today is {}/{}/{}", d.tm_mday,
	        d.tm_mon + 1, d.tm_year + 1900); //fix do mês

    //https://docs.rs/time/0.1.37/time/struct.Tm.html
    //tm_mon: i32 - Months since January - [0, 11]
}
