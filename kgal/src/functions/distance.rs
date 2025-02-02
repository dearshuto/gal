use std::ops::{Add, Div, Mul};

use crate::{
    traits::{ILine, ISqrt},
    IVectorAccessorX, IVectorAccessorY,
};

pub fn distance_2<T, TPoint, TLine>(p: &TPoint, l: &TLine) -> T
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + Clone + ISqrt<T>,
    TPoint: IVectorAccessorX<T> + IVectorAccessorY<T>,
    TLine: ILine<T>,
{
    (l.a() * p.x() + l.b() * p.y() + l.c()) / (l.a() * l.a() + l.b() * l.b()).sqrt()
}
