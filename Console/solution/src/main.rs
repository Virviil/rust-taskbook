extern crate time;

fn main() {
    task1();
    task2();
    task3();
    task4();
    task5();
    task6();
}

fn task1() {
    println!("{}", "Silence is golden");
}

fn task2(){
    println!("{}", "January");
    println!("{}", "Sunday");
    println!("{}", "Jack");
}

#[allow(dead_code)]
fn task2_universal() {
    let current_time = time::now();
    let month = match current_time.tm_mon {
        0=>"January",
        1=>"February",
        2=>"March",
        3=>"April",
        4=>"May",
        5=>"June",
        6=>"July",
        7=>"August",
        8=>"September",
        9=>"October",
        10=>"Nowember",
        11=>"December",
        _=>panic!("Unexpected month!")
    };
    let weekday = match current_time.tm_wday {
        0=>"Sunday",
        1=>"Monday",
        2=>"Tuesday",
        3=>"Wednesday",
        4=>"Thursday",
        5=>"Friday",
        6=>"Saturday",
        _=>panic!("Unexpected weekday!")
    };
    println!("{}", month);
    println!("{}", weekday);
    println!("{}", "Name");
}

fn task3(){
    println!("{}", "0");
    println!("{}", "00");
    println!("{}", "000");
    println!("{}", "0000");
    println!("{}", "00000");
}

fn task4(){
    println!("{}", "AAAAAAAA");
    println!("{}", "A      A");
    println!("{}", "A      A");
    println!("{}", "A      A");
    println!("{}", "AAAAAAAA");
}

fn task5(){
    println!("{}", "*     *     *");
    println!("{}", " *   * *   *");
    println!("{}", "  * *   * *");
    println!("{}", "   *     *");
}

fn task6(){
    println!("{}", 1+2-4);
}
