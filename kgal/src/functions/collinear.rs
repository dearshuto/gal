use std::ops::Sub;

use crate::traits::ICross;

pub trait ICollinearObject {
    type Point;
    fn collinear(&self, p: &Self::Point, q: &Self::Point, r: &Self::Point) -> bool;
}

// デフォルト実装
struct CollinearObject<TPoint, TVector> {
    _marker: std::marker::PhantomData<(TPoint, TVector)>,
}

impl<TPoint, TVector> CollinearObject<TPoint, TVector>
where
    TPoint: Into<TVector> + Clone,
    TVector: ICross + Sub<Output = TVector> + num::Zero,
{
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TPoint, TVector> ICollinearObject for CollinearObject<TPoint, TVector>
where
    TPoint: Into<TVector> + Clone,
    TVector: ICross + Sub<Output = TVector> + num::Zero,
{
    type Point = TPoint;

    fn collinear(&self, p: &Self::Point, q: &Self::Point, r: &Self::Point) -> bool {
        let pv = p.clone().into();
        let qv = q.clone().into();
        let rv = r.clone().into();

        // PQ と PR のふたつのベクトルが同じ方向を向いていたら同一直線上とみなす
        // 同一方向の判定は内積を利用
        let pq = qv - pv;
        let pr = rv - p.clone().into();
        let diff = pq.cross(&pr);
        diff.is_zero()
    }
}

// デフォルト実装を利用した同一直線上判定
pub fn collinear<TPoint, TVector>(p: &TPoint, q: &TPoint, r: &TPoint) -> bool
where
    TPoint: Into<TVector> + Clone,
    TVector: ICross + Sub<Output = TVector> + num::Zero,
{
    collinear_with(p, q, r, CollinearObject::new())
}

// オブジェクト指定で同一直線上判定
pub fn collinear_with<T>(p: &T::Point, q: &T::Point, r: &T::Point, obj: T) -> bool
where
    T: ICollinearObject,
{
    obj.collinear(p, q, r)
}

#[cfg(test)]
mod tests {
    #[test]
    fn collinear() {
        let p = nalgebra::Vector3::<f32>::new(1.0, 0.0, 0.0);
        let q = nalgebra::Vector3::new(2.0, 0.0, 0.0);
        let r = nalgebra::Vector3::new(3.0, 0.0, 0.0);

        assert!(super::collinear::<
            nalgebra::Vector3<f32>,
            nalgebra::Vector3<f32>,
        >(&p, &q, &r));
    }
}
