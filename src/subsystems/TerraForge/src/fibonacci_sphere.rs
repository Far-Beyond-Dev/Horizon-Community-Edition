use std::f64::consts::PI;

pub fn generate_fibonacci_sphere(num_samples: usize, radius: f64) -> Vec<(f64, f64, f64)> {
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let angle_increment = std::f64::consts::PI * 2.0 * phi;

    (0..num_samples).map(|i| {
        let t = i as f64 / num_samples as f64;
        let angle1 = angle_increment * i as f64;
        let y = (1.0 - 2.0 * t) * radius;
        let r = (radius * radius - y * y).sqrt();
        let x = r * angle1.cos();
        let z = r * angle1.sin();
        (x, y, z)
    }).collect()
}