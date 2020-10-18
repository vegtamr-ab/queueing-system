mod types;
use types::*;

fn get_student_value(cl: ConfidenceLevel) -> f64 {
    match cl {
        ConfidenceLevel::Low => 1.6649,
        ConfidenceLevel::Medium => 1.96,
        ConfidenceLevel::High => 2.5758,
        ConfidenceLevel::VeryHigh => 3.2905,
    }
}

fn number_of_entries(cl: ConfidenceLevel, p: f64) -> i32 {
    let delta = (cl as i32) as f64 / 1000.0;
    let st_value = get_student_value(cl);
    ((st_value * st_value * (1.0 - p)) / (p * delta * delta)) as i32
}

fn deny_probability(cl: ConfidenceLevel, n: i32) -> f64 {
    let delta = (cl as i32) as f64 / 1000.0;
    let st_value = get_student_value(cl);
    (st_value * st_value) / (n as f64 * delta * delta + st_value * st_value)
}

fn get_ideal_n(cl: ConfidenceLevel, n: i32, mut pd: Option<f64>) -> i32 {
    /* let p = match pd {
        Some(a) => a,
        None => SIMULATE CL N
    } */
    /* let nx = number_of_entries(cl, p); */
    /* let px = SIMULATE CL NX */
    /* if (px - p).abs() < (p * 0.1) {
        get_ideal_n(cl, nx, Some(px))
    } else {
        n
    } */
    0
}