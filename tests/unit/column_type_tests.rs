use super::*;

#[test]
fn vectors_compare_their_dimensions() {
    assert_ne!(ColumnType::Vector(Some(3)), ColumnType::Vector(Some(1536)));
    assert_ne!(ColumnType::Vector(Some(3)), ColumnType::Vector(None));
    assert_eq!(ColumnType::Vector(Some(3)), ColumnType::Vector(Some(3)));
    assert_eq!(ColumnType::Vector(None), ColumnType::Vector(None));
}
