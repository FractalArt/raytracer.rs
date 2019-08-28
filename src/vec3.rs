//! This module implements a lightweight vector.
//!
//! It will be used to deal with color at intermediate
//! stages (in the end the specific color class from
//! the image library will be used) and vectors in
//! three dimensional space.
//!
//! ```
//! use raytracer::vec3::Vec3;
//! let v = Vec3::new(1.,2.,3.);
//! ```
//!
//! The usual vector space operations are supported,
//! such as adding and subtracting vectors
//!
//! ```
//! # use raytracer::vec3::Vec3;
//! let v1 = Vec3(1., 2., 3.);
//! let v2 = Vec3(3., 4., 5.);
//! let sum = v1 + v2;
//! let diff = v1 - v2;
//! ```
//!
//! Vectors can also be multiplied both from the left and
//! from the right and divided from the right
//! by scalars.
//!
//! ```
//! # use raytracer::vec3::Vec3;
//! let v = Vec3(1., 2., 3.);
//! let prod_left = 32. * v;
//! let prod_right = v * 45.;
//! let quot_right = v / 2.;
//! ```
//!
//! All of the operations are also allowed in assign mode
//!
//! ```
//! # use raytracer::vec3::Vec3;
//! let mut v1 = Vec3(1., 2., 3.);
//! let mut v2 = Vec3(3., 4., 5.);
//! let scalar = 32;
//! v1 += v2;
//! v2 -= v1;
//! v1 *= 3.;
//! v2 /= 2.;
//! ```
//!
//! The dot and cross products are supported as well
//!
//! ```
//! use raytracer::vec3::Vec3;
//! use raytracer::vec3::dot;
//! use raytracer::vec3::cross;
//! let mut v1 = Vec3(1., 2., 3.);
//! let mut v2 = Vec3(3., 4., 5.);
//! let dot_product = dot(&v1, &v2);
//! assert_eq!(dot_product, 26.);
//! let cross_product = cross(&v1, &v2);
//! ```

use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    /// Create a new vector.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    /// Access the component red of the RGB code.
    /// Only useful if the underlying data is color.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let red = Vec3::new(255., 0., 0.);
    /// assert_eq!(red.r(), 255.);
    /// ```
    pub fn r(&self) -> f32 {
        self.0
    }

    /// Access the component g of the RGB code.
    /// Only useful if the underlying data is color.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let green = Vec3::new(0., 255., 0.);
    /// assert_eq!(green.g(), 255.);
    /// ```
    pub fn g(&self) -> f32 {
        self.1
    }

    /// Access the component blue of the RGB code.
    /// Only useful if the underlying data is color.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let blue = Vec3::new(0., 0., 255.);
    /// assert_eq!(blue.b(), 255.);
    /// ```
    pub fn b(&self) -> f32 {
        self.2
    }

    /// Access the x-component of the vector.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(v.x(), 1.);
    /// ```
    pub fn x(&self) -> f32 {
        self.0
    }

    /// Access the y-component of the vector.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(v.y(), 2.);
    /// ```
    pub fn y(&self) -> f32 {
        self.1
    }

    /// Access the z-component of the vector.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(v.z(), 3.);
    /// ```
    pub fn z(&self) -> f32 {
        self.2
    }

    /// Compute the momentum squared of a vector.
    ///
    /// Only makes sense if a spatial vector is considered.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let v = Vec3::new(0.,3.,4.);
    /// assert_eq!(v.squared_length(), 25.);
    /// ```
    pub fn squared_length(&self) -> f32 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    /// Compute modulus of a vector.
    ///
    /// Only makes sense if a spatial vector is considered.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let v = Vec3::new(0.,3.,4.);
    /// assert_eq!(v.length(), 5.);
    /// ```
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    /// Normalize a vector to unity.
    ///
    /// ```
    /// # use raytracer::vec3::Vec3;
    /// let v = Vec3::new(0.,3.,4.);
    /// assert_eq!(v.make_unit_vector().length(), 1.);
    /// ```
    pub fn make_unit_vector(&self) -> Vec3 {
        Vec3::new(self.0, self.1, self.2) / self.length()
    }
}

/// Negate a Vec3.
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

/// Add two vectors.
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

/// Subtract two vectors.
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

/// Multiply two vectors component-wise
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

/// Multiply the components by the vector by the components of rhs.
impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2);
    }
}

/// Multiply a vector by a number from the right.
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

/// Multiply a vector by a number from the left.
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs);
    }
}

/// Compute the dot product between two vectors.
///
/// ```
/// # use raytracer::vec3::Vec3;
/// # use raytracer::vec3::dot;
/// let v1 = Vec3(1., 2., 3.);
/// let v2 = Vec3(4., 5., 6.);
/// assert_eq!(dot(&v1, &v2), 32.);
/// ```
pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

/// Compute the cross product between two vectors.
///
/// ```
/// # use raytracer::vec3::Vec3;
/// # use raytracer::vec3::cross;
/// let unit_x = Vec3(1., 0., 0.);
/// let unit_y = Vec3(0., 1., 0.);
/// let unit_z = Vec3(0., 0., 1.);
/// let cross_product = cross(&unit_x, &unit_y);
/// assert_eq!(cross_product.x(), 0.);
/// assert_eq!(cross_product.y(), 0.);
/// assert_eq!(cross_product.z(), 1.);
/// let cross_product = cross(&unit_x, &unit_z);
/// assert_eq!(cross_product.x(), 0.);
/// assert_eq!(cross_product.y(), -1.);
/// assert_eq!(cross_product.z(), 0.);
/// let cross_product = cross(&unit_y, &unit_z);
/// assert_eq!(cross_product.x(), 1.);
/// assert_eq!(cross_product.y(), 0.);
/// assert_eq!(cross_product.z(), 0.);
/// ```
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3(
        v1.1 * v2.2 - v1.2 * v2.1,
        v1.2 * v2.0 - v1.0 * v2.2,
        v1.0 * v2.1 - v1.1 * v2.0,
    )
}

/// Create a unit vector without modifying the input.
///
/// ```
/// # use raytracer::vec3::Vec3;
/// # use raytracer::vec3::unit_vector;
/// let v = Vec3(5., 0., 0.);
/// let u = unit_vector(&v);
/// assert_eq!(u.x(), 1.);
/// ```
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

// ------------------------------------------------------------
// ----------------------- UNIT TESTS -------------------------
// ------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test negative Vec3.
    fn test_neg_vec3() {
        let v = Vec3(1., 2., 3.);
        let neg_v = -v;
        assert_eq!(neg_v.x(), -1.);
        assert_eq!(neg_v.y(), -2.);
        assert_eq!(neg_v.z(), -3.);
    }

    #[test]
    // Test default Vec3.
    fn test_default_vec3() {
        let v1: Vec3 = Default::default();
        assert_eq!(v1.x(), 0.);
        assert_eq!(v1.y(), 0.);
        assert_eq!(v1.z(), 0.);
    }

    #[test]
    // Test adding two vectors.
    fn add_vectors() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(3., 2., 1.);
        let sum = v1 + v2;
        assert_eq!(sum.x(), 4.);
        assert_eq!(sum.y(), 4.);
        assert_eq!(sum.z(), 4.);
    }

    #[test]
    // Test adding two vectors.
    fn add_assign_vectors() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(3., 2., 1.);
        v1 += v2;
        assert_eq!(v1.x(), 4.);
        assert_eq!(v1.y(), 4.);
        assert_eq!(v1.z(), 4.);
    }

    #[test]
    // Test subtracting two vectors.
    fn subtract_vectors() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(3., 2., 1.);
        let sum = v1 - v2;
        assert_eq!(sum.x(), -2.);
        assert_eq!(sum.y(), 0.);
        assert_eq!(sum.z(), 2.);
    }

    #[test]
    // Test subtract assigning two vectors.
    fn subtract_assign_vectors() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(3., 2., 1.);
        v1 -= v2;
        assert_eq!(v1.x(), -2.);
        assert_eq!(v1.y(), 0.);
        assert_eq!(v1.z(), 2.);
    }

    #[test]
    // Test multiplication of a vector with a float from the right.
    fn multiply_vector_by_float_right() {
        let v1 = Vec3::new(1., 2., 3.);
        let twice = v1 * 2.;
        assert_eq!(twice.x(), 2.);
        assert_eq!(twice.y(), 4.);
        assert_eq!(twice.z(), 6.);
    }

    #[test]
    // Test component-wise multiplication of two vectors.
    fn multiply_vector_by_vector_component_wise() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(3., 2., 1.);
        let component_product = v1 * v2;
        assert_eq!(component_product.x(), 3.);
        assert_eq!(component_product.y(), 4.);
        assert_eq!(component_product.z(), 3.);
    }

    #[test]
    // Test component-wise assign-multiplication of two vectors.
    fn multiply_assign_vector_by_vector_component_wise() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(3., 2., 1.);
        v1 *= v2;
        assert_eq!(v1.x(), 3.);
        assert_eq!(v1.y(), 4.);
        assert_eq!(v1.z(), 3.);
    }

    #[test]
    // Test multiplication of a vector with a float from the left.
    fn multiply_vector_by_float_left() {
        let v1 = Vec3::new(1., 2., 3.);
        let twice = 2. * v1;
        assert_eq!(twice.x(), 2.);
        assert_eq!(twice.y(), 4.);
        assert_eq!(twice.z(), 6.);
    }

    #[test]
    // Test multiplication assign of a vector with a float from the right.
    fn multiply_assign_vector_by_float_right() {
        let mut v1 = Vec3::new(1., 2., 3.);
        v1 *= 2.;
        assert_eq!(v1.x(), 2.);
        assert_eq!(v1.y(), 4.);
        assert_eq!(v1.z(), 6.);
    }

    #[test]
    // Test division of a vector by a float from the right.
    fn divide_vector_by_float_right() {
        let v1 = Vec3::new(1., 2., 3.);
        let half = v1 / 2.;
        assert_eq!(half.x(), 0.5);
        assert_eq!(half.y(), 1.);
        assert_eq!(half.z(), 1.5);
    }

    #[test]
    // Test division of a vector by a float from the right.
    fn divide_assign_vector_by_float_right() {
        let mut v1 = Vec3::new(1., 2., 3.);
        v1 /= 2.;
        assert_eq!(v1.x(), 0.5);
        assert_eq!(v1.y(), 1.);
        assert_eq!(v1.z(), 1.5);
    }

}
