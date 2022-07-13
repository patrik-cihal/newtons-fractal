use std::f32::consts::E;

use nannou::prelude::*;
use crate::camera::GraphCamera;

use rand::random;

struct Model {
    mfunc: fn(f32)->f32,
    camera: GraphCamera,
    tangents: Vec<f32>,
    last_mouse: Option<Vec2>,
}

pub fn init() {
    nannou::app(model)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();


    fn quadratic(x: f32) -> f32 {
        return (x-2.)*(x-0.8)*(x+0.5);
    }

    let camera = GraphCamera {
        pos: Vec2::new(0., 0.),
        scale: Vec2::new(5., 15.)
    };
    
    Model {
        mfunc: quadratic,
        camera,
        tangents: vec![random::<f32>()*5.-2.5],
        last_mouse: None
    }
}

fn mouse_pressed(app: &App, model: &mut Model, btn: MouseButton) {
    if btn == MouseButton::Left {
        model.last_mouse = Some(app.mouse.position());
    }
}

fn mouse_moved(app: &App, model: &mut Model, mouse_position: Vec2) {
    if let Some(last_mouse) = model.last_mouse {
        model.camera.translate((last_mouse-mouse_position)/app.main_window().rect().wh());
        model.last_mouse = Some(mouse_position);
    }
}

fn mouse_released(app: &App, model: &mut Model, btn: MouseButton) {
    if btn == MouseButton::Left {
        model.last_mouse = None;
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            // calculate where next tangent should be
            let x = model.tangents.last().unwrap().to_owned();

            // y = ax+b
            let (a, b) = tangent(&model.mfunc, x);

            // 0 = ax+b
            // x = -b/a
            model.tangents.push(-b/a);
        },
        Key::Z => {
            model.camera.zoom(Vec2::new(0.1, 0.));
        },
        Key::X => {
            model.camera.zoom(Vec2::new(-0.1, 0.));
        },
        Key::C => {
            model.camera.zoom(Vec2::new(0., 0.1));
        },
        Key::V => {
            model.camera.zoom(Vec2::new(0., -0.1));
        },
        _ => ()
    }
}

fn tangent(mfunc: &fn(f32) -> f32, x: f32) -> (f32, f32) {
    let y = mfunc(x);

    let a = (mfunc(x+0.005)-y)/0.005;
    let b = y-a*x;

    (a, b)
}

fn order_10(a: f32) -> f32 {
    let mut result = 32.;
    while a < 10f32.pow(result) {
        result -= 1.;
    }
    return result;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.main_window().rect();

    draw.background().color(Rgb::new(0.9, 0.9, 0.9));

    // x-axis
    draw.line()
        .start(Vec2::new(-win.w()/2., model.camera.real_y(0., win)))
        .end(Vec2::new(win.w()/2., model.camera.real_y(0., win)))
        .color(Rgb::new(0.7, 0.7, 1.))
        .stroke_weight(5.);

    // y-axis
    draw.line()
        .start(Vec2::new(model.camera.real_x(0., win), -win.h()/2.))
        .end(Vec2::new(model.camera.real_x(0., win), win.h()/2.))
        .color(RED)
        .stroke_weight(5.);

    // grid
    let mut x = 0.;
    let o = order_10(model.camera.scale.x)-1.;
    let grid_line = |start: Vec2, end: Vec2| {
        draw.line()
            .start(start)
            .end(end)
            .stroke_weight(1.)
            .color(rgb(0.7, 0.7, 0.7));
    };
    while model.camera.real_x(x, win)<win.w()/2. {
        x += 10f32.pow(o);
        grid_line(Vec2::new(model.camera.real_x(x, win), -win.h()/2.),
            Vec2::new(model.camera.real_x(x, win), win.h()/2.));
    }
    x = 0.;
    while model.camera.real_x(x, win)>-win.w()/2. {
        x -= 10f32.pow(o);
        grid_line(Vec2::new(model.camera.real_x(x, win), -win.h()/2.), 
            Vec2::new(model.camera.real_x(x, win), win.h()/2.));
    }

    let o = order_10(model.camera.scale.y)-1.;
    let mut y = 0.;
    while model.camera.real_y(y, win)<win.h()/2. {
        y += 10f32.pow(o);
        grid_line(Vec2::new(-win.w()/2., model.camera.real_y(y, win)), 
            Vec2::new(win.w()/2., model.camera.real_y(y, win)));
    }
    y = 0.;
    while model.camera.real_y(y, win)>-win.h()/2. {
        y -= 10f32.pow(o);
        grid_line(Vec2::new(-win.w()/2., model.camera.real_y(y, win)), 
            Vec2::new(win.w()/2., model.camera.real_y(y, win)));
    }

    // tangents
    for x in &model.tangents {
        // find out tangent f(x)
        let (a, b) = tangent(&model.mfunc, *x);
        let x0 = model.camera.pos.x-model.camera.scale.x/2.;
        let y0 = a*x0+b;
        let x1 = model.camera.pos.x+model.camera.scale.x/2.;
        let y1 = a*x1+b;

        let start_pos = model.camera.real_vec(Vec2::new(x0, y0), win);
        let end_pos = model.camera.real_vec(Vec2::new(x1, y1), win);
        draw.line()
            .start(start_pos)
            .end(end_pos)
            .color(Rgb::new(0.5, 0.5, 0.5))
            .stroke_weight(2.);
    }

    // draw function curve
    let points = 1000;
    for i in 1..points {
        let x0 = model.camera.pos.x+(((i-1) as f32/(points-1) as f32) as f32 - 0.5)*model.camera.scale.x;
        let y0 = (model.mfunc)(x0);
        let x1 = model.camera.pos.x+(((i as f32)/(points-1) as f32) - 0.5)*model.camera.scale.x;
        let y1 = (model.mfunc)(x1);

        let start_pos = model.camera.real_vec(Vec2::new(x0, y0), win);
        let end_pos = model.camera.real_vec(Vec2::new(x1, y1), win);

        draw.line()
            .start(start_pos)
            .end(end_pos)
            .color(BLACK)
            .stroke_weight(3.);
    }

    // draw approximated root
    let last_tangent = model.tangents.last().unwrap().clone();
    let (a, b) = tangent(&model.mfunc, last_tangent);
    let x = -b/a;
    draw.ellipse()
        .xy(model.camera.real_vec(Vec2::new(x, 0.), win))
        .radius(5.)
        .color(BLACK);

    // push drawing to frame
    draw.to_frame(app, &frame).unwrap();
}