use crate::hittable;
use crate::ray;
use crate::vec3;
use crate::plane;


#[derive(Clone)]
pub struct hittable_list
{
    // Basically we will make a list for every kind of hittable stuff
    pub spheres : Vec<hittable::Sphere>,
    pub planes : Vec<plane::plane>,
    pub finPlanes : Vec<plane::limited_plane>,
}


impl hittable_list
{


    pub fn hit(&mut self, r : ray::Ray, ray_tmin : f64, ray_tmax : f64, mut record : &mut  hittable::hitRecord) -> bool
    {
        let mut temp_record = hittable::hitRecord { p: vec3::Vec3 { x: (0.), y: (0.), z: (0.) }, normal: vec3::Vec3 { x: (0.), y: (0.), z: (0.) }, t: 0., font_face: false };
        
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;


        // Spheres iteration
        for i in &self.spheres
        {
            if i.hit(r, ray_tmin, closest_so_far, & mut temp_record)  
            {
                    let mut ptn = temp_record;
                    hit_anything = true;
                    closest_so_far = ptn.t;
                    *record = temp_record;
            }
        }

        for i in &self.planes
        {
            if i.hit(r, ray_tmin, closest_so_far, & mut temp_record)  
            {
                    let mut ptn = temp_record;
                    hit_anything = true;
                    closest_so_far = ptn.t;
                    *record = temp_record;
            }
        }

        for i in &self.finPlanes
        {   
            if i.hit(r, ray_tmin, closest_so_far, & mut temp_record)  
            {
                    let mut ptn = temp_record;
                    hit_anything = true;
                    closest_so_far = ptn.t;
                    *record = temp_record;
            }
        }
    
        

        return hit_anything; 

    }


}