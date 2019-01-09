extern crate piston_window;
extern crate find_folder;
extern crate vecmath;

use piston_window::draw_state::Blend;
use piston_window::*;
use piston_window::Transformed;
use piston_window::math::{ Matrix2d,Vec2d,rotate_radians,translate,transform_vec,add,sub,cast,dot};
use std::f64::consts::PI;

use nbez::{BezChain,Point2d,Bez3o,BezCurve};
use piston_window::draw_state::Stencil;
use vecmath::vec2_normalized;

fn main() {

    let tween_vec = {
        let curve: Bez3o<f64> = Bez3o::new(
            Point2d::new(0.0    ,0.0),
            Point2d::new(0.06   ,0.74),
            Point2d::new(0.32   ,1.00),
            Point2d::new(1.0    ,1.0)
        );
        let curve_chain: BezChain<f64, Bez3o<f64>, Vec<Point2d<f64>>> = BezChain::from_container(vec![
            curve.start,
            curve.ctrl0,
            curve.ctrl1,
            curve.end ]);
        let mut res = vec![];
        for curve in curve_chain.iter() {
            let mut t = 0.0;
            let zl = 1.0 / 40.0;

            for _i in 0..40{
                res.push(curve.interp(t).unwrap());
                t += zl;
            }
        }
        let last = res.len() - 1;
        res[last].y = 1.0;
        res
    };

    let mut window: PistonWindow = WindowSettings::new(
        "piston: draw_state",
        [150, 150]
    )
        .exit_on_esc(true)
        .samples(32)
        .vsync(true)
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
    let mut scale_x = 0.0;
    let mut scale_x_dir = 1.0;

    let mut ds_c1 = DrawState::new_outside();
    ds_c1.stencil = Some(Stencil::Clip(1));
    let mut ds_c2 = DrawState::new_outside();
    ds_c2.stencil = Some(Stencil::Clip(2));

    let mut ds_l1 = DrawState::new_outside();
    ds_l1.stencil = Some(Stencil::Outside(1));
    let mut ds_l2 = DrawState::new_outside();
    ds_l2.stencil = Some(Stencil::Inside(2));


    let PI2 = PI * 2.0;
    let mut f = true;

    let mut angle2 = 0.0;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c:Context, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);

            let transform = c.transform.trans(3.0, 3.0);

            let mat:Matrix2d = rotate_radians(angle);


            let pos:Vec2d =  transform_vec(mat,base);//base;



            let mut ds_clip = DrawState::new_clip();
            ds_clip.stencil = Some(Stencil::Clip(254));



            let c1_tf =  transform.trans(origin[0],origin[1]).trans(pos[0],pos[1]);

            Ellipse::new([1.0, 0.0, 0.0, 1.0])
                .draw(base_c1,
                      &ds_c1,
                      c1_tf, g);



            let fx:Vec2d= vec2_normalized(pos);
            let a = if f { (dot(fx,[1.0,0.0]) as f64).acos() } else { (dot(fx,[-1.0,0.0]) as f64).acos() };

            if PI - a < 0.1 { f = !f; }

            let l1_tf = transform.trans(origin[0],origin[1]).rot_rad(angle);

            Image::new().rect(base_logo).draw(&rust_logo,
                                              &ds_l1 ,
                                              l1_tf, g);


            //println!("{:?} ",a);

            let c2_tf =  transform.trans(origin[0],origin[1])
                        .trans(pos[0],pos[1])
                        .rot_rad(a)
                        .scale(scale_x,1.0);


//            Line::new([1.0, 0.0, 0.0, 1.0],1.0)
//                .draw([0.0,0.0,100.0,0.0],
//                    &(c.draw_state),
//                      transform.trans(origin[0],origin[1]).rot_rad(a),g);
//
//            Line::new([1.0, 0.0, 0.0, 1.0],1.0)
//                .draw([0.0,0.0,100.0,0.0],
//                      &(c.draw_state),
//                      transform.trans(origin[0],origin[1]),g);

            Ellipse::new([1.0, 0.0, 0.0, 1.0])
                .draw(base_c2,
                      //&(c.draw_state),
                      &ds_c2,
                      c2_tf, g);



            let l2_tf = transform.trans(origin[0],origin[1]).rot_rad(angle2);

            Image::new().rect(base_logo).draw(&rust_logo,
                              &ds_l2 ,
                              l2_tf, g);

           // if angle >= PI2 { angle = 0.0; }else{ angle += 0.1; }
            scale_x += scale_x_dir * 0.1;
            if scale_x_dir > 0.0 && 1.0 - scale_x < 0.1{
                scale_x_dir = 0.0 - scale_x_dir;
            }

            if scale_x_dir < 0.0 && scale_x < 0.1{
                scale_x_dir = 0.0 - scale_x_dir;
            }
            if cur_i >= tween_vec.len() { cur_i = 0; }

            angle =  PI * 2.0 * tween_vec[cur_i].y;
            angle2 =  PI * 2.0 * tween_vec[tween_vec.len() - 1 - cur_i].y;

            cur_i += 1;



        });

    }
}
