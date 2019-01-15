
extern crate test_beizer;

use test_beizer::{UnitBezier,division};

fn main() {
    let b = UnitBezier::new(0.0,0.0,1.0,1.0);
    let zl = 1.0 / 100.0;
    let mut t = 0.0;

    let mut last_y = 0.0;

    for n in 0..100 {
        let n_t = division(t,0.0,1.0,0.0001,|t_| { b.sample_curve_x(t_) } );
        let y = b.solve( n_t );
        println!("{} {} {} ",n_t, t, y - last_y );
        last_y = y;
        t += zl;
    }
}