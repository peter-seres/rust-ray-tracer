# rust_ray_tracer

This is simple CPU implementation of a ray tracer in Rust. The goal is mainly for me to learn the basic Rust programming concepts.
The shader features currently implemented:

* Lambert shading
* Single reflection

Implemented primitives are: spheres and infinite planes. Light sources are point lights for now.
The number of objects in the scene are provided at compile time at the moment.

## Example output

![traced](https://user-images.githubusercontent.com/27952562/117550379-4fe90980-b040-11eb-9084-549f87beebc2.png)

## Future to-do list

* N-depth reflections
* Anti-aliasing
* Specular reflections
* Emission material
* Camera movement
* Different lights: spots, sun, HDRI
