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

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0 ;

/// Takes a duration and returns the number of earth years in that duration
fn num_of_earth_years(d: &Duration) -> f64 {
    d.seconds as f64 / EARTH_YEAR_IN_SECONDS
}

fn num_of_years_wrt_orb_factor(d: &Duration, orb_factor: f64) -> f64 {
    num_of_earth_years(d) / orb_factor
}

#[macro_export]
macro_rules! planet {
    ( $($name:ident - $orb_period:expr), +) => {
        $(
        pub struct $name;
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
             num_of_years_wrt_orb_factor(d,  $orb_period)
    }
        })*
    };
}

planet!(
    Mercury - 0.2408467,
    Venus - 0.61519726,
    Earth - 1.0,
    Mars - 1.8808158,
    Jupiter - 11.862615,
    Saturn - 29.447498,
    Uranus - 84.016846,
    Neptune - 164.79132
);

