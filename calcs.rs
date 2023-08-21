pub fn calculate_image_height(iw: i32, ar : f64) -> i32
{
    let image_height_test : i32 = ((iw as f64)/ar) as i32; 
    let image_height = if image_height_test < 1 {1} else {image_height_test};

    return image_height;
}



pub fn calculate_viewport_width(vh : f64, iw : i32, ih : i32) -> f64
{
    return vh * ( (iw as f64) / (ih as f64) ) ;
}