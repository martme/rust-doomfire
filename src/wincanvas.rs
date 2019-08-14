
use std::ptr::null_mut;
use std::mem::size_of;
use winapi::shared::windef::{ 
    HBITMAP, 
    HDC, 
    HGDIOBJ, 
    HWND, 
};

use winapi::shared::minwindef::LPVOID;
use winapi::um::wingdi::{
    CreateCompatibleBitmap, 
    CreateCompatibleDC, 
    DC_BRUSH,
    DC_PEN,
    DeleteDC, 
    GetStockObject,
    DeleteObject, 
    SelectObject,
    SetDIBits,
    DIB_RGB_COLORS,
    LPBITMAPINFO,
    RGBQUAD,
    BITMAPINFOHEADER,
    BITMAPINFO,
    BI_RGB
};
use winapi::um::winuser::GetDC;


#[derive(Debug)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    
    dc: HDC,
    mem_dc: HDC,
    bmp: HBITMAP,
    canvas: HGDIOBJ,

    brush: HGDIOBJ,
    pen: HGDIOBJ,
}

#[derive(Debug)]
pub struct MonitorInfo {
    pub width: i32,
    pub height: i32
}

impl Canvas {

    pub fn detect_monitor() -> MonitorInfo {
        use winapi::um::wingdi::{ GetDeviceCaps, HORZRES, VERTRES };

        let dc: HDC = unsafe { GetDC(null_mut() as HWND) };
        let device_width: i32 = unsafe { GetDeviceCaps(dc, HORZRES) };
        let device_height: i32 = unsafe { GetDeviceCaps(dc, VERTRES) };

        MonitorInfo {
            width: device_width,
            height: device_height,
        }
    }

    pub fn from(width: i32, height: i32) -> Canvas {

        let dc: HDC = unsafe { GetDC(null_mut() as HWND) };
        let mem_dc: HDC = unsafe { CreateCompatibleDC(dc) };
        let bmp: HBITMAP = unsafe { CreateCompatibleBitmap(dc, width, height) }; 
        let canvas: HGDIOBJ = unsafe { SelectObject(mem_dc, bmp as HGDIOBJ) };

        let brush: HGDIOBJ = unsafe {
            let brush: HGDIOBJ = GetStockObject(DC_BRUSH as i32);
            SelectObject(mem_dc, brush)
        };
        let pen: HGDIOBJ = unsafe {
            let pen: HGDIOBJ = GetStockObject(DC_PEN as i32);
            SelectObject(mem_dc, pen)
        };

        Canvas { 
            width: width,
            height: height,
            dc: dc, 
            mem_dc: mem_dc, 
            bmp: bmp,
            canvas: canvas, 
            brush: brush,
            pen: pen,
        }
    }

    fn get_bitmapinfo(&self) -> BITMAPINFO {
        BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: self.width,
                biHeight: self.height * -1,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0
            },
            bmiColors: [RGBQUAD{
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0
            }; 1]
        }
    }

    pub fn set_raw_pixels(&self, buff: &Box<[u8]>, scale: i32) -> () {
        use winapi::um::wingdi::{ StretchBlt, SRCCOPY };
        let mut info = self.get_bitmapinfo();
        let info_ptr = &mut info as LPBITMAPINFO;
        unsafe {
            SetDIBits(self.dc, self.bmp, 0, self.height as u32, buff.as_ptr() as LPVOID, info_ptr, DIB_RGB_COLORS);
            StretchBlt(self.dc, 0, 0, self.width*scale, self.height*scale, self.mem_dc, 0, 0, self.width, self.height, SRCCOPY);
        }
    }

    pub fn dispose(self) {
        unsafe {
            DeleteDC(self.mem_dc);
            DeleteObject(self.canvas);
            DeleteObject(self.brush);
            DeleteObject(self.pen);
        }
    }
}
