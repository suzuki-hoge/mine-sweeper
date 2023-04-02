use itertools::Itertools;

use crate::game::Dot::{FlaggedMine, FlaggedSafe, Mine, Number, Unexplored, Wall};
use crate::game::Swept::{Bomb, Clear, Safe, Stay};

#[derive(Debug)]
pub struct Game {
    w: usize,
    h: usize,
    dots: Vec<Vec<Dot>>,
    unexplored_remains: usize,
}

impl Game {
    // 盤面を初期化する
    pub fn init(w: usize, h: usize) -> Self {
        let dots = vec![
            vec![Unexplored, Unexplored, Unexplored],
            vec![Unexplored, Unexplored, Unexplored],
            vec![Mine, Unexplored, FlaggedMine],
        ];
        Self {
            w,
            h,
            dots,
            unexplored_remains: 7,
        }
    }

    // 選択した箇所を更新し、選択結果を返す
    pub fn sweep(&mut self, x: usize, y: usize) -> Swept {
        match &self.dots[y][x] {
            &Mine | &FlaggedMine => Bomb,
            &Number(_) | &Wall => Stay,
            &Unexplored | &FlaggedSafe => {
                let mut mines = 0;

                self.rounds(x, y).iter().for_each(|&(x, y)| {
                    let dot = &self.dots[y][x];
                    if dot == &Mine || dot == &FlaggedMine {
                        mines += 1;
                    }
                });

                if &self.dots[y][x] == &Unexplored {
                    self.unexplored_remains -= 1;
                }

                self.dots[y][x] = Number(mines);

                // 0 の場合は連鎖させる
                if mines == 0 {
                    self.rounds(x, y).iter().for_each(|&(x, y)| {
                        if &self.dots[y][x] == &Unexplored {
                            self.sweep(x, y);
                        }
                    });
                }
                if self.unexplored_remains == 0 {
                    Clear
                } else {
                    Safe
                }
            }
        }
    }

    fn rounds(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (x, y, w, h) = (x as i64, y as i64, self.w as i64, self.h as i64);
        (-1..=1_i64)
            .cartesian_product(-1..=1_i64)
            .filter(|&p| p != (0, 0))
            .filter(|&(xd, yd)| 0 <= x + xd && x + xd < w && 0 <= y + yd && y + yd < h)
            .map(|(xd, yd)| ((x + xd) as usize, (y + yd) as usize))
            .collect_vec()
    }

    // Tauri が値を持つ enum を扱えないので、文字列にする
    pub fn show(&self) -> Vec<Vec<String>> {
        let mut dots = vec![vec![String::new(); self.w]; self.h];
        for y in 1..self.h + 1 {
            for x in 1..self.w + 1 {
                dots[y - 1][x - 1] = match &self.dots[y][x] {
                    &Unexplored | &Mine | &Wall => String::from(""),
                    &FlaggedSafe | &FlaggedMine => String::from("flag"),
                    &Number(n) => n.to_string(),
                };
            }
        }
        dots
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Dot {
    Wall,
    Unexplored,
    FlaggedSafe,
    FlaggedMine,
    Mine,
    Number(u64),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Swept {
    Clear,
    Safe,
    Bomb,
    Stay,
}

#[cfg(test)]
mod tests {
    use crate::game::Dot::{FlaggedMine, Mine, Number, Unexplored};
    use crate::game::Game;
    use crate::game::Swept::{Bomb, Clear, Safe, Stay};

    #[test]
    fn rounds_center() {
        let mut game = Game::init(3, 3);

        // 周囲 8 マスが得られる
        let mut exp = vec![];
        exp.extend(vec![(0, 0), (0, 1), (0, 2)]);
        exp.extend(vec![(1, 0), (1, 2)]);
        exp.extend(vec![(2, 0), (2, 1), (2, 2)]);
        assert_eq!(exp, game.rounds(1, 1));
    }

    #[test]
    fn rounds_left_top() {
        let mut game = Game::init(3, 3);

        // 左端と上端は返されない
        assert_eq!(vec![(0, 1), (1, 0), (1, 1),], game.rounds(0, 0));
    }

    #[test]
    fn rounds_right_bottom() {
        let mut game = Game::init(3, 3);

        // 右端と下端は返されない
        assert_eq!(vec![(1, 1), (1, 2), (2, 1),], game.rounds(2, 2))
    }

    #[test]
    fn sweep() {
        let mut game = Game::init(3, 3);
        assert_eq!(&Unexplored, &game.dots[0][0]);

        // 未探索が 0 に更新されている
        let swept = game.sweep(0, 0);
        assert_eq!(Safe, swept);
        assert_eq!(&Number(0), &game.dots[0][0]);

        // 0 のマスの周囲は連鎖解放されている
        assert_eq!(&Number(0), &game.dots[0][1]);
        assert_eq!(&Number(0), &game.dots[0][2]);
        assert_eq!(&Number(1), &game.dots[1][0]);
        assert_eq!(&Number(2), &game.dots[1][1]);
        assert_eq!(&Number(1), &game.dots[1][2]);

        // 地雷と範囲外は据え置きである
        assert_eq!(&Mine, &game.dots[2][0]);
        assert_eq!(&Unexplored, &game.dots[2][1]);
        assert_eq!(&FlaggedMine, &game.dots[2][2]);

        // 探索済みをクリックしても何も起きない
        let swept = game.sweep(0, 0);
        assert_eq!(&Number(0), &game.dots[0][0]);
        assert_eq!(Stay, swept);

        // 地雷を踏んだら終了
        let swept = game.sweep(0, 2);
        assert_eq!(Bomb, swept);

        // 地雷以外の全てを解放したら終了
        let swept = game.sweep(1, 2);
        assert_eq!(Clear, swept);
        assert_eq!(&Number(2), &game.dots[2][1]);
    }
}
