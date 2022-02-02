use super::bitmaps::{Bitmap};

pub struct Font {
    pub bitmap: Bitmap,
    pub char_width: usize,
    pub char_height: usize,
    pub chars_per_line: usize,
    pub num_lines: usize
}

impl Font {
    fn obtain_char_coords(&self, num: u8, prev: u8) -> (usize, usize) {
        let num: usize = if prev == 0 {
            num as usize
        } else {
            //decode the polish characters
            match (num, prev) {

                //Ą 
                (196, 132) => 12*16 + 0,

                //ą 
                (196, 133) => 14*16 + 0,

                //Ć 
                (196, 134) => 12*16 + 7,

                //ć 
                (196, 135) => 14*16 + 7,

                //Ę 
                (196, 152) => 12*16 + 6,

                //ę 
                (196, 153) => 14*16 + 6,

                //Ł 
                (197, 129) => 12*16 + 10,

                //ł 
                (197, 130) => 14*16 + 10,

                //Ń 
                (197, 131) => 12*16 + 15,

                //ń 
                (197, 132) => 14*16 + 15,

                //Ó 
                (195, 147) => 13*16 + 1,

                //ó 
                (195, 179) => 15*16 + 1,

                //Ś 
                (197, 154) => 12*16 + 11,

                //ś 
                (197, 155) => 14*16 + 11,

                //Ż 
                (197, 187) => 13*16 + 8,

                //ż 
                (197, 188) => 15*16 + 8,

                //Ź 
                (197, 185) => 12*16 + 1,

                //ź 
                (197, 186) => 14*16 + 1,


                _ => '?' as usize

            }
        };
        let x = ((num as usize) * self.char_width) % self.bitmap.width;
        let y = ((num as usize) * self.char_width) / self.bitmap.width;
        (x, y*self.char_height)
    }
    
    pub fn draw_str_line(&self, target: &mut Bitmap, 
                         x: isize, y: isize, text: &str){
        let tx = (target.width/2) as isize + x;
        let ty = (target.height/2) as isize - y;
        let mut tx = if tx < 0 {0} else {tx as usize};
        let ty = if ty < 0 {0} else {ty as usize};

        let mut it = text.bytes(); 
        while let Some(byte) = it.next() {
            let prev = if byte > 128 {
                 it.next().unwrap()    
            } else {
                 0
            };
            
            
            let (sx, sy) = self.obtain_char_coords(byte, prev);
            self.bitmap.draw_rectangle_on(
                target, tx, ty, sx, sy,
                self.char_width, self.char_height);
            tx += self.char_width;
        }
        

        
    }


}
