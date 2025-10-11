pub type Value = f64;

pub fn print_value(value: &f64) {
    if *value == 0.0 {
        print!("0");
        return;
    }

    if value.abs() >= 1e6 || value.abs() < 1e-4 {
        // Use scientific notation for very large/small numbers (like C's %g)
        print!("{:.6e}", value);
    } else {
        // Regular fixed-point with trimming
        let s = format!("{:.6}", value);
        let trimmed = s.trim_end_matches('0').trim_end_matches('.');
        print!("{trimmed}");
    }
}
