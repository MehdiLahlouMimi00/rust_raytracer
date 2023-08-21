use crate::{vec3::Vec3, ray::Ray};

mod vec3;
mod color;
mod ray;
mod calcs;

#[allow(unused_variables)]
fn main() {
    // Image variables
    let image_width:i32 = 400;
    let aspect_ratio = 16./9.;
    let image_height:i32 = calcs::calculate_image_height(image_width, aspect_ratio);
    

    // Camera 
    let viewport_height = 2.;
    let viewport_width = calcs::calculate_viewport_width(viewport_height, image_width, image_height);
    
    let focal_length = 1.;
    let camera_center = vec3::Vec3 {x:0., y:0., z:0.};
        
        // The sizing vectors
    let viewport_u = vec3::Vec3 {x : viewport_width, y : 0., z : 0.};    
    let viewport_v = vec3::Vec3 {x : 0., y : -viewport_height, z : 0.};

        // Elementary vectors
    let pixel_delta_u = viewport_u/(image_width as f64);
    let pixel_delta_v = viewport_v/(image_height as f64);

        // Rendered pixels
    let viewport_upper_left = camera_center - Vec3 {x: 0., y:0., z: focal_length} - viewport_u/2. - viewport_v/2.;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;


    // Render
    println!("P3\n{} {}\n255", image_width, image_height);    // header of the file

    for j in 0i32..image_height
    {
        for i in 0i32..image_width
        {

            let pixel_center = pixel00_loc + ((pixel_delta_u)*(i as f64)) + (pixel_delta_v*(j as f64));
            let ray_direction = pixel_center - camera_center;
            
            let r = Ray {orig : pixel_center, direc : ray_direction};

            let color_vector = ray_color(r);

            color::render_color(color_vector);
        }
    }

    
}



fn hit_sphere(center :vec3::Vec3, radius : f64, r : Ray) -> bool
{
    let oc = r.origin() - center;
    
    let a = vec3::dot_product(r.direction(), r.direction());
    let b = 2. * vec3::dot_product(oc, r.direction());
    let c = vec3::dot_product(oc, oc) + radius*radius;

    let discriminant = b*b - 4.*a*c;

    return discriminant >= 0.;
}


fn ray_color( r : ray::Ray) -> vec3::Vec3
{

    if hit_sphere(Vec3 { x: (0.), y: (0.), z: (-1.) }, 2., r)
    {
        return Vec3 { x : (1.), y : (0.), z : (0.)};
    }

    else 
    {
        let unit_direction = vec3::normalized_vector(r.direction());
        let a = 0.5 * (unit_direction.y + 1.);
        return (vec3::Vec3 {x : 1., y : 1., z : 1.}) * (1. - a) + (vec3::Vec3 {x : 0.7, y : 0.7, z : 1.}) * a;    
    }
    
}
