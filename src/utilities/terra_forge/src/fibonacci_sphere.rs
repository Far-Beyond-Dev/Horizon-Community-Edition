use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Instant;
use std::sync::Mutex;
use std::thread;

fn fibonacci_point(numsamples: usize, samplev: usize, seed: f64, min_lat: f64, max_lat: f64, min_lon: f64, max_lon: f64) -> (f64, f64, f64) {
    let fib_incr = PI * (3.0 - (5.0 as f64).sqrt());
    let numsamplesf = numsamples as f64;
    let rnd = seed * numsamplesf;

    let lat_offset = (max_lat - min_lat) / numsamplesf;
    let lon_range = max_lon - min_lon;
    let samplef = samplev as f64;

    let latitude = min_lat + (samplef + 0.5) * lat_offset;
    let phi = ((samplef + rnd) % numsamplesf) * fib_incr;
    let longitude = min_lon + (phi % lon_range + lon_range) % lon_range;

    let z = latitude.cos();
    let r = (1.0 - z * z).sqrt();
    let phi_corrected = longitude * PI / 180.0;

    let x = phi_corrected.cos() * r;
    let y = phi_corrected.sin() * r;

    (x * 1000.0, y * 1000.0, z * 1000.0)
}

pub fn generate_fibonacci_sphere(num_samples: usize, min_latitude: f64, max_latitude: f64, min_longitude: f64, max_longitude: f64, seed: f64) -> io::Result<Vec<(f64, f64, f64)>> {
    let start_time = Instant::now();

    let (tx, rx) = channel();
    let num_threads = 32;
    let samples_per_thread = num_samples / num_threads;
    let samples_leftover = num_samples % num_threads;

    let tx = Arc::new(Mutex::new(tx));
    let mut handles = vec![];

    for i in 0..num_threads {
        let tx = Arc::clone(&tx);
        let start = i * samples_per_thread;
        let end = if i == num_threads - 1 {
            start + samples_per_thread + samples_leftover
        } else {
            start + samples_per_thread
        };

        let handle = thread::spawn(move || {
            let mut thread_points = vec![];
            for samplev in start..end {
                let point = fibonacci_point(num_samples, samplev, seed, min_latitude, max_latitude, min_longitude, max_longitude);
                thread_points.push(point);
            }
            let mut tx = tx.lock().unwrap();
            for point in thread_points {
                tx.send(point).unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx);

    let mut points = vec![];
    for point in rx {
        points.push(point);
    }

    let duration = start_time.elapsed();

    let mut file = File::create("output.txt")?;
    writeln!(file, "Generated points:")?;
    writeln!(file, "Time elapsed: {:?}", duration)?;
    for point in &points {
        writeln!(file, "{:?}", point)?;
    }

    Ok(points)
}
