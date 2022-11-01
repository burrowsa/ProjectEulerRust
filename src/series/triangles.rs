pub struct Triangles {
    i: u32,
    s: u32,
}

impl Iterator for Triangles {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.s += self.i;
        return Some(self.s);
    }
}

pub fn triangles() -> Triangles {
    Triangles { i: 0, s: 0 }
}
