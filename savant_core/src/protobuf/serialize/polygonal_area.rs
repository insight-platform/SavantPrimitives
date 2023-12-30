use crate::primitives::PolygonalArea;
use crate::protobuf::generated;

impl From<&PolygonalArea> for generated::PolygonalArea {
    fn from(poly: &PolygonalArea) -> Self {
        let points = poly
            .get_vertices()
            .iter()
            .map(|p| generated::Point { x: p.x, y: p.y })
            .collect();

        let tags = poly.get_tags().map(|tags| generated::PolygonalAreaTags {
            tags: tags
                .iter()
                .map(|t| generated::PolygonalAreaTag { tag: t.clone() })
                .collect(),
        });

        generated::PolygonalArea { points, tags }
    }
}

impl From<&generated::PolygonalArea> for PolygonalArea {
    fn from(value: &generated::PolygonalArea) -> Self {
        let points = value
            .points
            .iter()
            .map(|p| crate::primitives::Point::new(p.x, p.y))
            .collect();

        let tags = value
            .tags
            .as_ref()
            .map(|tags| tags.tags.iter().map(|t| t.tag.clone()).collect());

        PolygonalArea::new(points, tags)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_polygonal_area() {
        use crate::primitives::PolygonalArea;
        use crate::protobuf::generated;

        assert_eq!(
            PolygonalArea::new(
                vec![
                    crate::primitives::Point::new(1.0, 2.0),
                    crate::primitives::Point::new(3.0, 4.0),
                ],
                None,
            ),
            PolygonalArea::from(&generated::PolygonalArea {
                points: vec![
                    generated::Point { x: 1.0, y: 2.0 },
                    generated::Point { x: 3.0, y: 4.0 },
                ],
                tags: None,
            })
        );
        assert_eq!(
            PolygonalArea::new(
                vec![
                    crate::primitives::Point::new(1.0, 2.0),
                    crate::primitives::Point::new(3.0, 4.0),
                ],
                Some(vec![Some("tag1".to_string()), Some("tag2".to_string())]),
            ),
            PolygonalArea::from(&generated::PolygonalArea {
                points: vec![
                    generated::Point { x: 1.0, y: 2.0 },
                    generated::Point { x: 3.0, y: 4.0 },
                ],
                tags: Some(generated::PolygonalAreaTags {
                    tags: vec![
                        generated::PolygonalAreaTag {
                            tag: Some("tag1".to_string()),
                        },
                        generated::PolygonalAreaTag {
                            tag: Some("tag2".to_string()),
                        },
                    ],
                }),
            })
        );
    }
}
