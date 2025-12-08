use glam::Vec3;
use miette::miette;
use std::str::FromStr;

fn vec3_from_str(s: &str) -> Result<Vec3, String> {
    let coords = s
        .trim()
        .split(',')
        .map(|x| {
            x.parse::<f32>()
                .map_err(|_| "Invalid coordinate value".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    if coords.len() == 3 {
        let vec = Vec3::new(coords[0], coords[1], coords[2]);
        return Ok(vec);
    }

    Err("Expected exactly 3 coordinates".to_string())
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, mut x: usize, mut y: usize) {
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return;
        }
        if self.size[x] < self.size[y] {
            (x, y) = (y, x);
        }
        self.parent[y] = x;
        self.size[x] += self.size[y];
    }
}

#[derive(Debug, Clone)]
struct Coordinates(Vec<Vec3>);

impl Coordinates {
    fn solve(&self) -> usize {
        let n = self.0.len();

        let mut pairs = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let dist = self.0[i].distance(self.0[j]);
                pairs.push((dist, i, j));
            }
        }

        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut uf = UnionFind::new(n);
        let mut last_pair = (0, 0);
        let mut num_circuits = n;

        for (_, i, j) in &pairs {
            if uf.find(*i) != uf.find(*j) {
                uf.union(*i, *j);
                last_pair = (*i, *j);
                num_circuits -= 1;

                if num_circuits == 1 {
                    break;
                }
            }
        }
        (self.0[last_pair.0].x as usize) * (self.0[last_pair.1].x as usize)
    }
}

impl FromStr for Coordinates {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .lines()
            .map(vec3_from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(coords))
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let coords = Coordinates::from_str(input).map_err(|e| miette!("{e}"))?;
    Ok(coords.solve())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let result = 25272;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
