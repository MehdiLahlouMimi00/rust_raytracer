use rand::prelude::*;

pub fn random() -> f64
{
    let mut rng = rand::thread_rng();
    return rng.gen();
}


pub fn random_between(min : f64, max : f64) -> f64
{
    return min + (max - min)*random();
}