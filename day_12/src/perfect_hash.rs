use std::usize;

pub struct HashMap<V> {
    inner: [Option<V>; u8::MAX as usize],
}

impl<V> HashMap<V> {
    pub fn new() -> Self {
        Self {
            inner: [const { None }; u8::MAX as usize],
        }
    }

    pub fn get(&self, key: u8) -> Option<&V> {
        self.inner[key as usize].as_ref()
    }

    pub fn get_mut(&mut self, key: u8) -> Option<&mut V> {
        self.inner[key as usize].as_mut()
    }

    pub fn insert(&mut self, key: u8, val: V) {
        let _ = self.inner[key as usize].insert(val);
    }

    pub fn remove(&mut self, key: u8) -> Option<V> {
        self.inner[key as usize].take()
    }

    pub fn entry(&mut self, key: u8) -> Entry<V> {
        Entry(&mut self.inner[key as usize])
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &V> {
        self.inner.iter().filter_map(|v| v.as_ref())
    }

    pub fn iter(&self) -> impl Iterator<Item = (u8, &V)> {
        self.inner
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.as_ref().map(|v| (i as u8, v)))
    }
}

pub struct Entry<'a, V: 'a>(&'a mut Option<V>);

impl<'a, V: 'a> Entry<'a, V> {
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        if let Some(v) = self.0.as_mut() {
            f(v)
        }
        self
    }

    pub fn or_insert(self, default: V) -> &'a mut V {
        self.0.get_or_insert_with(|| default)
    }
}
