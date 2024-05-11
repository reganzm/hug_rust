use std::collections::HashMap;
use std::mem::swap;

type LRUHandle<T> = *mut LRUNode<T>;

struct LRUNode<T> {
    next: Option<Box<LRUNode<T>>>,
    prev: Option<*mut LRUNode<T>>,
    data: Option<T>
}

struct LRUList<T> {
    head: LRUNode<T>,
    count: usize,
}

impl<T> LRUList<T> {
    fn new() -> LRUList<T> {
        LRUList {
            head: LRUNode {
                next: None,
                prev: None,
                data: None
            },
            count: 0
        }
    }

    fn insert(&mut self, elem: T) -> LRUHandle<T> {
        self.count += 1;

        // not first node
        if self.head.next.is_some() {
            let mut new = Box::new(LRUNode {
                data: Some(elem),
                next: None,
                prev: Some(&mut self.head as *mut LRUNode<T>),
            });
            let newp = new.as_mut() as *mut LRUNode<T>;

            self.head.next.as_mut().unwrap().prev = Some(newp);
            new.next = self.head.next.take();
            self.head.next = Some(new);

            newp
        } else {
            // first node
            let mut new = Box::new(LRUNode {
                data: Some(elem),
                next: None,
                prev: Some(&mut self.head as *mut LRUNode<T>),
            });
            let newp = new.as_mut() as *mut LRUNode<T>;

            self.head.prev = Some(newp);
            self.head.next = Some(new);

            newp
        }
    }

    fn remove(&mut self, node_handle: LRUHandle<T>) -> T {
        unsafe {
            let d = (*node_handle).data.take().unwrap();

            let mut current = (*(*node_handle).prev.unwrap()).next.take().unwrap();

            let prev = current.prev.unwrap();

            if current.next.is_some() {
                current.next.as_mut().unwrap().prev = current.prev.take();
            }

            (*prev).next = current.next.take();

            self.count -= 1;

            d
        }
    }

    fn reinsert_front(&mut self, node_handle: LRUHandle<T>) {
        unsafe  {
            let prevp = (*node_handle).prev.unwrap();

            if let Some(next) = (*node_handle).next.as_mut() {
                next.prev = Some(prevp)
            } else {
                self.head.prev = Some(prevp)
            }

            swap(&mut (*prevp).next, &mut (*node_handle).next);
            swap(&mut (*node_handle).next, &mut self.head.next);

            if let Some(ref mut newnext) = (*node_handle).next {
                (*node_handle).prev = newnext.prev;
                newnext.prev = Some(node_handle)
            } else {
                self.head.prev = Some(node_handle)
            }
        }
    }

    fn remove_last(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        let mut lasto = unsafe {(*(*self.head.prev.unwrap()).prev.unwrap()).next.take() };

        if let Some(ref mut last) = lasto {
            self.head.prev = last.prev;
            self.count -= 1;
            (*last).data.take()
        } else {
            None
        }
    }

    fn count(&self) -> usize {self.count}
}

pub type CacheKey = [u8; 16];
type CacheEntry<T> = (T, LRUHandle<CacheKey>);

pub struct Cache<T> {
    list: LRUList<CacheKey>,
    map: HashMap<CacheKey, CacheEntry<T>>,
    cap: usize,
}

impl<T> Cache<T> {
    pub fn new(cap: usize) -> Cache<T> {
        Cache {
            list: LRUList::new(),
            map: HashMap::with_capacity(1024),
            cap
        }
    }

    pub fn insert(&mut self, key: &CacheKey, elem: T) {
        if self.list.count >= self.cap {
            if let Some(removed_key) = self.list.remove_last() {
                self.map.remove(&removed_key).unwrap();
            } else {
                println!("failed get removed key");
            }
        }

        let lru_handle = self.list.insert(*key);
        self.map.insert(*key, (elem, lru_handle));
    }

    pub fn get(&mut self, key: &CacheKey) -> Option<&T> {
        match self.map.get(key) {
            None => None,
            Some(&(ref elem, ref lru_handle)) => {
                self.list.reinsert_front(*lru_handle);
                Some(elem)
            }
        }
    }

    pub fn remove(&mut self, key: &CacheKey) -> Option<T> {
        match self.map.remove(key) {
            None => None,
            Some((elem, lru_handle)) => {
                self.list.remove(lru_handle);
                Some(elem)
            }
        }
    }

    pub fn count(&self) -> usize {self.list.count}
}

#[cfg(test)]
mod test {
    use super::{LRUList, *};

    fn make_key(a: u8, b: u8, c: u8) -> CacheKey {
        [a, b, c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    }

    #[test]
    fn test_cache_add_rm() {
        let mut cache = Cache::new(128);

        let h_123 = make_key(1,2,3);
        let h_125 = make_key(1,2,5);
        let h_372 = make_key(3,7,2);
        let h_989 = make_key(9,8,9);

        cache.insert(&h_123, 123);
        cache.insert(&h_125, 125);
        cache.insert(&h_372, 372);
        cache.insert(&h_989, 989);

        assert_eq!(cache.count(), 4);

        assert_eq!(cache.get(&h_123).unwrap(), &123);
        assert_eq!(cache.get(&h_125).unwrap(), &125);
        assert_eq!(cache.get(&h_372).unwrap(), &372);
        assert_eq!(cache.get(&h_989).unwrap(), &989);

        assert_eq!(cache.remove(&h_123).unwrap(), 123);
        assert_eq!(cache.count(), 3);

    }
}


fn main() {
    println!("Hello, world!");
}
