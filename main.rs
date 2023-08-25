use std::f32::INFINITY;

use crate::{vec3::Vec3, ray::Ray};

use rand::prelude::*;


mod vec3;
mod color;
mod ray;
mod calcs;
mod hittable;
mod hittable_list;
mod camera;
mod interval;
mod utils;
mod plane;


#[allow(unused_variables)]
fn main() {

    // World
    let sphere2 : hittable::Sphere = hittable::Sphere {center : Vec3 { x: 0., y: 0., z: -1. }, radius : 0.5};
    let sphere1 : hittable::Sphere= hittable::Sphere {center : Vec3 { x: 0., y: -100.5, z: -1. }, radius : 100.};

    let plane1 : plane::plane = plane::plane {point : Vec3 { x: 0., y: 0., z: -1. }, normal:Vec3 { x: 0., y: 0., z: -0.5 }};


    let points = vec![Vec3 {x :0., y :0., z : -1.}, Vec3 {x :0., y :1., z : -1.}, Vec3 {x :2., y :1., z : -1.}, Vec3 {x :2., y :0., z : -1.}];
    let finPlane = plane::limited_plane::new__square(points);

    let mut world : hittable_list::hittable_list = hittable_list::hittable_list { spheres: vec![ ], planes : vec![] , finPlanes : vec![finPlane]};

    let camera = camera::Camera {aspect_ratio : 16./9., image_width : 400, samples_per_pixel : 100, center : vec3::Vec3 {x : 0., y : 0., z : 1.}, max_depth : 50};
    camera.render(&mut world);
    
}


