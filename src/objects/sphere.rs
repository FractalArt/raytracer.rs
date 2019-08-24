use crate::hit_record::HitRecord;
use crate::materials::Material;
use crate::objects::Hitable;
use crate::ray::Ray;
use crate::vec3::dot;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn radius(&self) -> &f32 {
        &self.radius
    }

    pub fn material(&self) -> &Rc<dyn Material> {
        &self.material
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
                    normal: (ray.point_at_parameter(t_candidate) - self.center)
                        / self.radius,
                    material: self.material.clone(),
                });
            }
            let t_candidate = (-b + discriminant.sqrt()) / a;
            if t_candidate > t_min && t_candidate < t_max {
                return Some(HitRecord {
                    parameter: t_candidate,
                    point_at_parameter: ray.point_at_parameter(t_candidate),
                    normal: (ray.point_at_parameter(t_candidate) - self.center)
                        / self.radius,
                    material: self.material.clone(),
                });
            }
        }

        None
    }
}
