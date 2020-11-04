use crate::hit_record::HitRecord;
use crate::materials::Material;
use crate::objects::Hitable;
use crate::ray::Ray;
use crate::vec3::dot;
use crate::vec3::Vec3;
use std::sync::Arc;

/// A Sphere in three-dimensional space.
///
/// It is characterized by three properties:  
/// - The coordinates of its center.  
/// - It's radius
/// - A pointer to the material that it is made of.  
///
/// The pointer is an `Arc` because we want to use
/// the Sphere object belonging to a scene in rayon
/// threads through `Arc`s without having to clone them.
pub struct Sphere {
    center: Vec3,
    radius: f32,
    // We want to use spheres with rayon.
    material: Arc<dyn Material>,
}

impl Sphere {
    /// Create a `Sphere` by specifying its `center`, `radius` and `Material`.
    ///
    /// ```
    /// use raytracer::objects::sphere;
    /// use raytracer::vec3::Vec3 as V3;
    /// use raytracer::materials::Dielectric;
    /// use std::sync::Arc;
    /// let center = V3::new(0., 0., 0.);
    /// let arc_material = Arc::new(Dielectric::new(1.5));
    /// let sphere = sphere::Sphere::new(center, 3., arc_material);
    /// ```
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /// Access the center of a `Sphere`.
    ///
    /// ```
    /// # use raytracer::objects::sphere;
    /// # use raytracer::vec3::Vec3;
    /// # use raytracer::materials::Dielectric;
    /// # use std::sync::Arc;
    /// let center = Vec3::new(0., 0., 0.);
    /// let arc_material = Arc::new(Dielectric::new(1.5));
    /// let sphere = sphere::Sphere::new(center, 3., arc_material);
    /// assert_eq!(sphere.center(), &Vec3(0., 0., 0.));
    /// ```
    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    /// Access the `radius` of a `Sphere`.
    ///
    /// ```
    /// # use raytracer::objects::sphere;
    /// # use raytracer::vec3::Vec3;
    /// # use raytracer::materials::Dielectric;
    /// # use std::sync::Arc;
    /// let center = Vec3::new(0., 0., 0.);
    /// let arc_material = Arc::new(Dielectric::new(1.5));
    /// let sphere = sphere::Sphere::new(center, 3., arc_material);
    /// assert_eq!(sphere.radius(), &3.0);
    /// ```
    pub fn radius(&self) -> &f32 {
        &self.radius
    }

    /// Access the `material` a `Sphere` is made of.
    pub fn material(&self) -> Arc<dyn Material> {
        Arc::clone(&self.material)
    }
}

impl Hitable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;
        // Construct the coefficients in a quadratic equation a*x^2 + b*x + c.
        let a = dot(ray.direction(), ray.direction());
        let b = dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - self.radius.powi(2);
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let t_candidate = (-b - discriminant.sqrt()) / a;
            if t_candidate > t_min && t_candidate < t_max {
                return Some(HitRecord {
                    parameter: t_candidate,
                    point_at_parameter: ray.point_at_parameter(t_candidate),
                    normal: (ray.point_at_parameter(t_candidate) - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
            let t_candidate = (-b + discriminant.sqrt()) / a;
            if t_candidate > t_min && t_candidate < t_max {
                return Some(HitRecord {
                    parameter: t_candidate,
                    point_at_parameter: ray.point_at_parameter(t_candidate),
                    normal: (ray.point_at_parameter(t_candidate) - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }

        None
    }
}
