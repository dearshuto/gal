pub mod cartesian;
mod functions;
mod traits;

pub use functions::{angle, angle_with, collinear, collinear_with, IAngleObject, ICollinearObject};

pub use traits::{
    ICross, IDot, INorm, IPoint2, IVector2, IVector3, IVectorAccessorX, IVectorAccessorY,
    IVectorAccessorZ,
};
