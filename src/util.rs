use crate::vec::Vec2;

pub fn tile_2d_collision(ap: Vec2<f64>, _as_: Vec2<f64>, bp: Vec2<f64>, bs: Vec2<f64>) -> bool {
    if ap.x + _as_.x > bp.x && bp.x + bs.x > ap.x {
        if ap.y + _as_.y > bp.y && bp.y + bs.y > ap.y {
            return true;
        }
    }
    false
}
