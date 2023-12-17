use std::{collections::BinaryHeap, cmp::Ordering};

type Num = u32;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
  Right = 0,
  Down = 1,
  Left = 2,
  Up = 3
}

type Pos = (usize, usize);

use Dir::*;

impl Dir {
  fn turn(self: &Dir) -> Vec<Dir> {
    match self {
      Right => vec![Up, Down],
      Down => vec![Right, Left],
      Left => vec![Down, Up],
      Up => vec![Left, Right],
    }
  }

  fn _step(self: &Dir, pos: Pos) -> Option<Pos> {
    let (r, c) = pos;
    match self {
      Right => Some((r, c + 1)),
      Down => Some((r + 1, c)),
      Left => if c > 0 { Some((r, c - 1)) } else { None},
      Up => if r > 0 { Some((r - 1, c)) } else { None }
    }
  }

  fn step(self: &Dir, pos: Pos, bounds: (usize, usize)) -> Option<Pos> {
    if let Some((r, c)) = self._step(pos) {
      if r < bounds.0 && c < bounds.1 {
        return Some((r, c));
      }
    }
    None
  }

  fn axis(self: &Dir) -> usize {
    ((*self as u8) % 2) as usize
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
  loss: Num,
  dir: Dir,
  pos: Pos,
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
      // Notice that the we flip the ordering on costs.
      // In case of a tie we compare positions - this step is necessary
      // to make implementations of `PartialEq` and `Ord` consistent.
      other.loss.cmp(&self.loss)
          .then_with(|| self.pos.cmp(&other.pos))
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

fn main() {
  let input = include_str!("in_1.txt");
  let output = solve(input);
  dbg!(output);
}

fn shortest(map: &Vec<Vec<u8>>, start: Pos, goal: Pos) -> Num {
  let maxr = map.len();
  let maxc = map[0].len();
  let ndir = 2;

  let mut best = vec![vec![vec![Num::MAX; ndir] ;maxc]; maxr];

  let mut heap = BinaryHeap::new();

  heap.push(State{loss: 0, dir: Dir::Up, pos: start});

  while let Some(State{loss, dir, pos}) = heap.pop() {
    if pos == goal {
      return loss;
    }

    if loss > best[pos.0 as usize][pos.1 as usize][dir.axis()] {
      continue;
    }
    best[pos.0 as usize][pos.1 as usize][dir.axis()] = loss;

    for dir in dir.turn() {
      let mut actloss = loss;
      let (mut nr, mut nc) = pos;
      for i in 1..11 {
        if let Some((r, c)) = dir.step((nr, nc), (maxr, maxc)) {
          (nr, nc) = (r, c);
          let heat = (map[nr as usize][nc as usize]- b'0') as Num;
          actloss += heat;

          if i >= 4 {
            let vbest =&mut best[nr as usize][nc as usize][(dir as usize) % 2];
            if actloss < *vbest {
              heap.push(State{loss: actloss, dir: dir, pos: (nr, nc)});

              *vbest = actloss; 
            }
          }
        }
      }
    }
  }
  panic!("No path found");
}

fn solve(input: &str) -> Num {
  let map = input.lines().map(|line| 
    line.bytes().collect::<Vec<_>>()
  ).collect::<Vec<_>>();

  shortest(&map, (0,0), (map.len() - 1, map[0].len() - 1))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn something() {
    assert_eq!(94, solve("2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"));
  }


  #[test]
  fn something2() {
    assert_eq!(71, solve("111111111111
999999999991
999999999991
999999999991
999999999991"));
  }
}

fn _dump(best: &Vec<Vec<Vec<Num>>>) {
  for row in best {
    for p in row {
      print!("{}", p.iter().filter(|x| **x != Num::MAX ).count());
    }
    println!();
  }
  println!("");
  println!("++++++++")
}

fn _sum(best: &Vec<Vec<Vec<Num>>>) -> Num {
  let mut sum = 0;
  for row in best {
    for p in row {
      sum += p.iter().filter(|x| **x != Num::MAX ).count()
    }
  }
  sum as Num
}