use crate::Vector3;

// Directions:
pub const ORIGIN: Vector3 = Vector3::new(0.0, 0.0, 0.0);
pub const UP: Vector3 = Vector3::new(0.0, 1.0, 0.0);
pub const DOWN: Vector3 = Vector3::new(0.0, -1.0, 0.0);
pub const RIGHT: Vector3 = Vector3::new(1.0, 0.0, 0.0);
pub const LEFT: Vector3 = Vector3::new(-1.0, 0.0, 0.0);
pub const FORWARD: Vector3 = Vector3::new(0.0, 0.0, -1.0);
pub const BACKWARD: Vector3 = Vector3::new(0.0, 0.0, 1.0);