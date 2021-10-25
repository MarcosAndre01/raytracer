use image::{RgbImage, Rgb};
use nalgebra::{Vector3};
use std::f64::INFINITY;

/// Contains all objects and lights to be rendered.
pub struct Scene {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
}

/// A 3d spherical primitive.
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: u32,
    pub color: Rgb<u8>,
    pub shininess: Option<i32>,
}

/// Type of the light.
pub enum LightKind {
    /// Ambient light that illuminates all points in the scene.
    Ambient,
    
    /// A point of light located in a specified position.
    /// Sends rays of light in all directions.
    Point(Vector3<f64>),

    /// Various light rays hitting the objects in the specified direction.
    Directional(Vector3<f64>),
}

/// Light that illuminates the objects in the scene.
pub struct Light {
    pub kind: LightKind,
    pub intensity: f64,
}

/// Bidimensional grid of pixels that make the final image.
/// The central pixel is located at position (x: 0, y: 0).
pub struct Canvas {
    image: RgbImage,
}

// [-C/2, C/2)
impl Canvas {
    /// Creates a new Canvas with the specified resolution.
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas{image: RgbImage::new(width, height)}
    }
    /// Updates the pixel at position (x, y) of the canvas.
    pub fn put_pixel(&mut self, x: i32, y: i32, pixel: Rgb<u8>) {
        let x = (self.image.width() as i32)/2 + x;
        let y = (self.image.height() as i32)/2 - (y + 1);
        self.image.put_pixel(x as u32, y as u32, pixel);
    }

    /// The width of this canvas.
    pub fn width(&self) -> u32 {
        self.image.width()
    }

    /// The height of this canvas.
    pub fn height(&self) -> u32 {
        self.image.height()
    }
}

struct Viewport {
    width: u32,
    height: u32,
    distance: f64, // Distance from the camera
}

trait Scalable {
    fn scale(&self, scalar: f64) -> Self;
}

// Create own color type, then convert inside put_pixel ??
impl Scalable for Rgb<u8> {
    fn scale(&self, scalar: f64) -> Rgb<u8> {
        let mut new_color = [0u8; 3];

        for i in 0..3 {
            new_color[i] = (self[i] as f64 * scalar).min(255.0) as u8;
        }

        Rgb(new_color)
    }
}

/// Renders the scene and saves it to the output.png file.
pub fn render(canvas: &mut Canvas, scene: &Scene) {
    let origin = Vector3::new(0.0, 0.0, 0.0);
    let viewport = Viewport {
        width: 1,
        height: 1,
        distance: 1.0
    };
   
    //for x, y, _  in canvas.enumerate_pixels()
    let cw = canvas.width() as i32;
    let ch = canvas.height() as i32;
    for x in -cw/2..cw/2 {
        for y in -ch/2..ch/2 {
            let direction = canvas_to_viewport(x, y, &canvas, &viewport);
            let color = trace_ray(&scene, &origin, &direction, 1.0, INFINITY);
            canvas.put_pixel(x, y, color);
        }
    }

    canvas.image.save("output.png").unwrap();
}

fn canvas_to_viewport(x: i32, y: i32, canvas: &Canvas, viewport: &Viewport) -> Vector3<f64> {
    Vector3::new(
        x as f64 * viewport.width as f64 / canvas.width() as f64,
        y as f64 * viewport.height as f64 / canvas.height() as f64,
        viewport.distance
    )
}

fn trace_ray(scene: &Scene, origin: &Vector3<f64>, direction: &Vector3<f64>, t_min: f64, t_max: f64) -> Rgb<u8> {
    let mut closest_t = INFINITY;
    let mut closest_sphere = None;

    for primitive in &scene.objects {
        let (t1, t2) = intersect_ray_sphere(&origin, &direction, &primitive);

        for t in [t1, t2] {
            if (t > t_min && t < t_max) && t < closest_t {
                closest_t = t;
                closest_sphere = Some(primitive);
            }
        }
    }

    match closest_sphere {
        Some(sphere) => {
            let point = origin + direction.scale(closest_t);
            let normal = (point - sphere.center).normalize();
            sphere.color.scale(compute_lighting(&scene, &point, &normal, &(-direction), sphere.shininess))
        }
        None => Rgb([255, 255, 255])
    }
}


fn intersect_ray_sphere(origin: &Vector3<f64>, direction: &Vector3<f64>, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;
    let co = origin - sphere.center;
      
    let a = direction.dot(&direction);
    let b = 2.0 * co.dot(&direction);
    let c = co.dot(&co) - (r*r) as f64;

    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return (INFINITY, INFINITY);
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0*a);
    let t2 = (-b - discriminant.sqrt()) / (2.0*a);

    (t1, t2)
}

fn compute_lighting(
    scene: &Scene, point: &Vector3<f64>, normal: &Vector3<f64>,
    view: &Vector3<f64>, shininess: Option<i32>
) -> f64 {
    let mut illumination = 0.0;

    for light in &scene.lights {
        let point_to_light: Vector3<f64>;

        match light.kind {
            LightKind::Ambient => {
                illumination += light.intensity;
                continue;
            },
            LightKind::Point(light_position) => point_to_light = (light_position - point).normalize(),
            LightKind::Directional(direction) => point_to_light = direction.normalize(),
        };

        // difuse
        illumination += light.intensity * normal.dot(&point_to_light).max(0.0);

        // specular
        if let Some(shininess) = shininess {
            let reflection = normal.scale(2.0*normal.dot(&point_to_light)) - point_to_light;
            let view = view.normalize();

            illumination += light.intensity * reflection.dot(&view).max(0.0).powi(shininess);
        }
    }

    illumination
}

