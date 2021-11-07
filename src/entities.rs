use std::collections::VecDeque;

pub struct Entities<T, const GROUP: u8> {
    vec: Vec<Option<T>>,
    free: VecDeque<GenerationalIndex>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct GenerationalIndex {
    generation: u64,
    index: u64,
}

impl GenerationalIndex {
    pub const fn single(index: usize) -> Self {
        GenerationalIndex {
            generation: 0,
            index: index as u64,
        }
    }

    fn new(group: u8, generation: u64, index: usize) -> Self {
        let generation = ((group as u64) << 56) | generation;
        let index = index as u64;
        GenerationalIndex { generation, index }
    }

    pub fn group(&self) -> u8 {
        (self.generation >> 56) as u8
    }

    fn generation(&self) -> u64 {
        (self.generation << 8) >> 8
    }

    fn index(&self) -> usize {
        self.index as usize
    }

    pub fn to_u128(self) -> u128 {
        (self.generation as u128) << 64 | (self.index as u128)
    }

    pub fn from_u128(num: u128) -> Self {
        let generation = (num >> 64) as u64;
        let index = ((num << 64) >> 64) as u64;
        GenerationalIndex { generation, index }
    }
}

impl std::fmt::Debug for GenerationalIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenerationalIndex")
            .field("group", &self.group())
            .field("generation", &self.generation())
            .field("index", &self.index())
            .finish()
    }
}

impl<T, const GROUP: u8> Entities<T, GROUP> {
    pub fn new() -> Self {
        debug_assert!(GROUP != 0, "entities's group cannot be 0");
        Entities {
            vec: Vec::new(),
            free: VecDeque::new(),
        }
    }

    pub fn push(&mut self, new: impl FnOnce(GenerationalIndex) -> T) -> GenerationalIndex {
        if let Some(mut idx) = self.free.pop_front() {
            idx.generation += 1;
            self.vec[idx.index()] = Some(new(idx));
            idx
        } else {
            let idx = GenerationalIndex::new(GROUP, 0, self.vec.len());
            self.vec.push(Some(new(idx)));
            idx
        }
    }

    pub fn remove(&mut self, idx: GenerationalIndex) {
        debug_assert!(
            idx.group() == GROUP,
            "index's group doesn't match entities's group"
        );
        self.vec[idx.index()] = None;
        self.free.push_back(idx);
    }
}

impl<T, const GROUP: u8> Default for Entities<T, GROUP> {
    fn default() -> Self {
        Entities::new()
    }
}

impl<T, const GROUP: u8> std::ops::Index<GenerationalIndex> for Entities<T, GROUP> {
    type Output = T;

    fn index(&self, idx: GenerationalIndex) -> &T {
        debug_assert!(
            idx.group() == GROUP,
            "index's group doesn't match entities's group"
        );
        self.vec[idx.index()].as_ref().unwrap()
    }
}

impl<T, const GROUP: u8> std::ops::IndexMut<GenerationalIndex> for Entities<T, GROUP> {
    fn index_mut(&mut self, idx: GenerationalIndex) -> &mut T {
        debug_assert!(
            idx.group() == GROUP,
            "index's group doesn't match entities's group"
        );
        self.vec[idx.index()].as_mut().unwrap()
    }
}

pub struct EntitiesIter<'a, T>(std::slice::Iter<'a, Option<T>>);

impl<'a, T> Iterator for EntitiesIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            match self.0.next() {
                Some(None) => {}
                Some(item) => break item.as_ref(),
                None => break None,
            }
        }
    }
}

impl<'a, T, const GROUP: u8> IntoIterator for &'a Entities<T, GROUP> {
    type Item = &'a T;
    type IntoIter = EntitiesIter<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        EntitiesIter(self.vec.iter())
    }
}

pub struct EntitiesIterMut<'a, T>(std::slice::IterMut<'a, Option<T>>);

impl<'a, T> Iterator for EntitiesIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        loop {
            match self.0.next() {
                Some(None) => {}
                Some(item) => break item.as_mut(),
                None => break None,
            }
        }
    }
}

impl<'a, T, const GROUP: u8> IntoIterator for &'a mut Entities<T, GROUP> {
    type Item = &'a mut T;
    type IntoIter = EntitiesIterMut<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        EntitiesIterMut(self.vec.iter_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_not_a_group_of_zero_for_entities() {
        Entities::<(), 0>::new();
    }

    #[test]
    fn should_allow_adding_and_indexing_entities() {
        let mut ent: Entities<u8, 1> = Entities::new();
        let a = ent.push(|_| 1);
        let b = ent.push(|_| 2);
        let c = ent.push(|_| 3);

        assert_eq!(ent[a], 1);
        assert_eq!(ent[b], 2);
        assert_eq!(ent[c], 3);
    }

    #[test]
    fn should_allow_iterating_over_entities() {
        let mut ent: Entities<u8, 1> = Entities::new();
        ent.push(|_| 1);
        ent.push(|_| 2);
        ent.push(|_| 3);

        assert_eq!(ent.into_iter().collect::<Vec<&u8>>(), vec![&1, &2, &3]);
    }

    #[test]
    fn should_skip_holes_when_iterating() {
        let mut ent: Entities<u8, 1> = Entities::new();
        ent.push(|_| 1);
        let b = ent.push(|_| 2);
        ent.push(|_| 3);
        let d = ent.push(|_| 4);
        ent.push(|_| 5);

        ent.remove(b);
        ent.remove(d);

        assert_eq!(ent.into_iter().collect::<Vec<&u8>>(), vec![&1, &3, &5]);
    }

    #[test]
    fn should_reuse_holes_when_inserting() {
        let mut ent: Entities<u8, 1> = Entities::new();
        ent.push(|_| 1);
        let b = ent.push(|_| 2);
        ent.push(|_| 3);
        let d = ent.push(|_| 4);
        ent.push(|_| 5);

        ent.remove(b);
        ent.remove(d);

        ent.push(|_| 6);
        ent.push(|_| 7);

        assert_eq!(
            ent.into_iter().collect::<Vec<&u8>>(),
            vec![&1, &6, &3, &7, &5]
        );
    }

    #[test]
    fn should_return_a_different_index_for_reused_holes() {
        let mut ent: Entities<u8, 1> = Entities::new();
        ent.push(|_| 1);

        let old = ent.push(|_| 2);
        ent.remove(old);

        let new = ent.push(|_| 3);

        assert_eq!(ent.into_iter().collect::<Vec<&u8>>(), vec![&1, &3]);
        assert_eq!(old.index(), new.index());
        assert_ne!(old.generation(), new.generation());
        assert_ne!(old, new);
    }

    #[test]
    #[should_panic]
    fn should_not_allow_using_an_index_when_group_doesnt_match() {
        let mut ent_a: Entities<u8, 1> = Entities::new();
        let mut ent_b: Entities<u8, 2> = Entities::new();

        let idx = ent_a.push(|_| 1);
        ent_b.push(|_| 1);

        let _ = ent_b[idx];
    }

    #[test]
    fn should_allow_converting_indexes_to_and_from_u128() {
        let mut ent: Entities<u8, 1> = Entities::new();
        let a = ent.push(|_| 1);
        let b = ent.push(|_| 2);
        let c = ent.push(|_| 3);

        assert_eq!(a, GenerationalIndex::from_u128(a.to_u128()));
        assert_eq!(b, GenerationalIndex::from_u128(b.to_u128()));
        assert_eq!(c, GenerationalIndex::from_u128(c.to_u128()));
    }
}
