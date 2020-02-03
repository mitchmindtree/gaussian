extern crate gaussian;
extern crate rand;

use rand::Rng;

#[test]
pub fn gen() {
    let mut rng = rand::thread_rng();
    for _ in 0..1_000_000 {
        let f: f64 = rng.gen();
        let r: f64 = rng.gen();
        let v = gaussian::gen(&mut rng, f, r);
        assert!(v >= 0.0 && v < 1.0);
    }
}
