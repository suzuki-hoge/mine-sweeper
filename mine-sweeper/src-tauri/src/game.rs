use crate::game::Dot::{FlaggedMine, FlaggedSafe, Mine, Number, Unexplored, Wall};
use crate::game::Swept::{Bomb, Safe, Stay};

#[derive(Debug)]
pub struct Game {
    w: usize,
    h: usize,
    dots: Vec<Vec<Dot>>,
}

impl Game {
    // 盤面を初期化する
    pub fn init(w: usize, h: usize) -> Self {
        let dots = vec![
            vec![Wall, Wall, Wall, Wall, Wall],
            vec![Wall, Unexplored, Unexplored, Unexplored, Wall],
            vec![Wall, Unexplored, Unexplored, Unexplored, Wall],
            vec![Wall, Mine, Unexplored, FlaggedMine, Wall],
            vec![Wall, Wall, Wall, Wall, Wall],
        ];
        Self { w, h, dots }
    }

    // 選択した箇所を更新し、選択結果を返す
    pub fn sweep(&mut self, x: usize, y: usize) -> Swept {
        match &self.dots[y + 1][x + 1] {
            &Mine | &FlaggedMine => Bomb,
            &Number(_) | &Wall => Stay,
            &Unexplored | &FlaggedSafe => {
                let mut mines = 0;
                for yd in y..=y + 2 {
                    for xd in x..=x + 2 {
                        let dot = &self.dots[y + yd][x + xd];
                        if dot == &Mine || dot == &FlaggedMine {
                            mines += 1;
                        }
                    }
                }
                self.dots[y + 1][x + 1] = Number(mines);
                Safe
            }
        }
    }

    // 番兵を除き地雷を隠す
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
    Safe,
    Bomb,
    Stay,
}

#[cfg(test)]
mod tests {
    use crate::game::Dot::{Number, Unexplored};
    use crate::game::Game;
    use crate::game::Swept::{Bomb, Safe, Stay};

    #[test]
    fn sweep() {
        let mut game = Game::init(3, 3);
        assert_eq!(&Unexplored, &game.dots[0 + 1][0 + 1]);

        // 未探索が 0 に更新されている
        let swept = game.sweep(0, 0);
        assert_eq!(&Number(0), &game.dots[0 + 1][0 + 1]);
        assert_eq!(Safe, swept);

        // 探索済みならそのままである
        let swept = game.sweep(0, 0);
        assert_eq!(&Number(0), &game.dots[0 + 1][0 + 1]);
        assert_eq!(Stay, swept);

        // 地雷を踏んだら終了
        let swept = game.sweep(0, 2);
        assert_eq!(Bomb, swept);
    }
}
