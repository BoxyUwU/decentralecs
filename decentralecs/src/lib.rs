#![forbid(unsafe_code)]

mod entities;
mod joinable;
mod storage;
mod world;

pub use entities::Entity;
pub use joinable::{ColumnIterator, Joinable, Maybe, Unsatisfied, WithEntities};
pub use not_ghost_cell::{SlowGhostCell, SlowGhostToken};
pub use storage::{Columns, ColumnsApi};
pub use world::{Archetype, EcsTypeId, EntityBuilder, World, WorldId};

pub fn get_two_mut<T>(vec: &mut [T], idx_1: usize, idx_2: usize) -> (&mut T, &mut T) {
    use std::cmp::Ordering;
    match idx_1.cmp(&idx_2) {
        Ordering::Less => {
            let (left, right) = vec.split_at_mut(idx_2);
            (&mut left[idx_1], &mut right[0])
        }
        Ordering::Greater => {
            let (left, right) = vec.split_at_mut(idx_1);
            (&mut right[0], &mut left[idx_2])
        }
        Ordering::Equal => {
            panic!("")
        }
    }
}

pub fn assert_world_id(id1: WorldId, id2: WorldId, type_name: &str) {
    if id1 != id2 {
        panic!(
            "[Mismatched WorldIds]: attempt to use World: WorldId({}) with {}: WorldId({})",
            id1.inner(),
            type_name,
            id2.inner(),
        )
    }
}
