use crate::math;

pub use self::camera::Camera;
pub use self::color::Color;
pub use self::material::Kind as MaterialKind;
pub use self::material::Material;
pub use self::render::Render;
pub use self::scene::Scene;
pub use self::skybox::Skybox;
pub use self::sphere::Sphere;
pub use self::spot_light::SpotLight;
pub use self::texture::Texture;
pub use self::trace::Trace;
pub use self::triangle::Triangle;

mod color;
mod material;
mod trace;
mod sphere;
mod texture;
mod skybox;
mod spot_light;
// TODO: move camera control to the render and make private
pub mod camera;
mod triangle;
mod scene;
mod render;

#[cfg(test)]
mod tests;

