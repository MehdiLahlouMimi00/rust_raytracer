// I will implement the stuff the more I go
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;


#[derive(Clone, Copy)]
pub struct Vec3
{
    pub x : f64, 
    pub y : f64,
    pub z : f64
}


impl Add for Vec3
{
    // Addition operator
    type Output = Vec3;

    fn add(self, other:Vec3) -> Vec3
    {
        Vec3 {x: self.x + other.x, y : self.y + other.y, z: self.z + other.z}
    }
}

impl Mul<f64> for Vec3
{
    // Multiplication operator
    type Output = Vec3;

    fn mul(self, other:f64) -> Vec3
    {
        Vec3 { x: (other*self.x), y: (other*self.y), z: (other*self.z) }
    }
}


impl Div<f64> for Vec3
{
    // Division operator
    type Output = Vec3;

    fn div(self, other:f64) -> Vec3
    {
        Vec3 { x: (self.x/other), y: (self.y/other), z: (self.z/other) }
    }
}

impl Vec3
{
    pub fn length(self) -> f64
    {
        return (self.x*self.x + self.y*self.y + self.z*self.y).sqrt();
    }

    

}

// Useful functions
pub fn dot_product(u:Vec3, v:Vec3) -> f64
{
    return u.x*v.x + u.y*v.y + u.z*v.z;
}

pub fn clone_vector(u:Vec3) -> Vec3
{
    return Vec3 { x: (u.x), y: (u.y), z: (u.z) }
}

pub fn normalized_vector(u:Vec3) -> Vec3
{
    return u/u.length();
}