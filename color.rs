use crate::vec3;

pub fn render_color(color: vec3::Vec3)
{
    let ir: u32 = (255.999 * color.x) as u32;
    let ig: u32 = (255.999 * color.y) as u32;
    let ib : u32 = (255.999 * color.z) as u32;

    println!("{} {} {}", ir, ig, ib); 
}