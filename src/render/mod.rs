mod color;
mod material;
mod trace;
mod sphere;
mod texture;
mod skybox;
mod spot_light;
mod camera;
mod triangle;
mod scene;
mod render;

mod tests;

use crate::math;

pub use self::color::Color;
pub use self::material::Material;
pub use self::material::Kind as MaterialKind;
pub use self::trace::Trace;
pub use self::sphere::Sphere;
pub use self::texture::Texture;
pub use self::skybox::Skybox;
pub use self::spot_light::SpotLight;
pub use self::camera::Camera;
pub use self::triangle::Triangle;
pub use self::scene::Scene;
pub use self::render::Render;
