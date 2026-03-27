use core::{
    mem,
    ops::{Index, IndexMut},
};

#[derive(Clone)]
pub struct Pool<T> {
    items: Vec<Option<T>>,
    tombs: Vec<usize>,
}
impl<T> Pool<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len() - self.tombs.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn insert(&mut self, item: T) -> usize {
        if let Some(index) = self.tombs.pop() {
            self.items[index] = Some(item);
            index
        } else {
            self.items.push(Some(item));
            self.items.len() - 1
        }
    }

    #[inline]
    pub fn reserve(&mut self) -> usize {
        if let Some(index) = self.tombs.pop() {
            self.items[index] = None;
            index
        } else {
            self.items.push(None);
            self.items.len() - 1
        }
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<T> {
        match mem::replace(&mut self.items[index], None) {
            Some(item) => {
                self.tombs.push(index);
                Some(item)
            }
            None => None,
        }
    }

    pub fn replace(&mut self, index: usize, item: T) -> Option<T> {
        self.items[index].replace(item)
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index).and_then(Option::as_ref)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index).and_then(Option::as_mut)
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe { self.items.get_unchecked(index).as_ref().unwrap_unchecked() }
    }

    #[inline]
    pub unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        unsafe {
            self.items
                .get_unchecked_mut(index)
                .as_mut()
                .unwrap_unchecked()
        }
    }

    // TODO: impl ExactSizeIterator since we know the exact len, but flat_map does not

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().flat_map(std::convert::identity)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut().flat_map(std::convert::identity)
    }

    pub fn slots(&self) -> impl Iterator<Item = (usize, Option<&T>)> {
        self.items.iter().enumerate().map(|(i, v)| (i, v.as_ref()))
    }

    pub fn slots_mut(&mut self) -> impl Iterator<Item = (usize, Option<&mut T>)> {
        self.items.iter_mut().enumerate().map(|(i, v)| (i, v.as_mut()))
    }
}
impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            tombs: Vec::new(),
        }
    }
}

impl<T> Index<usize> for Pool<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<T> IndexMut<usize> for Pool<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
