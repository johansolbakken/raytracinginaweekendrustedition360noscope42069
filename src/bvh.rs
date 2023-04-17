use crate::aabb::Aabb;

pub struct BvhNode {
    left: Box<BvhNode>,
    right: Box<BvhNode>,
    object_index: usize,
    bounding_box: Aabb,
}

impl BvhNode {}
