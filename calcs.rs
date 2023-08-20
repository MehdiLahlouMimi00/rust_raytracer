pub fn calculate_image_height(iw: u32, ar : f64) -> u32
{
    let image_height_test : u32 = (iw/(ar as u32)) as u32; 
    let image_height : u32;

    if image_height_test > 1
    {
        image_height = image_height_test;
    }

    else 
    {
        image_height = 1;
    }

    return image_height;
}



pub fn calculate_viewport_width(vh : f64, iw : u32, ih : u32) -> f64
{
    return vh * ((iw / ih) as f64);
}