use crate::ray;
use crate::vec3;


#[derive(Clone, Copy)]
pub struct hitRecord
{
    pub p : vec3::Vec3,
    pub normal : vec3::Vec3,
    pub t : f64,
    pub font_face : bool,

}

impl hitRecord
{
    pub fn set_face_normal(&mut self, r : ray::Ray, outward_normal : vec3::Vec3) -> ()
    {
        let front_face = vec3::dot_product(r.direction(), outward_normal) < 0.;
        

        if front_face
        {
            self.normal = outward_normal;
        }

        else
        {
            self.normal =  outward_normal * (-1.);
        }
    }
}


//////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy)]
pub struct Sphere
{
    pub center : vec3::Vec3,
    pub radius : f64

}

impl Sphere
{

    pub fn hit(self, r : ray::Ray, ray_tmin : f64, ray_tmax : f64,  record :  & mut hitRecord) -> bool
    {
        
        let oc = r.origin() - self.center;
    
        let a = r.direction().length() * r.direction().length() ;
        let half_b = vec3::dot_product(oc, r.direction());
        let c = vec3::dot_product(oc, oc) - self.radius*self.radius;
    
        
    
        let discriminant = half_b*half_b - a*c;
    
        if discriminant < 0.
        {
            return false;
        }
    
        else 
        {
            let discri_squarred = discriminant.sqrt();
            
            let mut root = -(half_b + discri_squarred)/a;
            
            if root <= ray_tmin || root >= ray_tmax
            {
                root = (-half_b + discri_squarred)/a;   
                
                if root <= ray_tmin || ray_tmax <= root
                {
                    return false;
                }
    
            }
    
            record.t = root;
            record.p = r.at(record.t);
            let outward_normal = (record.p - self.center)/self.radius;
            record.set_face_normal(r, outward_normal);
    
            return true;   
        }
    
    }

}