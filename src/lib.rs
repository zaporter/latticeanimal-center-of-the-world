use std::f64;
use image::DynamicImage;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::ImageData;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
static mut image_option : Option<DynamicImage> = None;

#[wasm_bindgen(start)]
pub fn start() {
    console_log!("Beginning Loading Image");
    let image_bytes = include_bytes!("blue_marble_1.png");
    let img = image::load_from_memory_with_format(image_bytes,image::ImageFormat::Png).map_err(|e| e.to_string()).expect("Failed to load image from memory");
    console_log!("Image Dimensions: {} {}", img.width(), img.height());
    
    unsafe{image_option = Some(img);}

    console_log!("Finished Loading image");
}
#[wasm_bindgen]
pub fn draw(d_long_deg : f32, d_lat_deg: f32, zoom : f32){

    console_log!("Beginning redrawing");
    // let image_bytes = include_bytes!("blue_marble_1.png");
    // let img = image::load_from_memory_with_format(image_bytes,image::ImageFormat::Png).map_err(|e| e.to_string()).expect("Failed to load image from memory");
    let img = unsafe{image_option.as_ref().unwrap()};
    
    console_log!("Image Dimensions: {} {}", img.width(), img.height());
    
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


    console_log!("Canvas dimensions : {} {}", canvas.width(), canvas.height());

    let width = canvas.width();
    let height = canvas.height();
    let mut d_long = d_long_deg.to_radians();
    let mut d_lat = d_lat_deg.to_radians();
    if d_long < 0. {
        d_long += 6.28;
    }
    let mut data = get_flat_earth_data(d_long,d_lat,zoom,width, &img);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height).expect("Failed creating image data");
    context.put_image_data(&data, 0., 0.).expect("Failed to put the data on the canvas");
    console_log!("Finished Drawing");
}

fn get_scaled_image_data(width:u32, height:u32, image : &DynamicImage) -> Vec<u8>{
    let original_width = image.width();
    let original_height = image.height();
    let rgbimage = image.to_rgb8();
    let mut data = Vec::new();
    for y in 0..height{
        for x in 0..width{

            let sample_x = (x as f32 / width as f32) * (original_width as f32);
            let sample_y = (y as f32 / height as f32) * (original_height as f32);
            let sample_rgb = rgbimage.get_pixel(sample_x as u32,sample_y as u32);
        
            data.push(sample_rgb.0[0]);
            data.push(sample_rgb.0[1]);
            data.push(sample_rgb.0[2]);
            data.push(255);
        }
    }
    data
}
fn get_flat_earth_data(d_long : f32, d_lat : f32, zoom : f32, image_diameter:u32, image : &DynamicImage) -> Vec<u8>{
    let sample_width = image.width();
    let sample_height = image.height();
    let rgbimage = image.to_rgb8();
    let mut data = Vec::new();
    for raw_y in 0..image_diameter{
        for raw_x in 0..image_diameter{
            let raw_center = image_diameter as f32 / 2.;
            let x = (raw_x as f32 - raw_center) / raw_center;
            let y = -(raw_y as f32 - raw_center) / raw_center;
            let vec_mag = ((x*x+ y*y) as f32).sqrt();
            // if we are inside the ring
            if vec_mag < 1.0{
                
                let c = vec_mag;
                let ccos = (c*3.14*zoom).cos();
                let csin = (c*3.14*zoom).sin();

                let mut latitude = y * csin * d_lat.cos();
                latitude /= c;
                latitude += ccos * d_lat.sin();
                latitude = latitude.asin();

                let mut longitude = x * csin;
                //longitude /= (c * d_lat.cos() * c.cos()) - (y * d_lat.sin() * c.sin());
                //longitude = longitude.atan();
                longitude = longitude.atan2((c * d_lat.cos() * ccos) - (y * d_lat.sin() * csin));
                longitude += d_long;
                 latitude %= 3.14;
                
                 longitude += 3.14;
                 longitude %= 6.28;

                let mut sample_x = sample_width  as f32 * ((longitude )/ 6.28);

                let mut sample_y = sample_height as f32 * ((latitude - 3.14/2.) / -3.14);
                sample_x = sample_x.clamp(0., sample_width as f32 - 1.0);
                sample_y = sample_y.clamp(0., sample_height as f32 - 1.0);
                let sample_rgb = rgbimage.get_pixel(sample_x as u32,sample_y as u32);
                data.push(sample_rgb.0[0]);
                data.push(sample_rgb.0[1]);
                data.push(sample_rgb.0[2]);
                data.push(255);
            }else{
                data.push(0);
                data.push(0);
                data.push(0);
                data.push(0);

            }
        }
    }
    data
}
