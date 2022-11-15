pub mod scene;
pub mod ray;
pub mod image;
pub mod camera;
pub mod renderer;
pub mod colors;
pub mod hit;
pub mod directions;

// Types
pub type Scalar = f64;
pub type Vector3 = nalgebra::Vector3<Scalar>;
pub type UnitVector3 = nalgebra::Unit<Vector3>;

// Constants
pub const PI: Scalar = std::f64::consts::PI;
pub const NORM_EPS: Scalar = 1e-10; // normalization threshold

