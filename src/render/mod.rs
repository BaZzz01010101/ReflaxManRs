mod color;
mod material;
mod trace;
mod sphere;
mod texture;
mod skybox;
mod spotlight;

mod tests;

use crate::math;

pub use self::color::Color;
pub use self::material::Material;
pub use self::material::Kind as MaterialKind;
pub use self::trace::Trace;
pub use self::sphere::Sphere;
pub use self::texture::Texture;
pub use self::skybox::Skybox;
