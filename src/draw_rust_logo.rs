extern crate piston_window;
extern crate find_folder;
extern crate vecmath;
extern crate glutin_window;
extern crate piston_demo;

use piston_window::draw_state::Blend;
use piston_window::*;
use piston_window::{ Position,Texture,Flip,PistonWindow,Transformed,AdvancedWindow,Event,TextureSettings,ImageSize,RenderArgs};
use piston_window::math::{ Matrix2d,Vec2d,rotate_radians,translate,transform_vec,add,sub,cast,dot};
use std::f64::consts::{ PI};
use glutin_window::GlutinWindow;

use nbez::{BezChain,Point2d,Bez3o,BezCurve};
use piston_window::draw_state::Stencil;
use vecmath::vec2_normalized;
use std::env::args;
use piston_demo::TransparentWindow::{WindowSettings,create_window };

const W:u16 = 150u16;
const H:u16 = 150u16;

fn main() {
    let default_ctrl_ps:Vec<f64> = vec![0.0 ,0.0,1.0,1.0];
    let mut ctrl_ps = vec![];

    let mut ctrl_ps_handler= &default_ctrl_ps;

    let mut aot = false; // always on top

    let mut stage = 0;
    args().for_each(|arg|{
        if stage != 0 {
            match stage {
                1 => {
                    let split_strs : Vec<&str> = arg.split(",").collect();
                    ctrl_ps = split_strs.iter().filter_map(|it|{
                        let mut s = it.to_string();
                        if it.len() == 0 { return None; }
                        if it.as_bytes()[0] == b'.'{
                            s.insert_str(0,"0");
                        }
                        str::parse::<f64>(s.as_str()).ok()
                    }).collect();
                    let ptr = &ctrl_ps as *const _;
                    ctrl_ps_handler = unsafe { &(*ptr) };
                },
                _ => {}
            }
            stage = 0;
        }else{
            match arg.as_str() {
                "-o" => aot = true,
                "-v" => stage = 1,
                _ => {}
            }
        }
    });

    if ctrl_ps_handler.len() < 4{
        ctrl_ps_handler = &default_ctrl_ps;
    }

    let tween_vec = {
        let curve: Bez3o<f64> = Bez3o::new(
            Point2d::new(0.0    ,0.0),
            Point2d::new(ctrl_ps_handler[0]   , ctrl_ps_handler[1]),
            Point2d::new(ctrl_ps_handler[2]   , ctrl_ps_handler[3]),
            Point2d::new(1.0    ,1.0)
        );
        let curve_chain: BezChain<f64, Bez3o<f64>, Vec<Point2d<f64>>> = BezChain::from_container(vec![
            curve.start,
            curve.ctrl0,
            curve.ctrl1,
            curve.end ]);

        let mut frame = 60usize;

        let mut res = vec![];
        for curve in curve_chain.iter() {
            let zl = 1.0 / frame as f64;
            let mut t = zl;
            for _i in 0..frame{
                if _i == frame - 1 { t = 0.9999999; }
                let temp = curve.interp(t).unwrap();
                res.push(temp.y);
                t += zl;
            }
        }
        let last = res.len() - 1;
        res[last] = 1.0;
        res
    };

    let mut window: PistonWindow<GlutinWindow> = WindowSettings::new(
        "piston: draw_state",
        (W as u32,H as u32)
    )
        .exit_on_esc(true)
        .samples(32)
        .vsync(true)
        .decorated(false)
        .resizable(false)
        .transparent(true)
        .always_on_top(aot)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let rust_logo = Texture::from_path(&mut window.factory,
                                       assets.join("rust.png"),
                                       Flip::None,
                                       &TextureSettings::new()).unwrap();
    //window.set_lazy(true);
    let rust_4_1_w = rust_logo.get_width() as f64 / 4.0;
    let rust_4_1_h = rust_logo.get_height() as f64 / 4.0;
    let origin:Vec2d = cast([rust_4_1_w * 2.0,rust_4_1_h * 2.0]);
    let mut angle = 0.0f64;
    let base:Vec2d = cast([0.0,rust_4_1_h]);

    let ( clip_w, clip_h) = (rust_4_1_w * 2.0, rust_4_1_h * 2.0);
    let (n_w,n_h) = (clip_w * 1.36,clip_h * 1.36);
    let base_c1 = [ -n_w/2.0,-n_h/2.0,n_w,n_h ];
    let base_c2 = [ -clip_w/2.0,-clip_h/2.0,clip_w,clip_h ];

    let base_logo = [-clip_w,-clip_h,clip_w * 2.0,clip_h * 2.0];

    let mut cur_i = 0usize;
    let mut cur_i2 = 0usize;
    let mut scale_x = 0.0;
    let mut scale_x_dir = 1i32;

    let mut ds_c1 = DrawState::new_outside();
    ds_c1.stencil = Some(Stencil::Clip(1));
    let mut ds_c2 = DrawState::new_outside();
    ds_c2.stencil = Some(Stencil::Clip(2));

    let mut ds_l1 = DrawState::new_outside();
    ds_l1.stencil = Some(Stencil::Outside(1));
    let mut ds_l2 = DrawState::new_outside();
    ds_l2.stencil = Some(Stencil::Inside(2));

    let mut mouse_button_down = false;
    let mut last_cursor_pos:Vec2d = [0.0,0.0];
    let mut begin_cursor_pos:Vec2d = [W as f64,H as f64];
    let mut now = Position{x:0,y:0};
    let mut lose_focus = false;

    let PI2 = PI * 2.0;
    let mut f = true;

    let mut angle2 = 0.0;

    while let Some(e) = window.next() {

        if let Event::Loop(Loop::Render(RenderArgs{ext_dt,..})) = e {
            window.draw_2d(&e, |c:Context, g| {

                clear([0.0, 0.0, 0.0, 0.0], g);

                let transform = c.transform.trans(3.0, 3.0);

                let mat: Matrix2d = rotate_radians(angle);

                let pos: Vec2d = transform_vec(mat, base);//base;


                let mut ds_clip = DrawState::new_clip();
                ds_clip.stencil = Some(Stencil::Clip(254));


                let c1_tf = transform.trans(origin[0], origin[1]).trans(pos[0], pos[1]);

                Ellipse::new([1.0, 0.0, 0.0, 1.0])
                    .draw(base_c1,
                          &ds_c1,
                          c1_tf, g);


                let fx: Vec2d = vec2_normalized(pos);
                let a = if f { (dot(fx, [1.0, 0.0]) as f64).acos() } else { (dot(fx, [-1.0, 0.0]) as f64).acos() };

                if PI - a < 0.1 { f = !f; }

                let l1_tf = transform.trans(origin[0], origin[1]).rot_rad(angle);

                Image::new().rect(base_logo).draw(&rust_logo,
                                                  &ds_l1,
                                                  l1_tf, g);

                //println!("{:?} ",a);

                let c2_tf = transform.trans(origin[0], origin[1])
                    .trans(pos[0], pos[1])
                    .rot_rad(a)
                    .scale(scale_x, 1.0);

//            draw Debug
//                Line::new([1.0, 0.0, 0.0, 1.0], 1.0)
//                    .draw([0.0, 0.0, 100.0, 0.0],
//                          &(c.draw_state),
//                          transform.trans(origin[0], origin[1]).rot_rad(if f {a}else{PI + a} ), g);
//
//                Line::new([1.0, 0.0, 0.0, 1.0], 1.0)
//                    .draw([0.0, 0.0, 100.0, 0.0] ,
//                          &(c.draw_state),
//                          transform.trans(origin[0], origin[1]), g);

                Ellipse::new([1.0, 0.0, 0.0, 1.0])
                    .draw(base_c2,
                          //&(c.draw_state),
                          &ds_c2,
                          c2_tf, g);


                let l2_tf = transform.trans(origin[0], origin[1]).rot_rad(angle2);

                Image::new().rect(base_logo).draw(&rust_logo,
                                                  &ds_l2,
                                                  l2_tf, g);

                // if angle >= PI2 { angle = 0.0; }else{ angle += 0.1; }

                if scale_x_dir > 0 && cur_i2 >= tween_vec.len() - 1 {
                    scale_x_dir = -1;
                }

                if scale_x_dir < 0 && cur_i2 == 0 {
                    scale_x_dir = 1;
                }

                cur_i2 += (scale_x_dir * 1) as usize;
                scale_x = 1.3 * tween_vec[cur_i2];

                if cur_i >= tween_vec.len() { cur_i = 0; }

                angle = PI2 * tween_vec[cur_i];
                angle2 = PI2 * tween_vec[tween_vec.len() - 1 - cur_i];

                cur_i += 1;
            });
        }

        if let Event::Input(Input::Move(Motion::MouseCursor(x,y))) = e {

            if mouse_button_down {
                let offset = sub([x,y],begin_cursor_pos);
                if offset[0].abs() >= 5.0 || offset[1].abs() >= 5.0
                {
                    //println!("{:?}, {:?} {:?}",now,offset,begin_cursor_pos);
                    now = window.get_position().unwrap();
                    window.set_position(Position{x:now.x + offset[0] as i32, y: now.y + offset[1] as i32});
                }
            }
            last_cursor_pos[0] = x;
            last_cursor_pos[1] = y;
        }


        if let Event::Input(Input::Button(ButtonArgs{button,state,..})) = e{
            if let Button::Mouse(MouseButton::Left) = button{
                match state {
                    ButtonState::Press => {
                        mouse_button_down = true;
                        if lose_focus{
                            begin_cursor_pos = [ (W >> 1) as f64,(H >> 1) as f64 ];
                            lose_focus = false;
                        }else{
                            begin_cursor_pos = last_cursor_pos;
                        }
                    },
                    ButtonState::Release => {
                        mouse_button_down = false;
                    }
                }
            }
        }

        if let Event::Input(Input::Focus(is_focus)) = e {
            if !is_focus { lose_focus = true; }
        }
    }
}


