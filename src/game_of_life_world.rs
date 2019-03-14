use std::iter::Iterator;
use std::ptr::copy;

#[derive(Copy, Clone,PartialEq)]
pub enum CellState{
    Alive,
    Dead
}

pub struct AroundCellIter{
    bx: i32,
    by: i32,
    step : u32
}

impl AroundCellIter{
    fn new(bx:i32,by:i32) -> AroundCellIter
    {
        AroundCellIter{ bx,by,step : 0 }
    }
}

impl Iterator for AroundCellIter{
    type Item = (i32,i32);

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.step {
            0 => Some((self.bx - 1   ,self.by - 1)),
            1 => Some((self.bx       ,self.by - 1)),
            2 => Some((self.bx + 1   ,self.by - 1)),
            3 => Some((self.bx + 1   ,self.by    )),
            4 => Some((self.bx + 1   ,self.by + 1)),
            5 => Some((self.bx       ,self.by + 1)),
            6 => Some((self.bx - 1   ,self.by + 1)),
            7 => Some((self.bx - 1   ,self.by    )),
            8 => None,
            _ => None
        };
        self.step += 1;
        res
    }
}

pub struct World{
    width : u32,
    height : u32,
    cell_states: Vec<CellState>,
    back_cell_states : Vec<CellState>
}

impl World{
    pub fn new(width:u32,height :u32) -> World
    {
        World{
            width,
            height,
            cell_states : vec![CellState::Dead;(width * height) as usize],
            back_cell_states : vec![CellState::Dead;(width * height) as usize]
        }
    }

    pub fn get_cell(&self,x:u32,y:u32) -> CellState
    {
        self.cell_states[(y * self.width + x) as usize]
    }

    pub fn set_cell(&mut self,x:u32,y:u32,cs:CellState)
    {
        if self.get_cell(x,y) == cs { return; }
        self.cell_states[(y * self.width + x) as usize] = cs;
    }

    pub fn set_alive(&mut self,x:u32,y:u32) -> bool
    {
        if !self.in_bound(x as _,y as _) { return false; }
        self.set_cell(x,y,CellState::Alive);
        true
    }

    pub fn set_dead(&mut self,x:u32,y:u32) -> bool
    {
        if !self.in_bound(x as _,y as _) { return false; }
        self.set_cell(x,y,CellState::Dead);
        true
    }


    pub fn get_cell_back(&self,x:u32,y:u32) -> CellState
    {
        self.back_cell_states[(y * self.width + x) as usize]
    }

    pub fn set_cell_back(&mut self,x:u32,y:u32,cs:CellState)
    {
        if self.get_cell_back(x,y) == cs { return; }
        self.back_cell_states[(y * self.width + x) as usize] = cs;
    }

    pub fn set_alive_back(&mut self,x:u32,y:u32) -> bool
    {
        if !self.in_bound(x as _,y as _) { return false; }
        self.set_cell_back(x,y,CellState::Alive);
        true
    }

    pub fn set_dead_back(&mut self,x:u32,y:u32) -> bool
    {
        if !self.in_bound(x as _,y as _) { return false; }
        self.set_cell_back(x,y,CellState::Dead);
        true
    }

    pub fn in_bound(&self,x:i32,y:i32) -> bool
    {
        if x >= self.width as i32 || y >= self.height as i32 || x < 0 || y < 0 { return false }else{ true }
    }

    pub fn get_around_alive_cell_count(&self,x:i32,y:i32) -> u32
    {
        let aciter = AroundCellIter::new(x,y);
        let mut res = 0u32;
        //dbg!((aciter.bx,aciter.by));
        aciter.for_each(|it|{
            //dbg!(it);
            if self.in_bound(it.0,it.1){
                if CellState::Alive == self.get_cell(it.0 as _,it.1 as _){
                    res += 1;
                }
            }
        });
        //dbg!(res);
        res
    }

    pub fn deduction(&mut self)
    {
        self.copy_to_back();
        for y in 0..self.height {
            for x in 0..self.width {
                let count = self.get_around_alive_cell_count(x as _,y as _);
                //dbg!((x,y,count));
                match count {
                    2 => { }    // 保持不變
                    3 => {      // 轉衛生
                        if !self.set_alive_back(x,y){  dbg!((x,y));  }
                    }
                    _ => {      // 轉為死
                        if !self.set_dead_back(x,y){  dbg!((x,y));  }
                    }
                }
            }
        }
        self.copy_to_forward();
    }

    fn copy_to_forward(&mut self)
    {
        let size = self.cell_states.len();
        let dst = &mut (self.cell_states[0]) as *mut CellState;
        let src = &(self.back_cell_states[0]) as *const CellState;
        unsafe{ copy(src,dst,size) };
    }

    fn copy_to_back(&mut self)
    {
        let size = self.cell_states.len();
        let dst = &mut (self.back_cell_states[0]) as *mut CellState;
        let src = &(self.cell_states[0]) as *const CellState;
        unsafe{ copy(src,dst,size) };
    }
}