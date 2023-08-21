use crate::vec3;

// Struct 
#[derive(Clone, Copy)]
pub struct Ray 
{
    pub orig : vec3::Vec3,
    pub direc : vec3::Vec3
}


impl Ray
{
    

    pub fn origin(self) -> vec3::Vec3
    {
        return self.orig;
    }

    pub fn direction(self) -> vec3::Vec3
    {
        return self.direc;
    }

    pub fn _at(self, t : f64) -> vec3::Vec3
    {
        return self.orig + self.direc*t;
    }
}