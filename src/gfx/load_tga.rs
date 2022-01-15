use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use super::colors::{Color, from_u8_rgb, from_u8_rgba};
use super::bitmaps::{Bitmap};

pub fn load_bitmap_from_tga<P: AsRef<Path>>(filename: P) -> io::Result<Bitmap> {
    let mut f = File::open(filename)?;
    let mut header = vec![0u8; 18];
    f.read_exact(&mut header)?;

    let width = ((header[13] as u16) << 8 | header[12] as u16) as usize;
    let height = ((header[15] as u16) << 8 | header[14] as u16) as usize;
    let bpp = header[16] as usize >> 3;
    let image_type = header[2];

    let mut data = vec![0u32; width*height];
    if image_type == 2 {
        
        let mut pixel_data = vec![0; bpp];
        for item in &mut data {
            f.read_exact(&mut pixel_data)?;
            *item = from_u8_rgb(
                pixel_data[2],
                pixel_data[1],
                pixel_data[0]
            );
        }
        
            
        
    } else if image_type == 10 {
        let mut index = 0;
        while index < width*height {
            let mut datapiece = [0;1];
            f.read_exact(&mut datapiece)?;
            let packet = datapiece[0];
            if packet >> 7 == 1 {
                let repetitions = packet - 128 + 1;
                let mut pixel_colors = vec![0; bpp];
                f.read_exact(&mut pixel_colors)?;
                for _ in 0..repetitions{
                    if bpp == 3 {
                        data[index] = from_u8_rgb(
                            pixel_colors[2],
                            pixel_colors[1],
                            pixel_colors[0]
                        );
                    } else {
                        //has alpha channel
                        data[index] = from_u8_rgba(
                            pixel_colors[2],
                            pixel_colors[1],
                            pixel_colors[0],
                            pixel_colors[3],
                        );
                    }
                    index += 1;
                }
            } else {
                let mut pixel_data = vec![0; bpp];
                for _ in 0..(packet + 1) {
                    f.read_exact(&mut pixel_data)?;
                    if bpp == 3 {
                        data[index] = from_u8_rgb(
                            pixel_data[2],
                            pixel_data[1],
                            pixel_data[0]
                        );
                    } else {
                        //has alpha channel
                        data[index] = from_u8_rgba(
                            pixel_data[2],
                            pixel_data[1],
                            pixel_data[0],
                            pixel_data[3],
                        );
                    }
                    index += 1;
                }

                
            }
        }
    } else {
        unimplemented!()
    }
    if header[17] >> 4 == 0 {
        for h in 0..height{
            let mut line = &mut data[h*width..(h+1)*width];
            line.reverse();
        }
        data.reverse();

        
    }

    
    
    Ok(Bitmap {width, height, data})

}
