pub mod cartesian;
mod functions;
mod traits;

pub use functions::angle;

pub use traits::{
    IDot, INorm, IVector2, IVector3, IVectorAccessorX, IVectorAccessorY, IVectorAccessorZ,
};
