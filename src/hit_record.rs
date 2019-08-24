use crate::materials::Material;
use crate::vec3::Vec3;
use std::rc::Rc;

/// Collect information on the hit point between a ray and an object.
///
/// The collected information contains:
/// 1. The parameter `t` at which the ray intersects the object.
/// 2. The intersection point itself, given by the ray at parameter `t`.
/// 3. The surface normal of the object at the hit point
// #[derive(Debug)]
pub struct HitRecord {
    pub parameter: f32,
    pub point_at_parameter: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}
