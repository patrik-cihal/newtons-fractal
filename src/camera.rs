use nannou::prelude::*;
use num::complex::Complex;

pub struct GraphCamera {
    pub pos: Vec2,
    pub scale: Vec2
}

impl GraphCamera {
    pub fn real_vec(&self, pos: Vec2, win: Rect) -> Vec2 {
        Vec2::new(self.real_x(pos.x, win), self.real_y(pos.y, win))   
    }
    pub fn real_y(&self, y: f32, win: Rect) -> f32 {
        (y-self.pos.y)/self.scale.y*win.h()
    }
    pub fn real_x(&self, x: f32, win: Rect) -> f32 {
        // we need to find a real position of x position by decreasing our pos shift dividing it by scale and multiplying by window width
        (x-self.pos.x)/self.scale.x*win.w()
    }
    pub fn virt_vec(&self, pos: Vec2, win: Rect) -> Vec2 {
        pos/win.wh()*self.scale
    }
    pub fn complex(&self, pos: Vec2, win: Rect) -> Complex<f32> {
        let vv = self.virt_vec(pos, win);
        Complex::new(vv.x, vv.y)
    }
    pub fn translate(&mut self, offset: Vec2) {
        self.pos += Vec2::new(offset.x*self.scale.x, offset.y*self.scale.y);
    }
    pub fn zoom(&mut self, offset: Vec2) {
        self.scale += self.scale*offset;
    }
}