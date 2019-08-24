use rand::prelude::*;

use crate::ray::Ray;
use crate::vec3::cross;
use crate::vec3::unit_vector;
use crate::vec3::Vec3;

// Choose a vector on the unit disk.
fn random_in_unit_disk() -> Vec3 {
    // Random number generator
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vec3(1., 1., 0.);
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

/// A simple camera.
#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    /// Create a new camera.
    ///
    /// This is done by specifying
    /// 1. The point `look_from` where the camera is located.
    /// 2. The point `look_at` where the camera is pointing at.
    /// 3. The vector `view_up` which indicates the tilting of the camera.
    /// 4. The vertical field of view in degrees.
    /// 5. The `aspect` indicating the ratio of the horizontal over the vertical
    ///    dimension of the image.
    /// 6. The `aperture` which indicates the size of the lens.
    /// 7. The distance `focus_dist` to the plane which is in focus.
    ///
    /// ```
    /// use raytracer::camera::Camera;
    /// use raytracer::vec3::Vec3;
    /// let vertical_field_of_view_in_degrees = 20.;
    /// let aspect_x_over_y = 2.;
    /// let look_from = Vec3(-2., 2., 1.);
    /// let look_at = Vec3(0., 0., -1.);
    /// let view_up = Vec3(0., 1., 0.);
    /// let aperture = 2.0;
    /// let dist_to_focus = 3.0;
    /// let cam = Camera::new(look_from,
    ///                       look_at,
    ///                       view_up,
    ///                       vertical_field_of_view_in_degrees, aspect_x_over_y,
    ///                       aperture, dist_to_focus);
    /// ```
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.;
        let theta = vfov * std::f32::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&view_up, &w));
        let v = cross(&w, &u);
        let lower_left_corner = origin
            - half_width * focus_dist * u
            - half_height * focus_dist * v
            - focus_dist * w;
        let horizontal = 2. * half_width * u * focus_dist;
        let vertical = 2. * half_height * v * focus_dist;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }

    /// Return the ray passing through a given point in the image.
    ///
    /// The ray passing through
    ///
    /// x_frac * x_dim + y_frac * y_dim
    ///
    /// is returned.
    pub fn get_ray(&self, x_frac: f32, y_frac: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner
                + x_frac * self.horizontal
                + y_frac * self.vertical
                - self.origin
                - offset,
        )
    }
}
