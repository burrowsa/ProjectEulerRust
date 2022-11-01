use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;

#[macro_export]
macro_rules! tri {
    ($($($x:expr),*);*) => {[
        $(
            &[$($x,)*][..],
        )*
    ]};
}

pub struct Triangle<'a> {
    tree: &'a [&'a [i64]],
    i_offset: usize,
    j_offset: usize,
    cache: Rc<RefCell<HashMap<(usize, usize), i64>>>, // Ideally we'd just memoize max_sum_through_triangle but memoize doesn't play well with lifetime info so implement our own caching.
}

impl<'a> Triangle<'a> {
    pub fn new(tree: &'a [&'a [i64]]) -> Triangle<'a> {
        return Triangle {
            tree,
            i_offset: 0,
            j_offset: 0,
            cache: Rc::new(RefCell::new(HashMap::new())),
        };
    }

    fn get(&self, i: usize, j: usize) -> i64 {
        return self.tree[self.i_offset + i][self.j_offset + j];
    }

    fn slice(&self, i: usize, j: usize) -> Triangle {
        return Triangle {
            tree: self.tree,
            i_offset: self.i_offset + i,
            j_offset: self.j_offset + j,
            cache: Rc::clone(&self.cache),
        };
    }

    fn len(&self) -> usize {
        return max(self.tree.len() - self.i_offset, 0);
    }

    fn _print(&self) {
        for row in 0..(self.tree.len() - self.i_offset) {
            println!(
                "{}",
                self.tree[row + self.i_offset]
                    .iter()
                    .skip(self.j_offset)
                    .take(row + 1)
                    .map(|x| format!("{},", x))
                    .collect::<String>()
            );
        }
    }
}

pub fn max_sum_through_triangle<'a>(t: Triangle<'a>) -> i64 {
    let key = (t.i_offset, t.j_offset);
    if !t.cache.borrow().contains_key(&key) {
        if t.len() > 0 {
            let x = t.get(0, 0)
                + max(
                    max_sum_through_triangle(t.slice(1, 0)),
                    max_sum_through_triangle(t.slice(1, 1)),
                );
            t.cache.borrow_mut().insert(key, x);
        } else {
            t.cache.borrow_mut().insert(key, 0);
        }
    }
    return *t.cache.borrow().get(&key).unwrap();
}
