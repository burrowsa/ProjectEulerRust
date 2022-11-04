pub struct Triangles {
    i: u32,
    s: u32,
}

impl Iterator for Triangles {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.s += self.i;
        Some(self.s)
    }
}

pub fn triangles() -> Triangles {
    Triangles { i: 0, s: 0 }
}

#[cfg(test)]
mod tests {
    use super::triangles;

    #[test]
    fn triangles_10() {
        let items: Vec<u32> = triangles().take(10).collect();
        assert_eq!(items, vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55])
    }
}
