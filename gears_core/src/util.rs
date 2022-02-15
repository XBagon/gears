use slotmap::SlotMap;
use std::ops::{Index, IndexMut};

pub struct LiftSlotMap<K: slotmap::Key, V>(SlotMap<K, Option<V>>);

impl<K: slotmap::Key, V> From<SlotMap<K, Option<V>>> for LiftSlotMap<K, V> {
    fn from(slot_map: SlotMap<K, Option<V>>) -> Self {
        Self(slot_map)
    }
}

impl<K: slotmap::Key, V> Index<K> for LiftSlotMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.0.index(index).as_ref().expect("Accessed lifted slot!")
    }
}

impl<K: slotmap::Key, V> IndexMut<K> for LiftSlotMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.0
            .index_mut(index)
            .as_mut()
            .expect("Accessed lifted slot!")
    }
}

impl<'a, K: slotmap::Key, V> LiftSlotMap<K, V> {
    pub fn with_key() -> Self {
        Self(SlotMap::with_key())
    }

    pub fn insert(&mut self, value: V) -> K {
        self.0.insert(Some(value))
    }

    pub fn iter(&mut self) -> impl Iterator<Item = (K, &V)> {
        self.0
            .iter()
            .map(|(k, v)| v.as_ref().map(|v| (k, v)))
            .flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.0
            .iter_mut()
            .map(|(k, v)| v.as_mut().map(|v| (k, v)))
            .flatten()
    }

    pub fn do_lifted(&mut self, key: K, mut f: impl FnMut(&mut Self, &mut V)) {
        let mut value = self.0.get_mut(key).unwrap().take().unwrap();
        f(self, &mut value);
        *self.0.get_mut(key).unwrap() = Some(value);
    }
}

#[cfg(test)]
#[test]
fn lift_test() {
    use crate::gear::GearId;

    let mut map = LiftSlotMap::<GearId, _>::with_key();
    let a = map.insert(String::from("abc"));
    let b = map.insert(String::from("def"));
    let c = map.insert(String::from("ghi"));
    map.do_lifted(a, |map, a| {
        for (_key, val) in map.iter_mut() {
            val.push(a.pop().unwrap())
        }
    });
    assert_eq!(map[a].len(), 1);
    assert_eq!(map[b].len(), 4);
    assert_eq!(map[c].len(), 4);
}
