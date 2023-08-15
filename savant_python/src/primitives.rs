/// Attribute module specifies attribute code for [crate::primitives::VideoObject] and [crate::primitives::VideoFrame].
///
pub mod attribute;
pub mod attribute_value;
/// Here are decleared bounding boxes
///
pub mod bbox;
/// The draw specification used to draw objects on the frame when they are visualized.
pub mod draw;
pub mod message;
/// Simple point structure.
pub mod point;
/// A structure representing polygonal areas and functions.
pub mod polygonal_area;
/// Implementation for Python attributes in VideoObject and VideoFrame.
pub mod pyobject;
/// A line consisting of two points.
pub mod segment;

use crate::primitives::message::video::frame::VideoFrameTransformation;

pub use attribute::Attribute;
pub use attribute_value::{AttributeValue, AttributeValueType, AttributeValuesView};
pub use bbox::{BBox, RBBox};
pub use draw::*;
pub use message::eos::EndOfStream;
pub use message::loader::load_message;
pub use message::saver::save_message;
pub use message::unspecified::UnspecifiedData;
pub use message::video::batch::VideoFrameBatch;

pub use message::video::frame::{VideoFrame, VideoFrameContent, VideoFrameTranscodingMethod};

use crate::primitives::message::video::frame_update::{
    AttributeUpdatePolicy, ObjectUpdatePolicy, VideoFrameUpdate,
};
use crate::primitives::message::video::object::{IdCollisionResolutionPolicy, VideoObject};
use crate::primitives::message::video::objects_view::VideoObjectsView;
pub use message::Message;
pub use point::Point;
pub use polygonal_area::PolygonalArea;
use pyo3::prelude::PyModule;
use pyo3::{pymodule, PyResult, Python};
pub use segment::{Intersection, IntersectionKind, Segment};

#[pymodule]
pub fn geometry(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Segment>()?;
    m.add_class::<IntersectionKind>()?;
    m.add_class::<Intersection>()?;
    m.add_class::<PolygonalArea>()?;
    m.add_class::<RBBox>()?;
    m.add_class::<BBox>()?;
    Ok(())
}

#[pymodule]
pub fn draw_spec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ColorDraw>()?;
    m.add_class::<BoundingBoxDraw>()?;
    m.add_class::<DotDraw>()?;
    m.add_class::<LabelDraw>()?;
    m.add_class::<LabelPositionKind>()?;
    m.add_class::<LabelPosition>()?;
    m.add_class::<PaddingDraw>()?;
    m.add_class::<ObjectDraw>()?;
    m.add_class::<SetDrawLabelKind>()?;
    Ok(())
}

#[pymodule]
pub fn primitives(_py: Python, m: &PyModule) -> PyResult<()> {
    use VideoFrameContent;
    use VideoFrameTransformation;

    m.add_class::<Attribute>()?;
    m.add_class::<AttributeUpdatePolicy>()?;
    m.add_class::<ObjectUpdatePolicy>()?;
    m.add_class::<AttributeValue>()?;
    m.add_class::<AttributeValueType>()?;
    m.add_class::<AttributeValuesView>()?;
    m.add_class::<EndOfStream>()?;
    m.add_class::<UnspecifiedData>()?;

    m.add_class::<VideoFrame>()?;
    m.add_class::<VideoFrameBatch>()?;
    m.add_class::<VideoFrameContent>()?;
    m.add_class::<VideoFrameTranscodingMethod>()?;
    m.add_class::<VideoFrameUpdate>()?;
    m.add_class::<VideoFrameTransformation>()?;

    m.add_class::<VideoObject>()?;
    m.add_class::<VideoObjectsView>()?;

    m.add_class::<IdCollisionResolutionPolicy>()?;

    Ok(())
}