use crate::kernels::{IVector2, IVector3};

pub trait ICartesian<T> {
    type Vector2: IVector2<T>;

    type Vector3: IVector3<T>;
}
