use crate::hittable_list;
use crate::random;
use crate::ray;
use crate::vec3;
use crate::color;
use crate::hittable;
use crate::interval;
use crate::utils;

use rand::prelude::*;


#[derive(Copy, Clone)]
pub struct Camera 
{
    pub aspect_ratio: f64,          
    pub image_width: i32,  
    pub samples_per_pixel : i32,   
    pub center : vec3::Vec3,
    pub max_depth : i32,    
}


impl Camera 
{
    pub fn render(&self, world: &mut hittable_list::hittable_list) {
        let (image_height, center, pixel00_loc, pixel_delta_u, pixel_delta_v) = self.initialize();

        println!("P3\n{} {}\n255", self.image_width, image_height);

        for j in 0i32..image_height {

            eprint!("\rScanlines remaining: {} ", image_height - j);
            
            for i in 0i32..self.image_width {
                /*let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
                let ray_direction = pixel_center - center;
                let r = ray::Ray {orig : center, direc : ray_direction };

                let pixel_color = self.ray_color(&r, world);
                color::render_color(pixel_color);    // Add a second */

                let mut pixel_color = vec3::Vec3 {x : 0., y : 0., z : 0.};
                
                for k in 0..self.samples_per_pixel
                {
                    let r = self.get_ray(i, j, pixel_delta_u, pixel_delta_v, pixel00_loc, center);
                    
                    pixel_color = pixel_color + self.ray_color(&r, world, self.max_depth);
                } 

                color::render_color(pixel_color, self.samples_per_pixel);


            }
        }

        
    }



    fn get_ray(self, i : i32, j : i32, pdu : vec3::Vec3, pdv : vec3::Vec3, p00l : vec3::Vec3, center : vec3::Vec3) -> ray::Ray
    {
        let pixel_center = p00l + pdu*(i as f64) + pdv * (j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square(pdu, pdv);

        let ray_origin = center;
        let ray_direction = pixel_sample - ray_origin;

        return ray::Ray { orig: ray_origin, direc: ray_direction  };
    }



    fn pixel_sample_square(self, pdu : vec3::Vec3, pdv : vec3::Vec3) -> vec3::Vec3
    {
        let px = -0.5 + utils::random();
        let py = -0.5 + utils::random();
        return pdu * px +  pdv * py;
    }



    fn initialize(&self) -> (i32, vec3::Vec3, vec3::Vec3, vec3::Vec3, vec3::Vec3)
    {
        let image_height = ((self.image_width as f64 / self.aspect_ratio) as i32).max(1);
        //let center = vec3::Vec3 {x : 0.0, y : 0.0, z : 0.0};

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        let viewport_u = vec3::Vec3 {x : viewport_width, y : 0.0, z : 0.0};
        let viewport_v = vec3::Vec3 {x : 0.0, y : -viewport_height, z : 0.0};

        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = self.center - vec3::Vec3 {x : 0.0, y : 0.0, z : focal_length} - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left +  (pixel_delta_u + pixel_delta_v)* 0.5;

        (image_height, self.center, pixel00_loc, pixel_delta_u, pixel_delta_v)
    }


    fn ray_color(&self, r: &ray::Ray, world: &mut hittable_list::hittable_list, depth : i32) -> vec3::Vec3
    {

        if depth <= 0
        {
            return vec3::Vec3{x : 0., y : 0., z : 0.};
        }

        else
        {
            let mut rec = hittable::hitRecord { p: vec3::Vec3 { x: (0.), y: (0.), z: (0.) }, normal: vec3::Vec3 { x: (0.), y: (0.), z: (0.) }, t: 0., font_face: false };

            if world.hit(*r, 0., 1e51, & mut rec)
            {
                let direction = rec.normal + vec3::random_unit_vector();
                //return (rec.normal + vec3::Vec3 {x : 1., y : 1., z : 1.})* 0.5;
                return  self.ray_color(&ray::Ray { orig: rec.p, direc: direction }, world, depth - 1) * 0.5;
            }

            
            else 
            {
                let unit_direction = vec3::normalized_vector(r.direction());
                let a = 0.5 * (unit_direction.y + 1.);
                
                return (vec3::Vec3 {x : 1., y : 1., z : 1.}) * (1. - a) + (vec3::Vec3 {x : 0.7, y : 0.7, z : 1.}) * a;
            }
        }
    }


   

}
