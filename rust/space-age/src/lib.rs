// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration{seconds}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0 ;
const MERCURY_ORBITAL_FACTOR: f64 = 0.2408467;

const VENUS_ORBITAL_FACTOR: f64 = 0.61519726;

/// Takes a duration and returns the number of earth years in that duration
fn num_of_earth_years(d: &Duration) -> f64 {
    d.seconds as f64 / EARTH_YEAR_IN_SECONDS
}

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        num_of_earth_years(d) / MERCURY_ORBITAL_FACTOR
    }

}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        num_of_earth_years(d) / VENUS_ORBITAL_FACTOR
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        num_of_earth_years(d)
    }
}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
