use raytracer::{Canvas, Scene, Sphere, Light, LightKind};
use nalgebra::{Vector3};
use image::Rgb;

fn main() {
    let mut canvas = Canvas::new(1024, 1024);

    let s1 = Sphere {
        center: Vector3::new(0.0, -1.0, 3.0),
        radius: 1,
        color: Rgb([255, 0, 0]),
        shininess: Some(500),
    };
    let s2 = Sphere {
        center: Vector3::new(2.0, 0.0, 4.0),
        radius: 1,
        color: Rgb([0, 0, 255]),
        shininess: Some(500),
    };
    let s3 = Sphere {
        center: Vector3::new(-2.0, 0.0, 4.0),
        radius: 1,
        color: Rgb([0, 255, 0]),
        shininess: Some(10),
    };
    let s4 = Sphere {
        center: Vector3::new(0.0, -5001.0, 0.0),
        radius: 5000,
        color: Rgb([255, 255, 0]),
        shininess: Some(1000),
    };

    let l1 = Light {
        intensity: 0.2,
        kind: LightKind::Ambient,
    };
    let l2 = Light {
        intensity: 0.6,
        kind: LightKind::Point(Vector3::new(2.0, 1.0, 0.0)),
    };
    let l3 = Light {
        intensity: 0.2,
        kind: LightKind::Directional(Vector3::new(1.0, 4.0, 4.0)),
    };


    let scene = Scene {
        objects: vec![s1, s2, s3, s4],
        lights: vec![l1, l2, l3],
    };
    
    raytracer::render(&mut canvas, &scene);
}
