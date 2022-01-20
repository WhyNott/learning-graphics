use super::colors::{Color, from_rgba_u8, from_u8_rgba};

pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>
}

impl Bitmap {
    pub fn draw_rectangle_on(&self, target: &mut Bitmap, tx: usize, ty: usize, sx: usize, sy: usize,  sw: usize, sh: usize){
        //(lets assume that y coord increases as you go downward)

        
        for i in 0..sh{
            
            let start = (ty+i)*target.width + tx;
            let end = if target.data.len() <= start+sw {
                target.data.len()
            } else if start+sw > (ty+i+1)*target.width {
                (ty+i+1)*target.width
            }
            else {
                start+sw
            };
            
            if start < end {
                let line = &mut target.data[start..end];
                for (j, pixel) in line.iter_mut().enumerate(){
                    let source_color = self.data[(i+sy)*sw+j+sx];
                    if ((source_color >> 24)&255) == 255 {
                        *pixel = source_color;
                    } else if ((source_color >> 24)&255) != 0 {
                        
                        //hadling transparency
                        let (sr, sg, sb, sa) = from_rgba_u8(source_color);
                        let (tr, tg, tb, ta) = from_rgba_u8(*pixel);

                        let (sr, sg, sb, sa) = (
                            (sr as f64 ),
                            (sg as f64 ),
                            (sb as f64 ),
                            (sa as f64 /255.0),
                        );

                        let (tr, tg, tb, ta) = (
                            (tr as f64 ),
                            (tg as f64 ),
                            (tb as f64 ),
                            (ta as f64 /255.0),
                        );
                        
                        
                        let oa = sa + ta*(1.0-sa);
                        let or = (sr*sa + tr*ta*(1.0-sa))/oa;
                        let og = (sg*sa + tg*ta*(1.0-sa))/oa;
                        let ob = (sb*sa + tb*ta*(1.0-sa))/oa;
                        
                        *pixel = from_u8_rgba(
                            (or) as u8,
                            (og) as u8,
                            (ob) as u8,
                            (oa*255.0) as u8
                        );
                        
                        
                        
                    }
                    
                }
            }
        }
        
    }
    

    pub fn draw_on(&self, target: &mut Bitmap, x: isize, y: isize){
        
        let tx = (target.width/2) as isize + x;
        let ty = (target.height/2) as isize - y;
        let tx = if tx < 0 {0} else {tx as usize};
        let ty = if ty < 0 {0} else {ty as usize};
        
        self.draw_rectangle_on(target, tx, ty, 0, 0, self.width, self.height);

        
    }
    
}
