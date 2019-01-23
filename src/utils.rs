use crate::game::Coordinate;
use cfg_if::cfg_if;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn coordinate_lerp(start: Coordinate, end: Coordinate, ratio: f64) -> Coordinate {
    if ratio == 0f64 {
        return start;
    }
    if ratio == 1f64 {
        return end;
    }
    let dx = end.x() - start.x();
    let dy = end.y() - start.y();
    Coordinate(start.x() + ratio * dx, start.y() + ratio * dy)
}

pub fn check_collision(coordinateA: Coordinate, coordinateB: Coordinate) -> bool {
    false
}