// I will implement the stuff the more I go
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;

use crate::utils;


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

impl Sub for Vec3 {
    
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3
    {
        Vec3 { x: (self.x - other.x), y: (self.y - other.y), z: (self.z - other.z) }
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
        return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
    }

    pub fn random_vector() -> Vec3
    {
        return Vec3 { x: utils::random(), y: utils::random() , z: utils::random() };
    }

    pub fn random_vector_between(min : f64, max : f64) -> Vec3
    {
        return Vec3 { x: utils::random_between(min, max), y: utils::random_between(min, max), z: utils::random_between(min, max) };
    }

}

// Useful functions
pub fn dot_product(u:Vec3, v:Vec3) -> f64
{
    return u.x*v.x + u.y*v.y + u.z*v.z;
}



pub fn normalized_vector(u:Vec3) -> Vec3
{
    return u/u.length();
}

pub fn random_in_unit_sphere() -> Vec3
{
    loop
    {
        let p = Vec3::random_vector_between(-1., 1.);
        if p.length() * p.length() < 1.
        {
            return p;
        }
    }
}


pub fn random_unit_vector() -> Vec3
{
    return normalized_vector(random_in_unit_sphere());
}

pub fn random_on_hemisphere(normal : Vec3) -> Vec3
{
    let on_unit_sphere = random_unit_vector();

    if dot_product(on_unit_sphere, normal) > 0.
    {
        return on_unit_sphere;
    }

    else
    {
        return on_unit_sphere* (-1.);
    }

}


pub fn cross(one : &Vec3, other: &Vec3) -> Vec3 {
    let cross_x = one.y * other.z - one.z * other.y;
    let cross_y = one.z * other.x - one.x * other.z;
    let cross_z = one.x * other.y - one.y * other.x;

    return Vec3 {x :cross_x, y : cross_y, z : cross_z}
}