mod functions;
mod traits;
pub mod util;

pub use functions::{
    angle, angle_with, collinear, collinear_with, distance_2, IAngleObject, ICollinearObject,
};

pub use traits::{
    ICross, IDot, ILine, INorm, ISqrt, IVectorAccessorX, IVectorAccessorY, IVectorAccessorZ,
};
