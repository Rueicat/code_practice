//解析時間的crate

use time::{Duration,PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga = Duration::seconds(1_000_000_000);
    start + giga
}


fn main() {
}

