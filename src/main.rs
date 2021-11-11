use std::io::{self, Write};
//use std::env;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let args = std::env::args().nth(1);
    let mut a = args.expect("lkjh").parse::<i32>().ok().expect("jölkj");
    let (mut s, mut s2, mut m, mut mi) = (0,0,0,0);
//    let mut s2 = 0;
//    let mut m = 0;
//    let mi = 0;

    print!("{}\n", mi);

    while a > 0 {
        print!("#");
        s+=1;
        m+=1;
        s2+=1;
        sleep(Duration::from_millis(1000));
        a-=1;
        if s2 == 10 {
            print!(" ");
            s2 = 0;
        }
        if s == 30 {
            print!("\n ");
            s = 0;
        }
        if m == 60 {
            print!("\n\n");
            m = 0;
            mi+=1;
            print!("{}\n", mi);
        }
	io::stdout().flush().unwrap();
    }
    print!("\n");
}
