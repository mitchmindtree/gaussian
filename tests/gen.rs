
extern crate gaussian;
extern crate rand;

#[test]
pub fn gen() {
    for _ in 0..1_000_000 {
        let v = gaussian::gen(::rand::random::<f32>(), ::rand::random::<f32>());
        assert!(v >= 0.0 && v < 1.0);
    }
}

#[test]
pub fn gen_map() {
    for _ in 0..1_000_000 {
        let v = gaussian::gen_map(::rand::random::<f32>(), ::rand::random::<f32>(), -50.0, 100.0);
        assert!(v >= -50.0 && v < 100.0);
    }
}

