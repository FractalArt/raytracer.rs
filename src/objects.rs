use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub mod sphere;

/// Trait for objects that can be hit by a ray of light.
pub trait Hitable: Send + Sync{
    // Subtraiting `Send` & `Sync` in order to be able to use the hitable 
    // objects in rayon threads without having to copy them.

    /// For an incoming ray check whether it intersects the object.
    ///
    /// Only allow intersection points given by a parameter `t` that falls
    /// within `t_min` and `t_max`.
    ///
    /// If the ray does not intersect the object, `None` is returned.
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitableList {
    hitable_objects: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    /// Create a collection of hitable objects.
    pub fn new(hitable_objects: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { hitable_objects }
    }
}

impl Hitable for HitableList {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for object in &self.hitable_objects {
            if let Some(hit) = object.intersect(&ray, t_min, closest_so_far) {
                closest_so_far = hit.parameter;
                hit_record = Some(hit);
            }
        }

        hit_record
    }
}
