extern crate num;
extern crate rand;
extern crate utils;

/// Gen raw gaussian value with distribution mean at 0.
pub fn gen_raw<R>(mut rng: R) -> f64
    where R: rand::Rng,
{
    use rand::distributions::normal::StandardNormal;
    let StandardNormal(f) = rng.gen();
    f
}

/// Generates a raw gaussian value between [0.0, 1.0) whose distribution's mean is at `value` with
/// the given amount of `randomness` between [0.0, 1.0).
///
/// **Panic**s if the `value` is less than 0.0 or greater than or equal to 1.0.
///
/// **Panic**s if the `randomness` is less than 0.0 or greater than or equal to 1.0.
pub fn gen<R>(mut rng: R, value: f64, randomness: f64) -> f64
    where R: rand::Rng,
{
    assert!(value >= 0.0);
    assert!(value < 1.0);
    assert!(randomness >= 0.0);
    assert!(randomness <= 1.0);

    // If there is no randomness, return the value as it was given.
    if randomness == 0.0 {
        return value;
    }

    // If there is complete randomness, generate a uniform distribution value.
    if randomness == 1.0 {
        return rand::Rng::gen_range(&mut rng, 0.0, 1.0);
    }

    // Offset the value over the normal distributions
    let offset_value = value * 2.0 - 1.0;

    // Keep attempting values until we get one that falls within the range.
    loop {
        let normal = gen_raw(&mut rng);
        let attempt = normal * randomness + offset_value;
        if -1.0 <= attempt && attempt < 1.0 {
            return (attempt + 1.0) * 0.5;
        }
    }
}

/// Generate a gaussian value mapped to the given range [min, max).
pub fn gen_map_range<R, T>(rng: R, value: T, randomness: f64, min: T, max: T) -> T
    where R: rand::Rng,
          T: num::NumCast + Copy,
{
    let f = utils::map_range(value, min, max, 0.0, 1.0);
    let r = gen(rng, f, randomness);
    utils::map_range(r, 0.0, 1.0, min, max)
}
