extern crate time;

fn main() {
    mismatched_types();
    atribuicao_multipla();
    atribuicao_sem_inferencia_de_tipo();
    variaveis_mutaveis();
    constantes1();
    constantes2();
}

fn mismatched_types(){
    let	d = time::now();
	println!("Today is {}/{}/{}", d.tm_mday,
	    d.tm_mon + 1, d.tm_year + 1900);

    //d = 2; //mismatched types - ^ expected struct `Tm`, found integer
}

fn atribuicao_multipla(){
    let	d = time::now();
    let	(day, month, year) = (d.tm_mday, d.tm_mon + 1, d.tm_year + 1900);
    println!("Today is {}/{}/{}", day, month, year);
}

fn atribuicao_sem_inferencia_de_tipo(){
    let d = time::now();
    let day: i32 = d.tm_mday;
    let month: i32 = d.tm_mon + 1;
    let year: i32 = d.tm_year + 1900;
    println!("Today is {}/{}/{}", day, month, year);
}

fn variaveis_mutaveis(){
    //let a = 20;
    //a = 22; //cannot assign twice to immutable variable `a`

    let mut b = 20;
    println!("{}", b);
    b = 22;
    println!("{}", b);
}

fn constantes1(){
    const Y: i32 = 3;
    println!("Const: {}", Y);
}

fn constantes2(){
    const THE_1900: i32 = 1900;

    let d = time::now();
    let day: i32 = d.tm_mday;
    let month: i32 = d.tm_mon + 1;
    let year: i32 = d.tm_year + THE_1900;
    println!("Today is {}/{}/{}", day, month, year);
}