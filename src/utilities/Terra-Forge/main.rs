mod planetSphere;

fn generate_voxel_terrain(num_voxels: usize) -> Vec<((f64, f64, f64), (f64, f64, f64))> {
    let sphere_radius = 100.0; // Example radius of the sphere
    let voxel_size = 10.0; // Example size of each voxel

    let mut voxels = Vec::new();

    // Generate voxels on the surface of the sphere
    for i in 0..num_voxels {
        let latitude = i as f64 * 360.0 / num_voxels as f64;
        for j in 0..num_voxels {
            let longitude = j as f64 * 360.0 / num_voxels as f64;

            // Generate opposite corners of the voxel
            let corner1 = planetSphere::generate_surface_point(latitude, longitude);
            let corner2 = planetSphere::generate_surface_point(latitude + voxel_size, longitude + voxel_size);

            voxels.push((corner1, corner2));
        }
    }

    voxels
}

fn main() {
    let num_voxels = 10;
    let voxel_terrain = generate_voxel_terrain(num_voxels);

    // Print or process voxel terrain data
    for (i, ((x1, y1, z1), (x2, y2, z2))) in voxel_terrain.iter().enumerate() {
        println!("Voxel {}: Corner 1: ({}, {}, {}), Corner 2: ({}, {}, {})", i+1, x1, y1, z1, x2, y2, z2);
    }
}