extern crate gaussian;
extern crate rand;

use rand::Rng;

#[test]
pub fn gen() {
    let mut rng = rand::thread_rng();
    for _ in 0..1_000_000 {
        let f = rng.next_f64();
        let r = rng.next_f64();
        let v = gaussian::gen(&mut rng, f, r);
        assert!(v >= 0.0 && v < 1.0);
    }
}

#[test]
pub fn gen_map() {
    let mut rng = rand::thread_rng();
    for _ in 0..1_000_000 {
        let f = rng.next_f64();
        let r = rng.next_f64();
        let v = gaussian::gen_map_range(&mut rng, f, r, -50.0, 100.0);
        assert!(v >= -50.0 && v < 100.0);
    }
}

