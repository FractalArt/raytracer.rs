//! A three-dimensional ray.
//!
//! It is parametrized by
//!
//! r = origin + t * direction
//!
//! where `r`, `origin` and `direction` are elements of type
//! `Vec3` and `t` is a parameter.

use crate::vec3::Vec3;

/// Ray in 3-dimensional space.
///
/// A ray is given by an origin and a direction.
#[derive(Debug, Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    /// Create a ray by specifying `origin` and `direction`.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// # use raytracer::ray::Ray;
    /// let origin = Vec3(0., 0., 0.);
    /// let direction = Vec3(1., 3., 0.);
    /// let ray = Ray::new(origin, direction);
    /// ```
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// Access the origin of the ray.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// # use raytracer::ray::Ray;
    /// let origin = Vec3(0., 0., 0.);
    /// let direction = Vec3(1., 3., 0.);
    /// let ray = Ray::new(origin, direction);
    /// assert_eq!(ray.origin().0, 0.);
    /// ```
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    /// Access the direction of the ray.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// # use raytracer::ray::Ray;
    /// let origin = Vec3(0., 0., 0.);
    /// let direction = Vec3(1., 3., 0.);
    /// let ray = Ray::new(origin, direction);
    /// assert_eq!(ray.direction().y(), 3.);
    /// ```
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    /// Evaluate the ray coordinates at a parameter point t.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// # use raytracer::ray::Ray;
    /// let origin = Vec3(0., 0., 0.);
    /// let direction = Vec3(1., 3., 0.);
    /// let ray = Ray::new(origin, direction);
    /// let point_on_ray = ray.point_at_parameter(2.);
    /// assert_eq!(point_on_ray.x(), 2.);
    /// assert_eq!(point_on_ray.y(), 6.);
    /// assert_eq!(point_on_ray.z(), 0.);
    /// ```
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
