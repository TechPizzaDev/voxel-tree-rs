use std::{
    hash::{BuildHasher, Hash},
    num::TryFromIntError,
};

use foldhash::{SharedSeed, fast::FoldHasher};
use hashbrown::{HashMap, HashSet, hash_map, hash_set};

use smallvec::SmallVec;

use crate::pool::Pool;

#[derive(Clone, Default)]
pub struct ZeroHasher;
impl BuildHasher for ZeroHasher {
    type Hasher = FoldHasher<'static>;

    #[inline(always)]
    fn build_hasher(&self) -> FoldHasher<'static> {
        FoldHasher::with_seed(0, SharedSeed::global_fixed())
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct VolId(u32);
impl From<VolId> for usize {
    fn from(value: VolId) -> Self {
        value.0 as usize
    }
}
impl TryFrom<usize> for VolId {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u32::try_from(value).map(VolId)
    }
}
impl std::fmt::Debug for VolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Bucket {
    ids: SmallVec<[VolId; 4]>,
}

pub trait Volume {
    type Key;
    type Iter: Iterator<Item = Self::Key>;

    fn keys(&self) -> Self::Iter;
}

#[derive(Clone)]
pub struct VolumeHash<K, V> {
    pub buckets: HashMap<K, Bucket, ZeroHasher>,
    pub values: Pool<V>,
}

impl<K, V> VolumeHash<K, V> {
    pub fn new() -> Self {
        VolumeHash {
            buckets: HashMap::default(),
            values: Pool::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.values.iter()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.values.iter_mut()
    }

    pub fn get(&self, id: VolId) -> Option<&V> {
        self.values.get(id.into())
    }

    pub fn get_mut(&mut self, id: VolId) -> Option<&mut V> {
        self.values.get_mut(id.into())
    }

    pub fn take(&mut self, id: VolId) -> Result<V, ()> {
        match self.values.remove(id.into()) {
            Some(val) => Ok(val),
            None => Err(()),
        }
    }
}

impl<K, V> VolumeHash<K, V>
where
    K: Hash + Eq,
{
    pub fn bucket_mut(&mut self, key: K) -> &mut Bucket {
        self.buckets.entry(key).or_default()
    }

    pub fn buckets_in_volume<'s, Q: Volume<Key = K>>(
        &'s self,
        volume: &Q,
    ) -> impl Iterator<Item = &'s Bucket> {
        volume.keys().flat_map(|key| self.buckets.get(&key))
    }

    pub fn items_in_volume<'s, Q: Volume<Key = K>>(
        &'s self,
        volume: &Q,
    ) -> impl Iterator<Item = VolId> {
        self.buckets_in_volume(volume)
            .flat_map(|bucket| bucket.ids.iter().cloned())
    }

    pub fn unique_values_in_volume<'s, Q: Volume<Key = K>>(
        &'s self,
        volume: &Q,
    ) -> impl Iterator<Item = &'s V> {
        let mut items: HashSet<VolId, ZeroHasher> = HashSet::default();
        items.extend(self.items_in_volume(volume));
        items.into_iter().flat_map(|id| self.values.get(id.into()))
    }

    pub fn insert_in_volume<Q: Volume<Key = K>>(&mut self, value: V, volume: &Q) -> VolId {
        let id = VolId::try_from(self.values.insert(value)).unwrap();
        for key in volume.keys() {
            self.bucket_mut(key).ids.push(id);
        }
        id
    }

    pub fn retain_in_volume<Q, F>(&mut self, volume: &Q, mut f: F)
    where
        Q: Volume<Key = K>,
        F: FnMut(&mut V) -> bool,
    {
        let mut remove_set: HashSet<VolId, ZeroHasher> = HashSet::default();

        for key in volume.keys() {
            let hash_map::Entry::Occupied(mut bucket) = self.buckets.entry(key) else {
                continue;
            };

            bucket.get_mut().retain(|item| {
                match remove_set.entry(item) {
                    hash_set::Entry::Occupied(_) => {
                        // id is already in set; remove it from bucket
                        false
                    }
                    hash_set::Entry::Vacant(e) => {
                        let Some(val) = self.values.get_mut(item.into()) else {
                            // remove id of tomb
                            return false;
                        };

                        if f(val) {
                            true
                        } else {
                            e.insert();
                            false
                        }
                    }
                }
            });

            // TODO: add option for dropping empty buckets
            // if bucket.get().is_empty() {
            //     bucket.remove();
            // }
        }

        for item in remove_set {
            self.take(item).unwrap();
        }
    }
}

impl<K, V> VolumeHash<K, V>
where
    K: Hash + Eq,
    V: Volume<Key = K>,
{
    pub fn insert(&mut self, value: V) -> VolId {
        let keys = value.keys();
        let id = VolId::try_from(self.values.insert(value)).unwrap();
        for key in keys {
            self.bucket_mut(key).ids.push(id);
        }
        id
    }

    pub fn remove(&mut self, id: VolId) -> Result<V, ()> {
        let Some(val) = self.values.remove(id.into()) else {
            return Err(());
        };

        for key in val.keys() {
            let hash_map::Entry::Occupied(mut bucket) = self.buckets.entry(key) else {
                continue;
            };
            bucket.get_mut().retain(|other| id != other);

            // TODO: make dropping configurable
            // if bucket.get().is_empty() {
            //     bucket.remove();
            // }
        }
        Ok(val)
    }
}

impl Bucket {
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn items(&self) -> impl Iterator<Item = VolId> {
        self.ids.iter().cloned()
    }

    pub fn retain<F: FnMut(VolId) -> bool>(&mut self, mut f: F) {
        self.ids.retain(|key| f(*key));
    }
}
