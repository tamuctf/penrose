use slab::Slab;
use std::num::NonZeroU32;
use crate::intersection_point::IntersectionPoint;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Key(NonZeroU32);

pub(crate) struct Chunk(Slab<IntersectionPoint>);

impl Chunk {
    pub(crate) fn new() -> Self {
        Self(Slab::new())
    }
    pub(crate) fn insert(&mut self, point: IntersectionPoint) -> Option<Key> {
        // TODO: determine what to do for overflow behavior
        let raw = self.0.insert(point);
        NonZeroU32::new(raw as u32 + 1).map(Key)
    }
    pub(crate) fn resolve(&self, key: Key) -> &IntersectionPoint {
        let key = key.0.get() - 1;
        &self.0[key as usize]
    }
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }
}

thread_local! {
    pub(crate) static CHUNK: RefCell<Chunk> = RefCell::new(Chunk::new());
}


