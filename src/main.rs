
use std::{thread, time};
use std::time::{SystemTime};
use winapi::shared::windef::COLORREF;
use winapi::um::wingdi::RGB;
mod wincanvas;


fn esc_pressed() -> bool {
    use winapi::um::winuser::{ GetKeyState, VK_ESCAPE };
    unsafe {
        GetKeyState(VK_ESCAPE) as u32 & 0x8000u32 != 0
    }
}

fn main() {
    let monitor = wincanvas::Canvas::detect_monitor();

    const SCALE: i32 = 6;
    let width: i32 = &monitor.width / SCALE;
    let height: i32 = &monitor.height / SCALE;
    let fire_width: usize = width as usize;

    let mut fire = vec![0u8; (width*height) as usize];
    let mut color_data: Box<[u8]> = (vec![0u8; (width*height*4) as usize]).into_boxed_slice();

    let canvas = wincanvas::Canvas::from(width, height);
    
    // set lower row to max temperature, temperature in 0..36
    for x in 0..width {
        let i = ((height - 1)*width + x) as usize;
        fire[i] = 36;
    }

    let start_time = SystemTime::now();
    loop {
        if SystemTime::now().duration_since(start_time).unwrap().as_secs() > 30 {
            break;
        }

        thread::sleep(time::Duration::from_millis(17));

        // exit on magic key combination
        if esc_pressed() {
            break;
        }
        // spread fire
        for x in 0..width {
            for y in 1..height {
                
                let src = (y*width + x) as usize;
                let pixel = fire[src];
                if pixel == 0 {
                    fire[src - fire_width] = 0;
                    color_data[4*src + 0] = 0u8;
                    color_data[4*src + 1] = 0u8;
                    color_data[4*src + 2] = 0u8;
                    color_data[4*src + 3] = 0xFF;
                }
                else {
                    let rand_idx = (rand::random::<u8>() & 3) as usize;
                    let dst = src - rand_idx + 1;

                    if dst >= fire_width {
                        let old_temp = fire[dst - fire_width];
                        let new_temp =  pixel - (rand_idx & 1) as u8;
                        fire[dst - fire_width] = new_temp;
                        if new_temp != old_temp {
                            let color = temp_to_color(new_temp) as u32;
                            color_data[4*src + 2] = color as u8;
                            color_data[4*src + 1] = (color >> 8) as u8;
                            color_data[4*src + 0] = (color >> 16) as u8;
                            color_data[4*src + 3] = 0xFF;
                        }
                    }
                }
            }
        }
        canvas.set_raw_pixels(&color_data, SCALE);
    }

    canvas.dispose();
}

fn temp_to_color(temperature: u8) -> COLORREF {
    match temperature {
        0x00 => RGB(0x07,0x07,0x07),
        0x01 => RGB(0x1F,0x07,0x07),
        0x02 => RGB(0x2F,0x0F,0x07),
        0x03 => RGB(0x47,0x0F,0x07),
        0x04 => RGB(0x57,0x17,0x07),
        0x05 => RGB(0x67,0x1F,0x07),
        0x06 => RGB(0x77,0x1F,0x07),
        0x07 => RGB(0x8F,0x27,0x07),
        0x08 => RGB(0x9F,0x2F,0x07),
        0x09 => RGB(0xAF,0x3F,0x07),
        0x0a => RGB(0xBF,0x47,0x07),
        0x0b => RGB(0xC7,0x47,0x07),
        0x0c => RGB(0xDF,0x4F,0x07),
        0x0d => RGB(0xDF,0x57,0x07),
        0x0e => RGB(0xDF,0x57,0x07),
        0x0f => RGB(0xD7,0x5F,0x07),
        0x10 => RGB(0xD7,0x5F,0x07),
        0x11 => RGB(0xD7,0x67,0x0F),
        0x12 => RGB(0xCF,0x6F,0x0F),
        0x13 => RGB(0xCF,0x77,0x0F),
        0x14 => RGB(0xCF,0x7F,0x0F),
        0x15 => RGB(0xCF,0x87,0x17),
        0x16 => RGB(0xC7,0x87,0x17),
        0x17 => RGB(0xC7,0x8F,0x17),
        0x18 => RGB(0xC7,0x97,0x1F),
        0x19 => RGB(0xBF,0x9F,0x1F),
        0x1a => RGB(0xBF,0x9F,0x1F),
        0x1b => RGB(0xBF,0xA7,0x27),
        0x1c => RGB(0xBF,0xA7,0x27),
        0x1d => RGB(0xBF,0xAF,0x2F),
        0x1e => RGB(0xB7,0xAF,0x2F),
        0x1f => RGB(0xB7,0xB7,0x2F),
        0x20 => RGB(0xB7,0xB7,0x37),
        0x21 => RGB(0xCF,0xCF,0x6F),
        0x22 => RGB(0xDF,0xDF,0x9F),
        0x23 => RGB(0xEF,0xEF,0xC7),
        0x24 => RGB(0xFF,0xFF,0xFF),
        _ => RGB(0xFF,0xFF,0xFF),
    }
}
