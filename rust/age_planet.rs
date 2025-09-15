// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {seconds: s}
    }
}

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

//每個星球都要規範trait, 預設有一個方法
pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / EARTH_YEAR_SECONDS / Self::ORBITAL_PERIOD
    }
}

pub struct Mercury;
impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.240_846_7;
}

pub struct Venus;
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.615_197_26;
}

pub struct Earth;
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}

pub struct Mars;
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.880_815_8;
}
pub struct Jupiter;
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862_615;
}
pub struct Saturn;
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447_498;
}
pub struct Uranus;
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016_846;
}
pub struct Neptune;
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.791_32;
}

//用Macro方法
macro_rules! planet {
    ($name:ident, $period:expr) => {
        pub struct $name;
        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

//example
planet!(Mercury, 0.240_846_7);
//and so on






fn main() {}
