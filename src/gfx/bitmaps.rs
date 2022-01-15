use super::colors::{Color};

pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>
}

impl Bitmap {
    pub fn draw_on(&self, target: &mut Bitmap, x: isize, y:isize){
        let x = (target.width/2) as isize + x;
        let y = (target.height/2) as isize - y;

        //Finish this later, its kind of boring
        //Also, maybe it would have been better to have this draw only a
        // section of the bitmap? would have been more general.
        //let mut i_target = y*(target.width as isize) + x;
        //let mut i = 0;
        //for _ in 0..self.height {
        //    for _ in 0..self.width {
        //        target.data[
        //    }
        //} 
        
    }
}
