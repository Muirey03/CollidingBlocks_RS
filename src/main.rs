use std::env;
use decimal::d128;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n = args[1].parse::<i32>().unwrap();

    let m1 = d128!(1.0);
    let m2 = d128!(100.0).pow(d128::from(n)) * m1;

    let mut v1 = d128!(0.0);
    let mut v2 = d128!(-1.0);

    let mut collisions = 0;

    loop {
        if v2 >= v1 && v1 >= d128!(0.0) {
            //Impossible to collide again - terminate
            break;
        }

        if v1 >= d128!(0.0) {
            //CoLM + CoE:
            let new_v1 = (d128!(2.0)*m2*v2 + m1*v1 - m2 * v1)/(m1 + m2);
            let new_v2 = (d128!(2.0)*m1*v1 + m2*v2 - m1 * v2)/(m1 + m2);
            v1 = new_v1;
            v2 = new_v2;
        } else {
            v1 = -v1;
        }
        collisions += 1;
    }

    println!("Collisions: {}", collisions);
}
