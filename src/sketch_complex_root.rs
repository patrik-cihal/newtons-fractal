use std::default;

use num::Complex;
use nannou::prelude::*;

pub fn init() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.main_window().rect();

    let polynomials = vec![[Complex::new(5., 0.), Complex::new(3., 0.)], [Complex::new(-1., 2.), Complex::new(0., 0.)]];

    let mfunc = |x: Complex<f32>| -> Complex<f32> {
        let mut result = Complex::default();
        polynomials.iter().for_each(|pol| {
            result += pol[0]*x.pow(pol[1]);
        });
        result
    };

    let dmfunc = |x: Complex<f32>| -> Complex<f32> {
        let mut result = Complex::default();
        polynomials.iter().for_each(|pol| {
            result += pol[0]*pol[1]*x.pow(pol[1]-1.);
        });
        result
    };

    let scale = Vec2::new(1., 1.);
    let mut roots = Vec::<Complex<f32>>::new();
    let mut colors = vec![];

    let acc = 0.0001;

    for real_x in 0..win.w() as i32 {
        for real_y in 0..win.h() as i32 {

            let x = (real_x as f32/win.w()-0.5)*scale.x;
            let y = (real_y as f32/win.h()-0.5)*scale.y;

            // apply newtons method until near root
            let mut z = Complex::new(x, y);
            while abs(mfunc(z).im) > acc || abs(mfunc(z).re) > acc {
                z = z-mfunc(z)/dmfunc(z);
            }


            let mut found_root = roots.len();
            for i in 0..roots.len() {
                if abs(z.re-roots[i].re) < acc*2. && abs(z.im-roots[i].im) < acc*2. {
                    found_root = i;
                    break;
                }
            }

            if found_root == roots.len() {
                roots.push(z);
                let color = Rgb::new(random::<f32>(), random::<f32>(), random::<f32>());
                colors.push(color);
            }

            draw.rect()
                .x(real_x as f32-win.w()/2.)
                .y(real_y as f32-win.h()/2.)
                .color(colors[found_root]);
        }
        println!("{real_x}");
    }
    draw.to_frame(app, &frame).unwrap();
}