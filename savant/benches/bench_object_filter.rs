#![feature(test)]

extern crate test;

use savant_rs::primitives::message::video::object::{
    InnerVideoObjectBuilder, VideoObjectTrackingData,
};
use savant_rs::primitives::message::video::query::*;
use savant_rs::primitives::{AttributeBuilder, AttributeValue, RBBox, VideoObject};
use test::Bencher;

fn get_objects() -> Vec<VideoObject> {
    (0..100)
        .into_iter()
        .map(|i| {
            let mut o = InnerVideoObjectBuilder::default()
                .creator(format!("created_by_{i}"))
                .label(format!("label_{i}"))
                .id(i)
                .confidence(Some(0.53))
                .bbox(RBBox::new(0.0, 0.0, 1.0, 1.0, None))
                .track(Some(VideoObjectTrackingData::new(
                    i,
                    RBBox::new(10.0, 20.0, 21.0, 231.0, None),
                )))
                .build()
                .unwrap();
            o.attributes.insert(
                ("test".to_string(), "test".to_string()),
                AttributeBuilder::default()
                    .creator("test".to_string())
                    .name("test".to_string())
                    .hint(Some("hint".to_string()))
                    .values(vec![AttributeValue::integer(1, None)])
                    .build()
                    .unwrap(),
            );
            VideoObject::from_inner_object(o)
        })
        .collect::<Vec<_>>()
}

#[bench]
fn bench_filtering(b: &mut Bencher) {
    use Query::*;

    let attr_exp = AttributesJMESQuery("[?(name=='test' && creator=='test')]".into());

    let expr = and![
        or![
            Creator(one_of(&["created_by_2", "created_by_4"])),
            or![
                Label(ends_with("2")),
                Label(ends_with("4")),
                Label(ends_with("6")),
            ]
        ],
        BoxAngleDefined,
        ParentDefined,
        attr_exp,
        or![Confidence(ge(0.6)), Confidence(le(0.4)),]
    ];

    let objs = get_objects();
    b.iter(|| {
        let _ = objs.iter().map(|o| expr.execute(o)).collect::<Vec<_>>();
    });
}

#[bench]
fn bench_empty_filtering(b: &mut Bencher) {
    let expr = Query::Idle;
    let objs = get_objects();
    b.iter(|| {
        let _ = objs.iter().map(|o| expr.execute(o)).collect::<Vec<_>>();
    });
}

#[bench]
fn bench_simple_filtering(b: &mut Bencher) {
    use Query::*;
    let expr = or![
        Creator(eq("created_by_20")),
        Creator(ends_with("created_by_10")),
    ];

    let objs = get_objects();
    b.iter(|| {
        let _ = objs.iter().map(|o| expr.execute(o)).collect::<Vec<_>>();
    });
}
