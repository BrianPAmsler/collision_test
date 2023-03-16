mod polygon;
mod vectors;
use polygon::Polygon;
use raqote::*;
use minifb::{MouseMode, Window, WindowOptions};
use vectors::Vector2;

fn draw_line(dt: &mut DrawTarget, line: &(Vector2, Vector2), color: (u8, u8, u8, u8)) {
    let mut pb = PathBuilder::new();

    pb.move_to(line.0.x, line.0.y);
    pb.line_to(line.1.x, line.1.y);
    let path = pb.finish();
    
    dt.stroke(
        &path,
        &Source::Solid(SolidSource {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        }),
        &StrokeStyle {
            width: 2.0,
            cap: LineCap::Round,
            join: LineJoin::Round,
            miter_limit: 0.0,
            dash_array: Vec::new(),
            dash_offset: 0.0,
        },
        &DrawOptions::new()
    );
}

fn draw_shape(dt: &mut DrawTarget, shape: &Polygon, color: (u8, u8, u8, u8)) {
    for line in shape.get_edges() {
        draw_line(dt, &line, color);
    }
}

fn main(){
    let mut window = Window::new("Raqote", 400, 400, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();

    let mut dt = DrawTarget::new(400, 400);

    let shape = Polygon::new((100.0, 100.0).into(), vec![(-20.0, 20.0).into(), (20.0, 20.0).into(), (20.0, -20.0).into(), (-20.0, -20.0).into(), (-40.0, 0.0).into()]);
    let mut shape2 = Polygon::new((125.0, 125.0).into(), vec![(-20.0, 20.0).into(), (20.0, 20.0).into(), (40.0, 0.0).into(),(20.0, -20.0).into(), (-20.0, -20.0).into()]);

    while window.is_active() {
        dt.clear(SolidSource { r: 0, g: 0, b: 0, a: 0xff });
        let mouse = window.get_mouse_pos(MouseMode::Pass).unwrap();

        shape2.center = mouse.into();
    
        draw_shape(&mut dt, &shape, (0xff, 0x0, 0x0, 0xff));
        draw_shape(&mut dt, &shape2, (0x0, 0x0, 0xff, 0xff));
    
        let collision = shape2.collide(&shape);
    
        match collision {
            Some(push) => {
                let mut shape3 = shape2.clone();
                shape3.center += push;
    
                draw_shape(&mut dt, &shape3, (0x0, 0xff, 0x0, 0xff));
            },
            None => ()
        }
        window.update_with_buffer(dt.get_data(), dt.width() as usize, dt.height() as usize).unwrap();
    }
}