pub mod utils {
    use crate::primitives::message::video::frame::InnerVideoFrameBuilder;
    use crate::primitives::message::video::object::InnerObjectBuilder;
    use crate::primitives::{
        AttributeBuilder, Intersection, IntersectionKind, Point, PyVideoFrameContent, Value,
    };
    use crate::primitives::{BBox, VideoFrame};
    use pyo3::pyfunction;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[pyfunction]
    pub fn gen_frame() -> VideoFrame {
        let mut f = VideoFrame::from_inner(
            InnerVideoFrameBuilder::default()
                .source_id("test".to_string())
                .pts(0)
                .framerate("test".to_string())
                .width(0)
                .height(0)
                .content(PyVideoFrameContent::none().inner)
                .dts(None)
                .transformations(Vec::default())
                .duration(None)
                .codec(None)
                .keyframe(None)
                .attributes(HashMap::default())
                .offline_objects(Default::default())
                .resident_objects(vec![
                    Arc::new(Mutex::new(
                        InnerObjectBuilder::default()
                            .id(0)
                            .track_id(None)
                            .modifications(Vec::default())
                            .bbox(BBox::new(0.0, 0.0, 0.0, 0.0, None))
                            .parent(None)
                            .attributes(HashMap::default())
                            .confidence(None)
                            .creator("test".to_string())
                            .label("test2".to_string())
                            .build()
                            .unwrap(),
                    )),
                    Arc::new(Mutex::new(
                        InnerObjectBuilder::default()
                            .id(1)
                            .track_id(None)
                            .modifications(Vec::default())
                            .bbox(BBox::new(0.0, 0.0, 0.0, 0.0, None))
                            .parent(None)
                            .attributes(HashMap::default())
                            .confidence(None)
                            .creator("test2".to_string())
                            .label("test".to_string())
                            .build()
                            .unwrap(),
                    )),
                    Arc::new(Mutex::new(
                        InnerObjectBuilder::default()
                            .id(2)
                            .track_id(None)
                            .modifications(Vec::default())
                            .bbox(BBox::new(0.0, 0.0, 0.0, 0.0, None))
                            .parent(None)
                            .attributes(HashMap::default())
                            .confidence(None)
                            .creator("test2".to_string())
                            .label("test2".to_string())
                            .build()
                            .unwrap(),
                    )),
                ])
                .build()
                .unwrap(),
        );

        f.set_attribute(
            AttributeBuilder::default()
                .creator("system".into())
                .name("test".into())
                .hint(None)
                .hint(Some("test".into()))
                .values(vec![Value::string("1".into(), None)])
                .build()
                .unwrap(),
        );

        f.set_attribute(
            AttributeBuilder::default()
                .creator("system2".into())
                .name("test2".into())
                .hint(None)
                .values(vec![Value::string("2".into(), None)])
                .build()
                .unwrap(),
        );

        f.set_attribute(
            AttributeBuilder::default()
                .creator("system".into())
                .name("test2".into())
                .hint(Some("test".into()))
                .values(vec![Value::string("3".into(), None)])
                .build()
                .unwrap(),
        );

        f.set_attribute(
            AttributeBuilder::default()
                .creator("test".to_string())
                .name("test".to_string())
                .hint(Some("hint".to_string()))
                .values(vec![
                    Value::bytes(vec![8, 3, 8, 8], [0; 192].into(), None),
                    Value::integers([0, 1, 2, 3, 4, 5].into(), None),
                    Value::string("incoming".to_string(), Some(0.56)),
                    Value::strings(vec!["abc".into(), "cde".into()], None),
                    Value::string("outgoing".to_string(), Some(0.64)),
                    Value::none(),
                    Value::bbox(BBox::new(0.0, 0.0, 0.0, 0.0, None), None),
                    Value::bboxes(
                        vec![
                            BBox::new(0.0, 0.0, 0.0, 0.0, None),
                            BBox::new(0.0, 0.0, 0.0, 0.0, None),
                        ],
                        None,
                    ),
                    Value::float(0.0, None),
                    Value::floats(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0], None),
                    Value::points(vec![Point::new(0.0, 0.0), Point::new(0.0, 0.0)], None),
                    Value::intersection(
                        Intersection::new(
                            IntersectionKind::Enter,
                            vec![(0, Some("x1".to_string())), (1, Some("y1".to_string()))],
                        ),
                        None,
                    ),
                ])
                .build()
                .unwrap(),
        );

        f
    }
}