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
    fn years_during(d: &Duration) -> f64;
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



/// Takes a duration and returns the number of earth years in that duration
fn num_of_earth_years(d: &Duration) -> f64 {
    d.seconds as f64 / EARTH_YEAR_IN_SECONDS
}

fn num_of_years_wrt_orb_factor(d: &Duration, orb_factor: f64) -> f64 {
    num_of_earth_years(d) / orb_factor
}

const MERCURY_ORBITAL_FACTOR: f64 = 0.2408467;
impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        num_of_years_wrt_orb_factor(d, MERCURY_ORBITAL_FACTOR)
    }

}
const VENUS_ORBITAL_FACTOR: f64 = 0.61519726;
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        num_of_years_wrt_orb_factor(d, VENUS_ORBITAL_FACTOR)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        num_of_earth_years(d)
    }
}

const MARS_ORBITAL_FACTOR: f64 = 1.8808158;
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        num_of_years_wrt_orb_factor(d, MARS_ORBITAL_FACTOR)
    }
}

const JUPITER_ORBITAL_FACTOR: f64 = 11.862615;
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        num_of_years_wrt_orb_factor(d, JUPITER_ORBITAL_FACTOR)
    }
}
const SATURN_ORBITAL_FACTOR: f64 = 29.447498;
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        num_of_years_wrt_orb_factor(d, SATURN_ORBITAL_FACTOR)
    }
}
const URANUS_ORBITAL_FACTOR: f64 = 84.016846;
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        num_of_years_wrt_orb_factor(d, URANUS_ORBITAL_FACTOR)
    }
}
const NEPTUNE_ORBITAL_FACTOR: f64 = 164.79132;
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        num_of_years_wrt_orb_factor(d, NEPTUNE_ORBITAL_FACTOR)
    }
}
