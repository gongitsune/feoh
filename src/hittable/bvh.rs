use rand::Rng;

use super::{aabb::AABB, HitRecord, Hittable};
use crate::{ray::Ray, Rand};
use core::cmp::Ordering;
use std::fmt;

pub struct BvhTree<'a> {
    nodes: Vec<BvhNode<'a>>,
    root: NodeId,
}

struct BvhNode<'a> {
    left: Option<NodeId>,
    right: Option<NodeId>,
    aabb: Option<AABB>,
    hittable: Option<&'a Box<dyn Hittable>>,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeId {
    index: usize,
}

impl<'a> BvhTree<'a> {
    fn hit(&self, id: NodeId, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let node = &self.nodes[id.index];

        if node.aabb.is_none() || node.aabb.is_some() && node.aabb.unwrap().hit(r, t_min, t_max) {
            match node.hittable {
                Some(ref hittable) => return hittable.hit(r, t_min, t_max),
                None => {}
            }

            let mut hit_left: Option<HitRecord> = None;
            let mut hit_right: Option<HitRecord> = None;

            if let Some(ref left_index) = node.left {
                hit_left = self.hit(*left_index, r, t_min, t_max);
            }

            if let Some(ref right_index) = node.right {
                hit_right = self.hit(*right_index, r, t_min, t_max);
            }

            match hit_left {
                Some(ref left) => match hit_right {
                    Some(ref right) => {
                        if left.t < right.t {
                            return hit_left;
                        } else {
                            return hit_right;
                        }
                    }
                    None => return hit_left,
                },
                None => {}
            }

            match hit_right {
                Some(_) => return hit_right,
                None => {}
            }
        }

        None
    }
}

impl<'a> Hittable for BvhTree<'a> {
    fn bounding_box(&self, _: (f32, f32)) -> Option<AABB> {
        self.nodes[self.root.index].aabb
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hit(self.root, ray, t_min, t_max)
    }
}

impl<'a> BvhTree<'a> {
    pub fn new(l: &'a mut [Box<dyn Hittable>], time: (f32, f32), rng: &mut Rand) -> BvhTree<'a> {
        let mut tree = BvhTree {
            nodes: Vec::new(),
            root: NodeId { index: 0 },
        };
        tree.root = tree.build(l, time, rng);

        tree
    }

    fn build(
        &mut self,
        l: &'a mut [Box<dyn Hittable>],
        time: (f32, f32),
        rng: &mut Rand,
    ) -> NodeId {
        let axis: i32 = rng.gen_range(0..3);

        match axis {
            0 => l.sort_by(|a, b| box_x_compare(a, b)),
            1 => l.sort_by(|a, b| box_y_compare(a, b)),
            2 => l.sort_by(|a, b| box_z_compare(a, b)),
            _ => panic!("Unexpected axis"),
        }

        let left: NodeId;
        let right: NodeId;

        if l.len() == 1 {
            return self.new_leaf(&l[0], time);
        } else if l.len() == 2 {
            left = self.new_leaf(&l[0], time);
            right = self.new_leaf(&l[1], time);
        } else {
            let half_len = l.len() / 2;
            let (left_hittables, right_hittables) = l.split_at_mut(half_len);

            left = self.build(left_hittables, time, rng);
            right = self.build(right_hittables, time, rng);
        }

        if let Some(left_box) = self.nodes[left.index].aabb {
            if let Some(right_box) = self.nodes[right.index].aabb {
                return self.new_node(
                    AABB::surrounding_box(&left_box, &right_box),
                    Some(left),
                    Some(right),
                );
            }
        }

        panic!("No bounding box in BvhNode::build");
    }

    fn new_leaf(&mut self, hittable: &'a Box<dyn Hittable>, time: (f32, f32)) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left: None,
            right: None,
            aabb: hittable.bounding_box(time),
            hittable: Some(hittable),
        });

        return NodeId { index: next_index };
    }

    fn new_node(&mut self, aabb: AABB, left: Option<NodeId>, right: Option<NodeId>) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left,
            right,
            aabb: Some(aabb),
            hittable: None,
        });

        return NodeId { index: next_index };
    }

    fn number_hittables(&self, id: NodeId) -> usize {
        let node = &self.nodes[id.index];
        let local_hittable = if node.hittable.is_some() { 1 } else { 0 };
        let count_left = if let Some(left_index) = node.left {
            self.number_hittables(left_index)
        } else {
            0
        };
        let count_right = if let Some(right_index) = node.right {
            self.number_hittables(right_index)
        } else {
            0
        };

        local_hittable + count_left + count_right
    }
}

impl<'a> fmt::Display for BvhTree<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BVH with {:?} hittables and {:?} nodes",
            self.number_hittables(self.root),
            self.nodes.len()
        )
    }
}

fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    const TIME: (f32, f32) = (0., 0.);
    if let Some(box_left) = a.bounding_box(TIME) {
        if let Some(box_right) = b.bounding_box(TIME) {
            if let Some(cmp) = box_left.min.x.partial_cmp(&box_right.min.x) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}

fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    const TIME: (f32, f32) = (0., 0.);
    if let Some(box_left) = a.bounding_box(TIME) {
        if let Some(box_right) = b.bounding_box(TIME) {
            if let Some(cmp) = box_left.min.y.partial_cmp(&box_right.min.y) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}

fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    const TIME: (f32, f32) = (0., 0.);
    if let Some(box_left) = a.bounding_box(TIME) {
        if let Some(box_right) = b.bounding_box(TIME) {
            if let Some(cmp) = box_left.min.z.partial_cmp(&box_right.min.z) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}
