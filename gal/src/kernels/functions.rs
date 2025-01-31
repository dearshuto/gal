use std::ops::{Div, Mul};

use super::{traits::IArcCos, IDot, INorm};

pub fn angle<T, TVector>(lhs: &TVector, rhs: &TVector) -> T
where
    T: IArcCos<T> + Div<Output = T> + Mul<Output = T>,
    TVector: IDot<T> + INorm<T>,
{
    let dot_product = lhs.dot(&rhs);
    let norm_l = lhs.norm();
    let norm_r = rhs.norm();
    let cos_theta = dot_product / (norm_l * norm_r);
    cos_theta.acos()
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector2;

    #[test]
    fn orthogonal() {
        let u = Vector2::new(0.0f32, 1.0f32);
        let v = Vector2::new(0.5f32, 0.0f32);

        // 90 度を期待
        assert_eq!(super::angle::<f32, _>(&u, &v), std::f32::consts::PI / 2.0);
    }

    #[test]
    fn angle_45() {
        let u = Vector2::new(1.0f32, 1.0f32);
        let v = Vector2::new(1.0f32, 0.0f32);

        // 45 度を期待
        assert_eq!(super::angle::<f32, _>(&u, &v), std::f32::consts::PI / 4.0);
    }
}
