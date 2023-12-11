use std::collections::{HashSet,HashMap};
use std::cmp::max;

pub fn run(file: &str,factor:usize) -> (usize,usize) {
    let data = read_data(file);
    (pt1(data.clone()),pt2(data,factor))
}

fn read_data(file: &str) -> Universe {
    let mut seen = HashSet::new();
    let mut n_rows: usize = 0;
    let mut n_cols: usize = 0;
    
    for (row,l) in std::fs::read_to_string(file).expect("Failed to read file").lines().enumerate() {
        n_rows = max(n_rows,row);
        for (col,ch) in l.chars().enumerate() {
            n_cols = max(n_cols,col);
            if ch == '#' {
                seen.insert((row,col));
            }
        }

    }

    Universe{
        galaxies: seen,
        n_rows,
        n_cols,
        empty_rows: Vec::new(),
        empty_cols: Vec::new(),
    }
}

fn pt1(mut data: Universe) -> usize {
    data.expand();
    let mut dists: HashMap<((usize,usize),(usize,usize)),usize> = HashMap::new();
    for &g in data.galaxies.iter() { 
        for &cmp in data.galaxies.iter() {
            if g == cmp {
                continue
            }
            let entry = if g < cmp {
                (g,cmp)
            } else {
                (cmp,g)
            };

            if !dists.contains_key(&entry) {
                dists.insert(entry,data.calc_min_dist(&g,&cmp));
            }
        }
    }
    dists.values().sum()

}

fn pt2(mut data: Universe,factor:usize) -> usize {
    data.expand();
    let mut dists: HashMap<((usize,usize),(usize,usize)),usize> = HashMap::new();
    for &g in data.galaxies.iter() { 
        for &cmp in data.galaxies.iter() {
            if g == cmp {
                continue
            }
            let entry = if g < cmp {
                (g,cmp)
            } else {
                (cmp,g)
            };

            if !dists.contains_key(&entry) {
                dists.insert(entry,data.calc_min_dist_big(&g,&cmp,factor));
            }
        }
    }
    dists.values().sum()

}


#[derive(Debug,Clone)]
struct Universe {
    galaxies: HashSet<(usize,usize)>,
    n_rows: usize,
    n_cols: usize,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Universe {
    fn expand(&mut self) {
        self.find_empty_rows();
        self.find_empty_cols();
    }

    fn list_galaxies(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for g in &self.galaxies{
            res.push(*g);
        }
        res
    }
    
    fn find_empty_rows(&mut self) { 
        for r in 0..=self.n_rows {
            let mut empty = true;
            for col in 0..=self.n_cols{
                if self.galaxies.contains(&(r,col)) {
                    empty = false;
                }
            }
            if empty {
                self.empty_rows.push(r);
            }
        }
    }
    fn find_empty_cols(&mut self) { 
        for col in 0..=self.n_cols {
            let mut empty = true;
            for r in 0..=self.n_rows{
                if self.galaxies.contains(&(r,col)) {
                    empty = false;
                }
            }
            if empty {
                self.empty_cols.push(col);
            }
        }
    }
    
    fn get_true_loc(&self, gal: &(usize,usize)) -> (usize,usize) {
        let exp_rows = self.empty_rows.iter().filter(|&r| *r < gal.0).count();
        let exp_cols = self.empty_cols.iter().filter(|&c| *c < gal.1).count();
        (gal.0+exp_rows, gal.1+exp_cols)
    }

    fn calc_min_dist(&self, base: &(usize,usize), cmp: &(usize, usize)) -> usize {
        let ref_exp = self.get_true_loc(base);
        let cmp_exp = self.get_true_loc(cmp);
        let x = if ref_exp.0 > cmp_exp.0 {
            ref_exp.0 - cmp_exp.0
        } else {
            cmp_exp.0 - ref_exp.0
        };
        let y = if ref_exp.1 > cmp_exp.1 {
            ref_exp.1 - cmp_exp.1
        } else {
            cmp_exp.1 - ref_exp.1
        };

        x + y
    }

    fn get_true_loc_big(&self, gal: &(usize,usize),factor: usize) -> (usize,usize) {
        let exp_rows = self.empty_rows.iter().filter(|&r| *r < gal.0).count();
        let exp_cols = self.empty_cols.iter().filter(|&c| *c < gal.1).count();
        (gal.0+(exp_rows*(factor-1)), gal.1+(exp_cols*(factor-1)))
    }

    fn calc_min_dist_big(&self, base: &(usize,usize), cmp: &(usize, usize),factor: usize) -> usize {
        let ref_exp = self.get_true_loc_big(base,factor);
        let cmp_exp = self.get_true_loc_big(cmp,factor);
        let x = if ref_exp.0 > cmp_exp.0 {
            ref_exp.0 - cmp_exp.0
        } else {
            cmp_exp.0 - ref_exp.0
        };
        let y = if ref_exp.1 > cmp_exp.1 {
            ref_exp.1 - cmp_exp.1
        } else {
            cmp_exp.1 - ref_exp.1
        };

        x + y
    }
}