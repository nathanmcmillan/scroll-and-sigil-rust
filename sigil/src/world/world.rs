use crate::map::line::Line;
use crate::map::sector;
use crate::map::sector::Sector;
use crate::map::triangulate::triangulate_sector;
use crate::things::thing::Thing;
use crate::things::thing::Updatable;
use std::collections::HashSet;

pub const WORLD_SCALE: f32 = 0.25;
pub const WORLD_CELL_SHIFT: i32 = 5;

#[derive(Default)]
pub struct WorldCell {
    pub lines: Vec<usize>,
    pub things: Vec<usize>,
}

impl WorldCell {
    pub fn add_thing(&mut self, thing: usize) {
        self.things.push(thing);
    }
    pub fn remove_thing(&mut self, thing: usize) {
        let len = self.things.len();
        for i in 0..len {
            if self.things[i] == thing {
                self.things.remove(i);
                return;
            }
        }
    }
}

pub struct World {
    pub things: Vec<Thing>,
    pub sectors: Vec<Sector>,
    pub cells: Vec<WorldCell>,
    pub cell_columns: usize,
    pub cell_rows: usize,
}

fn build_cell_lines(cells: &mut Vec<WorldCell>, cell_columns: usize, line: &Line) {
    let dx = (line.b.x - line.a.x).abs();
    let dy = (line.b.y - line.a.y).abs();

    let mut x = line.a.x.floor() as i32;
    let mut y = line.a.y.floor() as i32;

    let mut n = 1;
    let mut error;
    let x_inc;
    let y_inc;

    if dx == 0.0 {
        x_inc = 0;
        error = std::f32::MAX;
    } else if line.b.x > line.a.x {
        x_inc = 1;
        n += (line.b.x).floor() as i32 - x;
        error = (line.a.x.floor() + 1.0 - line.a.x) * dy;
    } else {
        x_inc = -1;
        n += x - line.b.x.floor() as i32;
        error = (line.a.x - line.a.x.floor()) * dy;
    }

    if dy == 0.0 {
        y_inc = 0;
        error = std::f32::MIN;
    } else if line.b.y > line.a.y {
        y_inc = 1;
        n += (line.b.y).floor() as i32 - y;
        error -= (line.a.y.floor() + 1.0 - line.a.y) * dx;
    } else {
        y_inc = -1;
        n += y - line.b.y.floor() as i32;
        error -= (line.a.y - line.a.y.floor()) * dx;
    }

    while n > 0 {
        let cell = &mut cells[(x as usize >> WORLD_CELL_SHIFT) + (y as usize >> WORLD_CELL_SHIFT) * cell_columns];
        cell.lines.push(line.index);

        if error > 0.0 {
            y += y_inc;
            error -= dx;
        } else {
            x += x_inc;
            error += dy;
        }

        n -= 1;
    }
}

impl World {
    pub fn new() -> Self {
        World {
            things: Vec::new(),
            sectors: Vec::new(),
            cells: Vec::new(),
            cell_columns: 0,
            cell_rows: 0,
        }
    }
    pub fn add_sector(&mut self, mut sector: Sector) {
        sector.index = self.sectors.len();
        self.sectors.push(sector);
    }
    pub fn add_thing(&mut self, thing: Thing) {
        self.things.push(thing);
    }
    pub fn find_sector(&self, x: f32, y: f32) -> Option<&Sector> {
        for sector in self.sectors.iter() {
            if sector.outside.is_some() {
                continue;
            }
            return Some(sector::find(&self.sectors, &sector, x, y));
        }
        Option::None
    }
    pub fn get_sector(&self, index: usize) -> &Sector {
        &self.sectors[index]
    }
    fn build_lines(&mut self, index: usize) {
        let sector = &mut self.sectors[index];
        let lines = sector.lines.len();
        if lines == 0 {
            return;
        }

        let bottom = sector.bottom;
        let floor = sector.floor;
        let ceil = sector.ceiling;
        let top = sector.top;

        let plus;
        let minus;

        if sector.outside.is_none() {
            plus = Option::None;
            minus = Some(sector.index);
        } else {
            plus = Some(sector.index);
            minus = sector.outside;
        }

        let mut u = 0.0;
        for i in 0..lines {
            let line = &mut sector.lines[i];
            build_cell_lines(&mut self.cells, self.cell_columns, line);
            line.update_sectors(plus, minus);
            let x = line.a.x - line.b.x;
            let y = line.a.y - line.b.y;
            let s = u + (x * x + y * y).sqrt() * WORLD_SCALE;
            if let Some(wall) = &mut line.bottom {
                wall.update(bottom, floor, u, bottom * WORLD_SCALE, s, floor * WORLD_SCALE);
            }
            if let Some(wall) = &mut line.middle {
                wall.update(floor, ceil, u, floor * WORLD_SCALE, s, ceil * WORLD_SCALE);
            }
            if let Some(wall) = &mut line.top {
                wall.update(ceil, top, u, ceil * WORLD_SCALE, s, top * WORLD_SCALE);
            }
            u = s;
        }
    }

    pub fn build(&mut self) {
        let mut top = 0.0;
        let mut right = 0.0;
        for sector in self.sectors.iter() {
            for vec in sector.vecs.iter().copied() {
                if vec.y > top {
                    top = vec.y;
                }
                if vec.x > right {
                    right = vec.x;
                }
            }
        }
        let len = self.sectors.len();
        for i in 0..len {
            let mut list = Vec::new();
            let sector = &self.sectors[i];
            for k in 0..len {
                if i == k {
                    continue;
                }
                let other = &self.sectors[k];
                let mut contains = true;
                for vec_o in other.vecs.iter().copied() {
                    for vec_s in sector.vecs.iter() {
                        if vec_s.eq(vec_o) {
                            contains = false;
                            break;
                        }
                    }
                    if !contains {
                        break;
                    }
                    if !sector.contains(vec_o.x, vec_o.y) {
                        contains = false;
                        break;
                    }
                }
                if contains {
                    list.push(k);
                }
            }
            let sector = &mut self.sectors[i];
            println!("inside list {:?}", list);
            sector.inside.append(&mut list);
        }
        for i in 0..len {
            let mut dead = HashSet::new();
            {
                let inside_len = self.sectors[i].inside.len();
                for k in 0..inside_len {
                    let inside = self.sectors[i].inside[k];
                    let other_len = self.sectors[inside].inside.len();
                    for o in 0..other_len {
                        let other = self.sectors[inside].inside[o];
                        dead.insert(other);
                    }
                }
            }
            {
                let sector = &mut self.sectors[i];
                for d in dead.iter().copied() {
                    sector.inside.retain(|f| *f != d);
                }
            }
            let inside_len = self.sectors[i].inside.len();
            for k in 0..inside_len {
                let inside = self.sectors[i].inside[k];
                let inner = &mut self.sectors[inside];
                inner.outside = Some(i);
            }
        }
        let cell_size = (1 << WORLD_CELL_SHIFT) as f32;
        self.cell_rows = (top / cell_size).ceil() as usize;
        self.cell_columns = (right / cell_size).ceil() as usize;
        self.cells.resize_with(self.cell_columns * self.cell_rows, Default::default);
        for s in 0..self.sectors.len() {
            triangulate_sector(&mut self.sectors, s, WORLD_SCALE);
        }
        for i in 0..self.sectors.len() {
            self.build_lines(i);
        }
    }

    pub fn update(&mut self) {
        for thing in self.things.iter_mut() {
            thing.update();
        }
    }
}
