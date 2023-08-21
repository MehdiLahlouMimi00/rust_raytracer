use crate::vec3;

pub fn render_color(color: vec3::Vec3)
{
    let ir: i32 = (255.999  * color.x) as i32;
    let ig: i32 = (255.999 * color.y) as i32;
    let ib : i32 = (255.999 * color.z) as i32;

    println!("{} {} {}", ir, ig, ib); 
}