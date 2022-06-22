use crate::pos::{Direction, Pos};
use oorandom::Rand32;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq)]
enum MapCell {
    EMPTY,
    WALL,
    STONE,
    BORDER,
    EXIT,
}

struct Player {
    pos: Pos,
}

struct Diamond {
    pos: Pos,
}

pub struct Map {
    terrain: HashMap<(u32, u32), MapCell>,
    player: Player,
    diamonds: Vec<Diamond>,
}

impl Map {
    pub fn generate(seed: u32) -> Self {
        let mut rng = Rand32::new(seed.into());
        let mut terrain = HashMap::new();

        for x in 1..36 {
            for y in 1..36 {
                terrain.insert((x, y), MapCell::WALL);
            }
        }

        for x in 0..36 {
            terrain.insert((x, 0), MapCell::BORDER);
        }
        for y in 0..36 {
            terrain.insert((36, y), MapCell::BORDER);
        }
        for x in (1..37).rev() {
            terrain.insert((x, 36), MapCell::BORDER);
        }
        for y in (1..37).rev() {
            terrain.insert((0, y), MapCell::BORDER);
        }

        for _ in 0..rng.rand_range(10..40) {
            terrain.insert(
                (rng.rand_range(2..35), rng.rand_range(2..35)),
                MapCell::STONE,
            );
        }

        let room_count = rng.rand_range(4..7);
        let mut rooms_center = Vec::with_capacity(7);
        for _ in 0..room_count {
            let top = rng.rand_range(2..25);
            let left = rng.rand_range(2..25);

            rooms_center.push((left + 5, top + 5));

            for x in 1..10 {
                for y in 1..10 {
                    terrain.insert((left + x, top + y), MapCell::EMPTY);
                }
            }

            if rng.rand_float() < 0.5 {
                for y in 2..9 {
                    terrain.insert((left, top + y), MapCell::EMPTY);
                    terrain.insert((left + 10, top + y), MapCell::EMPTY);
                }
            }

            if rng.rand_float() < 0.5 {
                for x in 2..9 {
                    terrain.insert((left + x, top), MapCell::EMPTY);
                    terrain.insert((left + x, top + 10), MapCell::EMPTY);
                }
            }
        }

        for c in &rooms_center {
            terrain.insert(*c, MapCell::BORDER);
        }

        for i in 0..room_count {
            for j in i + 1..room_count {
                let i = i as usize;
                let j = j as usize;
                if rng.rand_float() < 0.5 {
                    if rooms_center[i].0 < rooms_center[j].0 {
                        for x in rooms_center[i].0..rooms_center[j].0 + 1 {
                            terrain.insert((x, rooms_center[i].1), MapCell::EMPTY);
                        }
                    } else {
                        for x in rooms_center[j].0..rooms_center[i].0 + 1 {
                            terrain.insert((x, rooms_center[i].1), MapCell::EMPTY);
                        }
                    }
                    if rooms_center[i].1 < rooms_center[j].1 {
                        for y in rooms_center[i].1..rooms_center[j].1 + 1 {
                            terrain.insert((rooms_center[j].0, y), MapCell::EMPTY);
                        }
                    } else {
                        for y in rooms_center[j].1..rooms_center[i].1 + 1 {
                            terrain.insert((rooms_center[j].0, y), MapCell::EMPTY);
                        }
                    }
                }
            }
        }

        let mut diamonds = Vec::new();

        for _ in 0..6 {
            let d = Diamond::new(rng.rand_range(2..35), rng.rand_range(2..35));

            if terrain[&d.position().into()] != MapCell::EMPTY {
                if terrain[&d.position().moved(Direction::UP).into()] != MapCell::EMPTY
                    && terrain[&d.position().moved(Direction::LEFT).into()] != MapCell::EMPTY
                    && terrain[&d.position().moved(Direction::DOWN).into()] != MapCell::EMPTY
                    && terrain[&d.position().moved(Direction::RIGHT).into()] != MapCell::EMPTY
                {
                    for x in 0..3 {
                        for y in 0..3 {
                            terrain.insert(
                                (d.position().x + x - 1, d.position().y + y - 1),
                                MapCell::EMPTY,
                            );
                        }
                    }
                } else {
                    terrain.insert((d.position().x, d.position().y), MapCell::EMPTY);
                }
            }

            diamonds.push(d);
        }

        if terrain[&(18, 18)] != MapCell::EMPTY {
            let nearest = rooms_center
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    (
                        index,
                        (((item.0 as i32 - 18).pow(2) + (item.1 as i32 - 18).pow(2)) as f32).sqrt(),
                    )
                })
                .min_by(|(_, dist1), (_, dist2)| {
                    dist1.partial_cmp(dist2).unwrap_or(Ordering::Equal)
                })
                .unwrap()
                .0;
            if rooms_center[nearest].0 < 18 {
                for x in rooms_center[nearest].0..18 + 1 {
                    terrain.insert((x, rooms_center[nearest].1), MapCell::EMPTY);
                }
            } else {
                for x in 18..rooms_center[nearest].0 + 1 {
                    terrain.insert((x, rooms_center[nearest].1), MapCell::EMPTY);
                }
            }
            if rooms_center[nearest].1 < 18 {
                for y in rooms_center[nearest].1..18 + 1 {
                    terrain.insert((18, y), MapCell::EMPTY);
                }
            } else {
                for y in 18..rooms_center[nearest].1 + 1 {
                    terrain.insert((18, y), MapCell::EMPTY);
                }
            }
        }

        for x in 17..20 {
            for y in 17..20 {
                terrain.insert((x, y), MapCell::EMPTY);
            }
        }

        terrain.insert((18, 18), MapCell::EXIT);

        let mut player_pos = (0, 0);

        while terrain[&player_pos] != MapCell::EMPTY {
            player_pos = (rng.rand_range(4..33), rng.rand_range(4..33));
        }

        Self {
            terrain,
            player: Player::new(player_pos.0, player_pos.1),
            diamonds,
        }
    }

    pub fn empty() -> Self {
        Self {
            terrain: HashMap::new(),
            player: Player::new(0, 0),
            diamonds: Vec::new(),
        }
    }

    pub fn repr(&self) -> String {
        let mut string = String::with_capacity(1369);

        for y in 0..37 {
            'lbl: for x in 0..37 {
                if self.player.position() == (x, y) {
                    string.push('J');
                    continue;
                }
                for d in &self.diamonds {
                    if d.position() == (x, y) {
                        string.push('D');
                        continue 'lbl;
                    }
                }
                string.push(match self.terrain.get(&(x, y)).unwrap() {
                    MapCell::EMPTY => ' ',
                    MapCell::WALL => 'M',
                    MapCell::STONE => 'P',
                    MapCell::BORDER => 'B',
                    MapCell::EXIT => 'S',
                });
            }
        }

        string
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.player.move_(direction);
    }
}

impl Player {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Pos::new(x, y),
        }
    }

    pub fn move_(&mut self, direction: Direction) {
        self.pos = self.pos.moved(direction);
    }
}

impl Diamond {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Pos::new(x, y),
        }
    }
}

trait Entity {
    fn position(&self) -> Pos;
}

impl Entity for Player {
    fn position(&self) -> Pos {
        self.pos
    }
}

impl Entity for Diamond {
    fn position(&self) -> Pos {
        self.pos
    }
}
