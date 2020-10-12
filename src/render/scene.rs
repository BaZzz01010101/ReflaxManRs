use std::f32::consts::{PI, FRAC_PI_2};

use anyhow::{Result, Error, Context};

use super::math::{Vector3, Matrix33, clamp};
use super::math::constants::{DELTA, VERY_SMALL_NUMBER};
use super::{Skybox, SpotLight, Sphere, Triangle, Color, Texture, Material, MaterialKind, Trace};
use std::rc::Rc;

#[derive(Default)]
pub struct Scene {
  skybox: Skybox,
  skybox_color: Color,
  trace_objects: Vec<Box<dyn Trace>>,
  spot_lights: Vec<SpotLight>,
  diff_light_color: Color,
  diff_light_power: f32,
}

impl Scene {
  pub fn new(skybox: Skybox, diff_light_color: Color, diff_light_power: f32) -> Scene {
    Scene {
      skybox,
      skybox_color: &diff_light_color * diff_light_power,
      trace_objects: Vec::new(),
      spot_lights: Vec::new(),
      diff_light_color,
      diff_light_power,
    }
  }

  pub fn add_sphere(&mut self, center: Vector3, radius: f32, material: Material) {
    let sphere = Sphere::new(center, radius, material);
    self.trace_objects.push(Box::new(sphere));
  }

  pub fn add_triangle(&mut self, vertices: [&Vector3; 3], material: Material,
                      texture_data: Option<(Rc<Texture>, [(f32, f32); 3])>)
  {
    let mut triangle = Triangle::new(vertices, material);

    if let Some((
      texture, [
      (u0, v0),
      (u1, v1),
      (u2, v2),
      ])) = texture_data
    {
      triangle.set_texture(texture, [u0, u1, u2], [v0, v1, v2]);
    }

    self.trace_objects.push(Box::new(triangle));
  }

  pub fn add_spot_light(&mut self, origin: Vector3, radius: f32, color: Color, power: f32) {
    let spot_light = SpotLight::new(origin, radius, color, power);
    self.spot_lights.push(spot_light);
  }

  pub fn trace(&self, origin: &Vector3, ray: &Vector3, max_reflections: u32) -> Result<Color> {
    let mut origin = origin.clone();
    let mut ray = ray.clone();
    let random_vec = Vector3::random_inside_sphere(1.0);
    let mut color_multiplier = Color::new(1.0, 1.0, 1.0);
    let mut output_color = Color::new(0.0, 0.0, 0.0);

    // going deep up to maxReflections
    for _ in 0..max_reflections {
      let mut min_distance = f32::MAX;
      let mut hit_object: Option<&Box<dyn Trace>> = None;
      let mut drop = Vector3::default();
      let mut norm = Vector3::default();
      let mut reflect = Vector3::default();
      let mut drop_material = Material::new(MaterialKind::Metal, Color::new(0.0, 0.0, 0.0), 0.0, 0.0);

      // tracing intersections with all scene objects and select closest
      for obj in &self.trace_objects {
        let mut cur_drop = Vector3::default();
        let mut cur_norm = Vector3::default();
        let mut cur_reflect = Vector3::default();
        let mut cur_drop_material = Material::new(MaterialKind::Metal, Color::new(0.0, 0.0, 0.0), 0.0, 0.0);
        let mut cur_dist: f32 = 0.0;
        let hit = obj.trace(&origin, &ray, Some(&mut cur_drop), Some(&mut cur_norm), Some(&mut cur_reflect), Some(&mut cur_dist), Some(&mut cur_drop_material))?;

        if hit && cur_dist < min_distance {
          min_distance = cur_dist;
          drop = cur_drop;
          norm = cur_norm;
          reflect = cur_reflect;
          drop_material = cur_drop_material;
          hit_object = Some(obj);
        }
      }

      if let Some(hit_object) = hit_object {
        let ray_length = ray.length();
        let norm_length = norm.length();
        let reflect_length = reflect.length();
        let mut sum_light_color = Color::new(0.0, 0.0, 0.0);
        let mut sum_spec_color = Color::new(0.0, 0.0, 0.0);

        // tracing each light source visibility
        for lt in &self.spot_lights {
          let light = lt;
          let drop_to_light = &light.origin - &drop;

          // check only if drop point faced to light source
          if &drop_to_light * &norm > VERY_SMALL_NUMBER {
            // make randomization within a radius of light source for smooth shadows
            let light_radius = light.radius;
            let drop_to_light_randomized = &drop_to_light + &random_vec * light_radius;
            let mut in_shadow = false;

            // checking whether we are in the shadow of some scene object
            for obj in &self.trace_objects {
              let obj_ptr = obj as *const Box<Trace>;
              let hit_obj_ptr = hit_object as *const Box<Trace>;

              // skip the object that was hit from shadow check
              if std::ptr::eq(obj_ptr, hit_obj_ptr) {
                continue;
              }

              if obj.trace(&drop, &drop_to_light_randomized, None, None, None, None, None)?
              {
                in_shadow = true;
                break;
              }
            }

            // if we are not in the shadow - proceed illumination
            if !in_shadow {
              // calc illumination from current light source
              let drop_to_light_length = drop_to_light.length();
              let light_color = &light.color;
              let light_power = light.power;
              let mut a = drop_to_light_length * norm_length;

              let light_drop_cos = if a > VERY_SMALL_NUMBER {
                &drop_to_light * &norm / a
              } else {
                0.0
              };

              if light_power > VERY_SMALL_NUMBER {
                sum_light_color += light_color * light_drop_cos * light_power;
              }

              // calc specular reflection from current light source
              a = drop_to_light.sq_length();

              let light_angular_radius_sq_cos = if a > VERY_SMALL_NUMBER {
                1.0 - light_radius * light_radius / a
              } else {
                0.0
              };

              if light_angular_radius_sq_cos > 0.0 {
                let drop_to_light_randomized = drop_to_light.normalized() + &random_vec * (1.0 - drop_material.reflectivity);
                a = drop_to_light_randomized.length() * reflect_length;

                let mut reflect_specular_cos = if a > VERY_SMALL_NUMBER {
                  drop_to_light_randomized * &reflect / a
                } else {
                  0.0
                };

                reflect_specular_cos = clamp(reflect_specular_cos + (1.0 - light_angular_radius_sq_cos.sqrt()), 0.0, 1.0);

                if reflect_specular_cos > VERY_SMALL_NUMBER {
                  let mut spec_power = reflect_specular_cos;

                  if light_radius > VERY_SMALL_NUMBER {
                    spec_power = f32::powf(spec_power, 1.0 + 3.0 * drop_material.reflectivity * drop_to_light_length / light_radius) * drop_material.reflectivity;
                    sum_spec_color += light_color * spec_power;
                  }
                }
              }
            }
          }
        }

        let reflectivity = drop_material.reflectivity;
        let color = &drop_material.color;
        let kind = drop_material.kind;

        sum_light_color = &self.diff_light_color * self.diff_light_power + sum_light_color;

        let mut fin_color: Color;
        if kind == MaterialKind::Dielectric {
          // for dielectric materials count reflectivity using rough approximation of the Fresnel curve
          let a = ray_length * norm_length;

          let drop_angle_cos = if a > VERY_SMALL_NUMBER {
            clamp(&ray * -norm / a, 0.0, 1.0)
          } else {
            0.0
          };

          let reflectivity = 0.2 + 0.8 * f32::powi(1.0 - drop_angle_cos, 3);

          fin_color = (1.0 - reflectivity) * color * sum_light_color + sum_spec_color;
          fin_color *= &color_multiplier;

          // multiply colorMul with counted reflectivity to reduce subsequent reflections impact
          color_multiplier *= reflectivity;
        } else {
          // for metal materials roughly set reflectivity equal to 0.8 according to Fresnel curve
          let reflectivity = 0.8;

          fin_color = (1.0 - reflectivity) * color * sum_light_color + sum_spec_color;
          fin_color *= &color_multiplier;

          // multiply color_multiplier with counted reflectivity and material color
          // to reduce impact and colorize subsequent reflections
          color_multiplier *= color * reflectivity;
        }

        // summarize reflected colors
        output_color += fin_color;
        output_color.clamp();

        // exit if color multiplier too small and counting of subsequent reflection has no sense
        if color_multiplier.r < 0.01 && color_multiplier.g < 0.01 && color_multiplier.b < 0.01 {
          break;
        }

        // select reflection as new ray for tracing and randomize it depending on reflectivity of object
        origin = drop;
        ray = reflect.normalized() + &random_vec * (1.0 - reflectivity);
      } else {
        // no intersections, tracing skybox
        output_color += &color_multiplier * self.skybox.trace(&ray)? * &self.skybox_color;
        output_color.clamp();
        break;
      }
    }

    Ok(output_color)
  }
}