use std::{io, ops::Mul};
use std::prelude::*;

use num::Complex;
use nannou::prelude::*;

use std::time::Instant;

use crate::{camera::GraphCamera, permutations};

const ROOT_RADIUS: f32 = 8.;
const RENDER_STEP: u64 = 30;

struct Model {
    roots: Vec<Complex<f32>>,
    colors_pallet: Vec<Rgb>,
    camera: GraphCamera,
    selected_root: Option<usize>,
    data: Vec<Vec<Complex<f32>>>,
    iterations: usize
}

pub fn init() {
    nannou::app(model)
        .view(view)
        .run();
}

fn random_color(mix: Rgb, strength: f32) -> Rgb {
    let red = random::<f32>();
    let green = random::<f32>();
    let blue = random::<f32>();

    Rgb::new(mix.red*strength+red*(1.-strength), mix.green*strength+green*(1.-strength), mix.blue*strength+blue*(1.-strength))
}

fn model(app: &App) -> Model {

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Couldn't read buffer.");
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
        let angle = x as f32/roots_count as f32*2.*PI+random::<f32>()-0.5;
        roots.push(Complex::new(angle.cos(), angle.sin()));
    }

    // x^3 - 1 = 0
    // roots = vec![Complex::new(1., 0.), Complex::new(-1./2., -3f32.sqrt()/2.), Complex::new(-1./2., 3f32.sqrt()/2.)];

    // fill pallete
    let mut colors_pallet = vec![];
    for _ in 0..roots_count {
        colors_pallet.push(random_color(Rgb::new(0.9, 0.9, 1.), 0.2));
    }
    
    let camera = GraphCamera { pos: Vec2::new(0., 0.), scale: Vec2::new(4., 3.) };
    let data = vec![vec![Complex::default(); win.h() as usize]; win.w() as usize];

    let mut model = Model { 
        roots,
        colors_pallet,
        camera, 
        selected_root: None, 
        data,
        iterations: 0
    };

    compute(win, &mut model);
    return model;
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
            model.roots[selected_root] = model.camera.complex(app.mouse.position(), win);
            model.selected_root = None;
            compute(win, model);
        }
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let win = app.main_window().rect();
    match key {
        Key::Left => {
            if model.iterations != 0 {
                model.iterations -= 1;
            }
        },
        Key::Right => {
            model.iterations += 1;
        },
        Key::Z => {
            model.camera.zoom(Vec2::new(0.3, 0.));
        },
        Key::X => {
            model.camera.zoom(Vec2::new(-0.3, 0.));
        },
        Key::C => {
            model.camera.zoom(Vec2::new(0., 0.3));
        },
        Key::V => {
            model.camera.zoom(Vec2::new(0., -0.3));
        },
        _ => return
    };
    compute(win, model);
}

fn compute(win: Rect, model: &mut Model) {
    let start = Instant::now();

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

    model.data = vec![vec![Complex::default(); win.h() as usize]; win.w() as usize];

    for x in 0..model.data.len() {
        for y in 0..model.data[x].len() {
            let pos = Vec2::new(x as f32, y as f32) - win.wh()/2.;
            model.data[x][y] = model.camera.complex(pos, win);
            for _ in 0..model.iterations {
                let z = model.data[x][y];
                model.data[x][y] = z-mfunc(z)/dmfunc(z);
            }
        }
    }

    println!("Computation took: {:?}.", start.elapsed());
}   

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.main_window().rect();

    for x in 0..model.data.len() {
        let part = (app.elapsed_frames()%RENDER_STEP) as f32/RENDER_STEP as f32;
        let start = part*model.data[x].len() as f32;
        let start = start as usize;
        for y in start..start+model.data[x].len()/RENDER_STEP as usize {
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
            let max_offset = model.camera.complex(win.wh()/2., win); // half screen
            let max_distance = (max_offset.re.pow(2.)+max_offset.im.pow(2.)).sqrt();
            let min_distance = min_distance.min(max_distance);

            let shade_strength = 0.4;
            let cmp = (1.-min_distance/max_distance).pow(3.)*shade_strength+1.-shade_strength;

            let color = rgb(model.colors_pallet[i].red*cmp, model.colors_pallet[i].green*cmp, model.colors_pallet[i].blue*cmp);

            draw.rect()
                .xy(Vec2::new(x as f32, y as f32)-win.wh()/2.)
                .w(1.)
                .h(1.)
                .color(color);
        }
    }

    for (i, root) in model.roots.iter().enumerate() {
        let pos = model.camera.real_vec(Vec2::new(root.re, root.im), win);

        let color = 
            if model.selected_root.is_some() && model.selected_root.unwrap() == i {
                Rgb::new(0.8, 0.8, 0.8)
            }
            else {
                Rgb::new(0.2, 0.2, 0.2)
            };

        draw.ellipse()
            .xy(pos)
            .radius(ROOT_RADIUS)
            .color(color)
            .stroke_weight(2.)
            .stroke(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
