extern crate piston_window;
extern crate find_folder;
extern crate vecmath;

use piston_window::draw_state::Blend;
use piston_window::*;
use vecmath::Vector2;
use vecmath::Matrix3;
use vecmath::col_mat3_transform_vec2;
use std::f32::consts::PI;

use nbez::{BezChain,Point2d,Bez3o,BezCurve};

fn main() {
    println!("Press A to change blending");
    println!("Press S to change clip inside/out");

    let tween_vec = {
        let curve: Bez3o<f32> = Bez3o::new(
            Point2d::new(0.0    ,0.0),
            Point2d::new(0.06   ,0.74),
            Point2d::new(0.32   ,1.00),
            Point2d::new(1.0    ,1.0)
        );
        let curve_chain: BezChain<f32, Bez3o<f32>, Vec<Point2d<f32>>> = BezChain::from_container(vec![
            curve.start,
            curve.ctrl0,
            curve.ctrl1,
            curve.end ]);
        let mut res = vec![];
        for curve in curve_chain.iter() {
            let mut t = 0.0f32;
            let zl = 1.0f32 / 40.0f32;

            for _i in 0..40{
                res.push(curve.interp(t).unwrap());
                t += zl;
            }
        }
        let last = res.len() - 1;
        res[last].y = 1.0f32;
        res
    };

    let mut window: PistonWindow = WindowSettings::new(
            "piston: draw_state",
            [344, 344]
        )
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let blends = [Blend::Alpha, Blend::Add, Blend::Invert, Blend::Multiply];
    let mut blend = 0;
    let mut clip_inside = true;
    let rust_logo = Texture::from_path(&mut window.factory,
                                       assets.join("rust.png"),
                                       Flip::None,
                                       &TextureSettings::new()).unwrap();
    //window.set_lazy(true);
    let rust_4_1_w = rust_logo.get_width() as f32 / 4.0f32;
    let rust_4_1_h = rust_logo.get_height() as f32 / 4.0f32;
    let origin:Vector2<f32> = [100.0 + rust_4_1_w * 2.0,100.0 + rust_4_1_h * 2.0];
    let mut angle = 0f32;
    let base:Vector2<f32> = [rust_4_1_w,rust_4_1_h];

    let mut cur_i = 0usize;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c:Context, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
//            g.clear_stencil(0);
//            Rectangle::new([1.0, 0.0, 0.0, 1.0])
//                .draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);
//
//            let draw_state = c.draw_state.blend(blends[blend]);
//            Rectangle::new([0.5, 1.0, 0.0, 0.3])
//                .draw([50.0, 50.0, 100.0, 100.0], &draw_state, c.transform, g);

            let transform = c.transform.trans(100.0, 100.0);
            // Compute clip rectangle from upper left corner.
            let mat:Matrix3<f32> = [
                [angle.cos(),-angle.sin(),0.0],
                [angle.sin(), angle.cos(),0.0],
                [0.0,0.0,1.0]
            ];

            let mut pos = col_mat3_transform_vec2(mat,base);

            pos[0] += origin[0];
            pos[1] += origin[1];



            let (clip_x, clip_y, clip_w, clip_h) = (pos[0] - rust_4_1_w, pos[1] - rust_4_1_h, rust_4_1_w * 2.0, rust_4_1_h * 2.0);

            let clipped = c.draw_state.scissor([clip_x as u32, clip_y as u32, clip_w as u32, clip_h as u32]);
            Image::new().draw(&rust_logo, &clipped, transform, g);

            let angle2 = angle + PI;

            let mat:Matrix3<f32> = [
                [angle2.cos(),-angle2.sin(),0.0],
                [angle2.sin(), angle2.cos(),0.0],
                [0.0,0.0,1.0]
            ];

            let mut pos = col_mat3_transform_vec2(mat,base);

            pos[0] += clip_w;
            pos[1] += clip_h;

            //let transform = c.transform.trans(200.0, 200.0);
            Ellipse::new([1.0, 0.0, 0.0, 1.0])
                .draw([
                          (pos[0] - rust_4_1_w) as f64,
                          (pos[1] - rust_4_1_h) as f64,
                                    clip_w as f64,
                                    clip_h as f64  ],
                      &DrawState::new_clip(),
                                transform, g);
            Image::new().draw(&rust_logo,
                &if clip_inside { DrawState::new_inside() }
                else { DrawState::new_outside() },
                transform, g);

            //if angle >= PI * 2.0 { angle = 0f32; }else{ angle += 0.1; } 匀速
            if cur_i >= tween_vec.len() { cur_i = 0; }
            angle = PI * 2.0 * tween_vec[cur_i].y;
            cur_i += 1;

        });

        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            blend = (blend + 1) % blends.len();
            println!("Changed blending to {:?}", blend);
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            clip_inside = !clip_inside;
            if clip_inside {
                println!("Changed to clip inside");
            } else {
                println!("Changed to clip outside");
            }
        }
    }
}