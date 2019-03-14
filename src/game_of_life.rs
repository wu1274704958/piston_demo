extern crate piston_window;
extern crate glutin_window;

use piston_window::*;
use glutin_window::GlutinWindow;

const WORLD_W: u32 = 100;
const WORLD_H: u32 = 100;

const CELL_W: u32 = 4;

const fn CELL_W_1_2() -> u32
{
    CELL_W  / 2
}

fn RECT_MODEL() -> [f64;4]
{
    let w_1_2 = CELL_W_1_2() as f64;
    [-w_1_2,-w_1_2,CELL_W as f64,CELL_W as f64]
}

mod game_of_life_world;

use game_of_life_world::{World,CellState};

fn main() {
    let mut window: PistonWindow<GlutinWindow> = WindowSettings::new(
        "game of life",
        (WORLD_W * CELL_W, WORLD_H * CELL_W),
    )
        .exit_on_esc(true)
        .samples(32)
        .vsync(true)
        .decorated(true)
        .resizable(false)
        //.transparent(true)
        .build()
        .unwrap();
    let mut lbd = false;
    let mut world = World::new(WORLD_W,WORLD_H);
    world.set_alive(1,0);
    world.set_alive(1,1);
    world.set_alive(2,1);

    while let Some(e) = window.next() {
        if let Event::Loop(Loop::Render(RenderArgs { ext_dt, .. })) = e {
            window.draw_2d(&e, |c: Context, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                for y in 0..WORLD_H{
                    for x in 0..WORLD_W {
                        if let CellState::Alive = world.get_cell(x,y)
                        {
                            draw_cell(x,y,g,c);
                        }
                    }
                }
                world.deduction();
            });
        }

        if let Event::Input(Input::Move(Motion::MouseCursor(x, y))) = e {
            if lbd{
                let cp = get_cell_pos(x,y);
                world.set_alive(cp.0,cp.1);
            }
        }


        if let Event::Input(Input::Button(ButtonArgs { button, state, .. })) = e {
            if let Button::Mouse(MouseButton::Left) = button {
                match state {
                    ButtonState::Press => {
                        lbd = true;

                    }
                    ButtonState::Release => {
                        lbd = false;
                    }
                }
            }

            if let Button::Mouse(MouseButton::Middle) = button {
                match state {
                    ButtonState::Press => {
//                        world.deduction();
                    }
                    ButtonState::Release => {

                    }
                }
            }
        }
    }
}

fn get_cell_pos(x:f64,y:f64) -> (u32,u32)
{
    ((x / CELL_W as f64).round() as u32,  (y / CELL_W as f64).round() as u32)
}

fn draw_cell(x:u32,y:u32,g:&mut G2d,c:Context)
{
    let trans = c.transform.trans((x * CELL_W + CELL_W_1_2()) as _ , (y * CELL_W + CELL_W_1_2()) as _);
    Rectangle::new([1.0,1.0,1.0,1.0])
        .draw(RECT_MODEL(),&(c.draw_state),trans,g);
}

