use std::{io};

use num::Complex;
use nannou::prelude::*;

use crate::{camera::GraphCamera, permutations};

const ROOT_RADIUS: f32 = 8.;

struct Model {
    roots: Vec<Complex<f32>>,
    colors_pallet: Vec<Rgb>,
    camera: GraphCamera,
    selected_root: Option<usize>,
    data: Vec<Vec<Complex<f32>>>,
    redraw: bool
}

pub fn init() {
    nannou::app(model)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let roots_count = buffer.trim().parse::<i32>().unwrap();

    app.new_window()
        .key_pressed(key_pressed)
        .resizable(false)
        .size(1280, 720)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .build()
        .unwrap();

    let win = app.main_window().rect();

    let mut roots = Vec::default();
    // push roots in circular fashion
    for x in 0..roots_count {
        let angle = x as f32/roots_count as f32*PI+random::<f32>()-0.5;
        roots.push(Complex::new(angle.cos(), angle.sin()));
    }

    // roots = vec![Complex::new(1., 0.), Complex::new(-1./2., -3f32.sqrt()/2.), Complex::new(-1./2., 3f32.sqrt()/2.)];

    // fill pallete
    let mut colors_pallet = vec![];
    for _ in 0..roots_count {
        let r = random::<f32>();
        let g = random::<f32>();
        let b = random::<f32>();
        colors_pallet.push(Rgb::new(r, g, b));
    }
    
    let camera = GraphCamera { pos: Vec2::new(0., 0.), scale: Vec2::new(3., 3.) };
    let mut data = vec![vec![Complex::default(); win.h() as usize]; win.w() as usize];
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            let pos = Vec2::new(x as f32, y as f32)-win.wh()/2.;
            data[x][y] = camera.complex_vec(pos, win);
        }
    }


    return Model { 
        roots,
        colors_pallet,
        camera, 
        selected_root: None, 
        data,
        redraw: true
    };
}

fn mouse_pressed(app: &App, model: &mut Model, btn: MouseButton) {
    let win = app.main_window().rect();
    if btn == MouseButton::Left {
        for i in 0..model.roots.len() {
            let root_pos = model.camera.real_vec(Vec2::new(model.roots[i].re, model.roots[i].im), win);
            let distance = root_pos.distance(app.mouse.position());
            if distance < ROOT_RADIUS {
                model.selected_root = Some(i);
            }
        }
    }
}

fn mouse_released(app: &App, model: &mut Model, btn: MouseButton) {
    let win = app.main_window().rect();
    if btn == MouseButton::Left {
        if let Some(selected_root) = model.selected_root {
            model.roots[selected_root] = model.camera.complex_vec(app.mouse.position(), win);
            model.selected_root = None;
            for x in 0..win.w() as usize {
                for y in 0..win.h() as usize {
                    let pos = Vec2::new(x as f32, y as f32) - win.wh()/2.;
                    model.data[x][y] = model.camera.complex_vec(pos, win);
                }
            }
        }
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        compute_step(app, model);
    }
}

fn compute_step(_app: &App, model: &mut Model) {
    let mfunc = |x: Complex<f32>| -> Complex<f32> {
        model.roots.iter().map(|root| {
            x-root
        }).product()
    };

    // find the coefficients
    let n = model.roots.len();
    let mut coef = vec![Complex::default(); n+1];
    for i in 0..model.roots.len() {
        coef[i] = permutations(&model.roots, n-i)
            .iter().map(|x | {x.iter().product::<Complex<f32>>()*(-1f32).pow(x.len() as f32)})
            .sum();
    };
    coef[n] = Complex::<f32>::new(1., 0.);

    // get the derivative
    let dmfunc = |x: Complex<f32>| -> Complex<f32> {
        let mut result = Complex::default();
        for i in 1..coef.len() {
            result += coef[i] * i as f32 * x.pow((i-1) as f32);
        }
        result
    };

    for x in 0..model.data.len() {
        for y in 0..model.data[x].len() {
            let z = model.data[x][y];
            model.data[x][y] = z-mfunc(z)/dmfunc(z);
        }
    }
}   

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.main_window().rect();

    for x in 0..model.data.len() {
        for y in 0..model.data[x].len() {
            let z = model.data[x][y];
            let mut min_distance = f32::INFINITY;
            let mut i: usize = 0;
            for j in 0..model.roots.len() {
                let offset = model.roots[j]-z;
                let distance = (offset.re*offset.re+offset.im*offset.im).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    i = j;
                }
            }
            draw.rect()
                .xy(Vec2::new(x as f32, y as f32)-win.wh()/2.)
                .w(1.)
                .h(1.)
                .color(model.colors_pallet[i]);
        }
    }

    for root in &model.roots {
        let mut pos = model.camera.real_vec(Vec2::new(root.re, root.im), win);

        draw.ellipse()
            .xy(pos)
            .radius(ROOT_RADIUS)
            .color(Rgb::new(0.2, 0.2, 0.2))
            .stroke_weight(2.)
            .stroke(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}


