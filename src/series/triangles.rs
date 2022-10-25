pub struct Triangles {
    i: i64,
    s: i64,
}

impl Iterator for Triangles {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.s += self.i;
        return Some(self.s);
    }
}

pub fn triangles() -> Triangles {
    Triangles { i: 0, s: 0 }
}
