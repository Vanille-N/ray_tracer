use crate::internal::*;

#[derive(Clone)]
pub struct Sky {
    map: Vec<Vec<RGB>>,
    hgt: usize,
    wth: usize,
}

impl Sky {
    pub fn new(file: &str) -> Self {
        let img = std::fs::read_to_string(file)
            .unwrap()
            .replace("\n", " ")
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut it = img.iter();
        let _ = it.next();
        let mut get = || it.next().unwrap().parse::<usize>().unwrap();
        let wth = get();
        let hgt = get();
        let max = get() as f64;
        let mut map = Vec::new();
        for _ in 0..hgt {
            let mut v = Vec::new();
            for _ in 0..wth {
                let r = get() as f64;
                let g = get() as f64;
                let b = get() as f64;
                v.push(RGB(r / max, g / max, b / max));
            }
            map.push(v);
        }
        Self { map, hgt, wth }
    }

    pub fn uniform(c: RGB) -> Self {
        Self {
            hgt: 1,
            wth: 1,
            map: vec![vec![c]],
        }
    }
}

impl Sky {
    pub fn color(&self, dir: Vec3) -> RGB {
        let (x, y) = {
            let mut v = dir;
            v.1 = 0.;
            let v = v.unit();
            (v.0, v.2)
        };
        let rise = dir.unit().1.abs();
        let mid_i = self.hgt as f64 / 2.;
        let mid_j = self.wth as f64 / 2.;
        let rad = mid_i.min(mid_j) / 1.1;
        let i = mid_i + y * rad * (1. - rise);
        let j = mid_j + x * rad * (1. - rise);
        self.map[i as usize][j as usize]
    }
}
