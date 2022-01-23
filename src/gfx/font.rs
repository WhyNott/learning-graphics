use super::bitmaps::{Bitmap};

pub struct Font {
    pub bitmap: Bitmap,
    pub char_width: usize,
    pub char_height: usize,
    pub chars_per_line: usize,
    pub num_lines: usize
}

impl Font {
    fn obtain_char_coords(&self, num: u8) -> (usize, usize) {
        let x = ((num as usize) * self.char_width) % self.char_height;
        let y = ((num as usize) * self.char_width) / self.char_height;
        //println!("{} {}", x, y);
        (x, y)
    }
    
    pub fn draw_str_line(&self, target: &mut Bitmap, 
                         x: isize, y: isize, text: &str){
        let tx = (target.width/2) as isize + x;
        let ty = (target.height/2) as isize - y;
        let mut tx = if tx < 0 {0} else {tx as usize};
        let ty = if ty < 0 {0} else {ty as usize};

        for byte in text.bytes() {
            let (sx, sy) = self.obtain_char_coords(byte);
        //    let (sx, sy) = (0, 0);
            self.bitmap.draw_rectangle_on(
                target, tx, ty, sx, sy,
                self.char_width, self.char_height);
            tx += self.char_width;
        }

        
    }


}
