use crate::vec3;
use crate::interval;




pub fn linear_to_gamma(linera_component : f64) -> f64
{
    return linera_component.sqrt();
}






pub fn render_color(color: vec3::Vec3, spp : i32)
{
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1./(spp as f64);

    r *= scale;
    g *= scale;
    b *= scale;


    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);


    const INTERVAL :interval::interval = interval::interval {min : 0., max : 0.999};

    let rc = (INTERVAL.clamp(r) * 255.) as i32 ;
    let gc = (INTERVAL.clamp(g) * 255.) as i32 ;
    let bc = (INTERVAL.clamp(b) * 255.) as i32 ;

    println!("{} {} {}", rc, gc, bc);


    
}