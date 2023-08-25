use crate::hittable::hitRecord;
use crate::vec3;
use crate::ray;
use crate::hittable;



/// Infinite plane
#[derive(Clone)]
pub struct plane
{
    // This class is just an interface, it is basically for a limited plane
    pub point : vec3::Vec3,
    pub normal : vec3::Vec3
}

impl plane
{
    pub fn hit(&self, r : ray::Ray, ray_tmin : f64, ray_tmax : f64, record : & mut hitRecord ) -> bool
    {
        let top = vec3::dot_product(self.point - r.orig, self.normal);
        let bot = vec3::dot_product(r.direc, self.normal);

        if bot > 1e-16
        {
            let t = top/bot;


            if  t >= 0.
            {

                record.p = self.point;
                record.t = t;
                record.normal = self.normal;

                return true;
            }

            else 
            {
                return false;    
            }
        }

        else 
        {
            return false;    
        }
    }
}


/// Limited plane
#[derive(Clone)]
pub struct limited_plane
{
    limit_points : Vec<vec3::Vec3>,
    large_plane : plane,
}

impl limited_plane
{
    pub fn new__square(points :Vec<vec3::Vec3> ) -> Self
    {
        // Basically we need to build a plane starting from the points 
        let point = points.last().unwrap();
        
        let A = points[0];
        let B = points[1];
        let C = points[2];


        let AB = B - A;
        let AC = C -A;
        
        let normal =vec3::normalized_vector(vec3::cross(&AB, &AC));
    
        return limited_plane { limit_points: points.clone(), large_plane: plane { point: *point, normal: normal } }
    
    }

    pub fn hit(&self, r : ray::Ray, t_min : f64, t_max : f64, record : & mut hitRecord) -> bool
    {
        if !self.large_plane.hit(r,t_min, t_max,  record)
        {
            return false;
        }

        else 
        {
            // If it hits the normal plane
            let top = vec3::dot_product(self.large_plane.point - r.orig, self.large_plane.normal);
            let bot = vec3::dot_product(r.direc, self.large_plane.normal);
            
            let t = top/bot;
            
            let pointOfCollision = r.at(t);

            // Detection of collision
            let A = self.limit_points[0];
            let B = self.limit_points[1];
            let C = self.limit_points[2];
            let D = self.limit_points[3];


            let AB = B - A;
            let AC = C -A;
            let AD = D -A;

            let AP = pointOfCollision - A;
            let u = vec3::dot_product(AP, AB)/vec3::dot_product(AB, AB);
            let v = vec3::dot_product(AP, AD)/vec3::dot_product(AD, AD);

            ///////////
            
            

            if u >= 1. || v >= 1. || u <= 0. || v <= 0. 
            {
                return false
            }
            
            else
            {
                record.p = self.large_plane.point;
                record.t = t;
                record.normal = self.large_plane.normal;
                return true;
            }

        }

    }
}