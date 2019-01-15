#![crate_type="lib"]




pub struct UnitBezier{
    ax:f64,
    ay:f64,
    bx:f64,
    by:f64,
    cx:f64,
    cy:f64,
}

impl UnitBezier {
    pub fn new (p1x:f64,p1y:f64,p2x:f64,p2y:f64) -> Self
    {
        let cx = 3.0 * p1x;
        let bx = 3.0 * (p2x - p1x) - cx;
        let ax = 1.0 - cx - bx;
        let cy = 3.0 * p1y;
        let by = 3.0 * (p2y - p1y) - cy;
        let ay = 1.0 - cy - by;

        UnitBezier {
            ax,ay,bx,by,cx,cy
        }
    }

    pub fn sample_curve_x(&self,t:f64) ->f64 {
         ((self.ax * t + self.bx) * t + self.cx) * t
    }

    pub fn sample_curve_y(&self,t:f64) ->f64 {
         ((self.ay * t + self.by) * t + self.cy) * t
    }

    pub fn solve(&self,t:f64) -> f64 {
        self.sample_curve_y(self.sample_curve_x(t))
    }
}

pub fn division<F>(x:f64,t1:f64,t2:f64, epsilon:f64,func:F) -> f64
    where F:Fn(f64)->f64
{
    let mut t0 = t1;
    let mut t1 = t2;
    let mut x2 = 0.0;
    let mut t2 = x;
    if t2 < t0 { return t0;}
    if t2 > t1 { return t1;}
    while t0 < t1 {
        x2 = func(t2);
        if (x2 - x).abs() < epsilon { return t2; }
        if x > x2 { t0 = t2; } else { t1 = t2; }
        t2 = (t1 - t0) * 0.5 + t0;
    }
    return t2;
}