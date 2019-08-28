use rand::prelude::*;

use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::dot;
use crate::vec3::unit_vector;
use crate::vec3::Vec3;

/// Generate a random vector in the unit sphere.
pub fn random_in_unit_sphere() -> Vec3 {
    // TODO: CHECK whether it is efficient to regenerate a random number generator on every function call.
    let mut rng = rand::thread_rng();
    // Start with a vector that has length larger than 1!
    let mut p = Vec3(10., 10., 10.);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3(1., 1., 1.);
    }
    p
}

/// Schlick's approximation for the dependence of reflectivity of glass on the angle.
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

/// Compute the reflected ray of `v` with the normal at the intersection point given by `n`.
///
/// ```
/// # use raytracer::vec3::Vec3;
/// # use raytracer::materials::reflect;
/// let incoming = Vec3(1., -1., 0.);
/// let normal = Vec3(0., 1., 0.);
/// let reflected = reflect(&incoming, &normal);
/// assert_eq!(reflected.x(), 1.);
/// assert_eq!(reflected.y(), 1.);
/// assert_eq!(reflected.z(), 0.);
/// ```
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * dot(&v, &n) * *n
}

/// Compute the refracted ray vector.
///
/// Input is the incoming vector `v` and the normal `n` at the point
/// where the ray hits the surface between two materials of different
/// refractive indices whose ratio is given by `ni_over_nt`.
///
/// Sometimes, refraction is not possible, if Snell's law has no solution
/// and in this case None is returned.
pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = unit_vector(&v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1. - dt.powi(2));
    if discriminant > 0. {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

/// A material trait.
///
/// A material is characterized by the way a ray is scattered
/// from the hit point.
///
/// Given the incoming ray and hit record containing the information
/// on the point where this ray hits the surface, the scattered ray,
/// as well as the attenuation vector are returned.
pub trait Material: Send + Sync {
    // Subtraiting `Send` & `Sync` in order to be able to use the material
    // objects in rayon threads using `Arc` without having to copy them.

    /// Return the scattered ray and the attenuation.
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

/// A Lambertian (diffuse) material.
#[derive(Default, Debug)]
pub struct Lambertian {
    attenuation: Vec3,
}

impl Lambertian {
    /// Create a Lambertian material by specifying its attenuation.
    ///
    /// ```
    /// # use raytracer::materials::Lambertian;
    /// # use raytracer::vec3::Vec3;
    /// // Define how each color component is attenuated by the diffuse material.
    /// let attenuation = Vec3(0.8, 0.7, 0.9);
    /// let diffuse_material = Lambertian::new(attenuation);
    /// ```
    pub fn new(attenuation: Vec3) -> Lambertian {
        Lambertian { attenuation }
    }

    /// Extract the attenuation information from the Lambertian material.
    ///
    /// ```
    /// # use raytracer::materials::Lambertian;
    /// # use raytracer::vec3::Vec3;
    /// // Define how each color component is attenuated by the diffuse material.
    /// let attenuation = Vec3(0.8, 0.7, 0.9);
    /// let diffuse_material = Lambertian::new(attenuation);
    /// assert_eq!(diffuse_material.attenuation().x(), 0.8);
    /// assert_eq!(diffuse_material.attenuation().y(), 0.7);
    /// assert_eq!(diffuse_material.attenuation().z(), 0.9);
    /// ```
    pub fn attenuation(&self) -> &Vec3 {
        &self.attenuation
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.point_at_parameter + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(
            hit.point_at_parameter,
            target - hit.point_at_parameter,
        );
        Some((scattered, self.attenuation))
    }
}

/// A metal (reflective) material.
#[derive(Default, Debug)]
pub struct Metal {
    attenuation: Vec3,
    fuzzy: f32,
}

impl Metal {
    /// Create a new metal by specifying its attenuation and fuzzyness.AsMut
    ///
    /// The attenuation specifies how each color component in the scattered ray
    /// is attenuated while the `fuzzy` parameter indicates the randomness in
    /// the direction of the reflection. `fuzzy` is forced to be between 0 and 1.
    ///
    /// A fuzzyness of 0 indicates perfect reflection.
    ///
    /// ```
    /// # use raytracer::materials::Metal;
    /// # use raytracer::vec3::Vec3;
    /// // Define how each color component is attenuated by the reflective material.
    /// let attenuation = Vec3(0.8, 0.7, 0.9);
    /// let metal = Metal::new(attenuation, 0.0);
    /// ```
    pub fn new(attenuation: Vec3, fuzzy: f32) -> Metal {
        if fuzzy > 1. {
            Metal {
                attenuation,
                fuzzy: 1.,
            }
        } else {
            Metal { attenuation, fuzzy }
        }
    }

    /// Extract the attenuation information from the Metal material.
    ///
    /// ```
    /// # use raytracer::materials::Metal;
    /// # use raytracer::vec3::Vec3;
    /// // Define how each color component is attenuated by the reflective material.
    /// let attenuation = Vec3(0.8, 0.7, 0.9);
    /// let metal = Metal::new(attenuation, 0.1);
    /// assert_eq!(metal.attenuation().x(), 0.8);
    /// assert_eq!(metal.attenuation().y(), 0.7);
    /// assert_eq!(metal.attenuation().z(), 0.9);
    /// ```
    pub fn attenuation(&self) -> &Vec3 {
        &self.attenuation
    }

    /// Extract the fuzzy parameter from the Metal material.
    ///
    /// ```
    /// # use raytracer::materials::Metal;
    /// # use raytracer::vec3::Vec3;
    /// // Define how each color component is attenuated by the reflective material.
    /// let attenuation = Vec3(0.8, 0.7, 0.9);
    /// let fuzzy = 0.1;
    /// let metal = Metal::new(attenuation, fuzzy);
    /// assert_eq!(metal.fuzzy(), 0.1);
    /// ```
    pub fn fuzzy(&self) -> f32 {
        self.fuzzy
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&unit_vector(ray.direction()), &hit.normal);
        let scattered = Ray::new(
            hit.point_at_parameter,
            reflected + self.fuzzy * random_in_unit_sphere(),
        );
        if dot(&scattered.direction(), &hit.normal) > 0. {
            Some((scattered, self.attenuation))
        } else {
            None
        }
    }
}

/// A Dielectric (transparent) material.
///
/// A dielectric material is characterized by its refractive index.
#[derive(Debug, Default)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    /// Create a new dielectric material by specifying its refractive index.
    ///
    /// ```
    /// # use raytracer::materials::Dielectric;
    /// let ref_idx = 1.5;
    /// let dielectric = Dielectric::new(ref_idx);
    /// ```
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }

    /// Extract the refractive index of a dielectric material.
    ///
    /// ```
    /// # use raytracer::materials::Dielectric;
    /// let ref_idx = 1.5;
    /// let dielectric = Dielectric::new(ref_idx);
    /// assert_eq!(dielectric.ref_idx(), 1.5);
    /// ```
    pub fn ref_idx(&self) -> f32 {
        self.ref_idx
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.direction(), &hit.normal);
        let normal_dir = dot(&ray.direction(), &hit.normal);
        let outward_normal = if normal_dir > 0. {
            -hit.normal
        } else {
            hit.normal
        };
        let ni_over_nt = if normal_dir > 0. {
            self.ref_idx
        } else {
            1.0 / self.ref_idx
        };
        let cosine = if normal_dir > 0. {
            self.ref_idx() * dot(&ray.direction(), &hit.normal) / ray.direction().length()
        } else {
            -dot(&ray.direction(), &hit.normal) / ray.direction().length()
        };

        let mut rng = rand::thread_rng();

        let attenuation = Vec3(1., 1., 1.);
        match refract(&ray.direction(), &outward_normal, ni_over_nt) {
            None => Some((Ray::new(hit.point_at_parameter, reflected), attenuation)),
            Some(refracted) => {
                if rng.gen::<f32>() < schlick(cosine, self.ref_idx) {
                    Some((Ray::new(hit.point_at_parameter, reflected), attenuation))
                } else {
                    Some((Ray::new(hit.point_at_parameter, refracted), attenuation))
                }
            }
        }
    }
}
