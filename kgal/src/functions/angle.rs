use std::ops::{Div, Mul};

use crate::{traits::IArcCos, IDot, INorm};

pub trait IAngleObject {
    type Output;
    type Vector;

    fn angle(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Output;
}

struct AngleObject<TOutput, TVector> {
    _marker: std::marker::PhantomData<(TOutput, TVector)>,
}

impl<TOutput, TVector> AngleObject<TOutput, TVector>
where
    TOutput: IArcCos<TOutput> + Div<Output = TOutput> + Mul<Output = TOutput>,
    TVector: IDot<TOutput> + INorm<TOutput>,
{
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TOutput, TVector> IAngleObject for AngleObject<TOutput, TVector>
where
    TOutput: IArcCos<TOutput> + Div<Output = TOutput> + Mul<Output = TOutput>,
    TVector: IDot<TOutput> + INorm<TOutput>,
{
    type Output = TOutput;
    type Vector = TVector;

    fn angle(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Output {
        let dot_product = lhs.dot(&rhs);
        let norm_l = lhs.norm();
        let norm_r = rhs.norm();
        let cos_theta = dot_product / (norm_l * norm_r);
        cos_theta.acos()
    }
}

pub fn angle<T, TVector>(lhs: &TVector, rhs: &TVector) -> T
where
    T: IArcCos<T> + Div<Output = T> + Mul<Output = T>,
    TVector: IDot<T> + INorm<T>,
{
    let obj = AngleObject::new();
    angle_with(lhs, rhs, obj)
}

pub fn angle_with<T>(lhs: &T::Vector, rhs: &T::Vector, obj: T) -> T::Output
where
    T: IAngleObject,
{
    obj.angle(lhs, rhs)
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector2;

    #[test]
    fn orthogonal() {
        let u = Vector2::new(0.0f32, 1.0f32);
        let v = Vector2::new(0.5f32, 0.0f32);

        // 90 度を期待
        assert_eq!(super::angle(&u, &v), std::f32::consts::PI / 2.0);
    }

    #[test]
    fn angle_45() {
        let u = Vector2::new(1.0f32, 1.0f32);
        let v = Vector2::new(1.0f32, 0.0f32);

        // 45 度を期待
        assert_eq!(super::angle(&u, &v), std::f32::consts::PI / 4.0);
    }
}
