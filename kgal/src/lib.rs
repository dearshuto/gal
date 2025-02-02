mod functions;
mod traits;
pub mod util;

pub use functions::{
    angle, angle_with, collinear, collinear_with, distance_2, IAngleObject, ICollinearObject,
};

pub use traits::{
    ICompareX, ICross, IDot, ILine, INorm, ISqrt, IVectorAccessorX, IVectorAccessorY,
    IVectorAccessorZ,
};
