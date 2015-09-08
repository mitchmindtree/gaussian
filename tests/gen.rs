
extern crate gaussian;

#[test]
pub fn gen() {
    for _ in 0..1_000_000 {
        let v = gaussian::gen(0.5, 1.0);
        assert!(v >= 0.0 && v < 1.0);
    }
}

#[test]
pub fn gen_map() {
    for _ in 0..1_000_000 {
        let v = gaussian::gen_map(0.5, 1.0, -50.0, 100.0);
        assert!(v >= -50.0 && v < 100.0);
    }
}

