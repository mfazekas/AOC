use std::iter::once;

fn main() {
  let input = include_str!("in_1.txt");
  let output = solve(input);
  dbg!(output);
}

#[derive(Debug, Clone, Copy)]
enum Dir {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
}

use Dir::*;

fn visit(d: Dir, r: usize, c: usize, map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<Vec<bool>>>) {
    let v = &mut visited[r][c][d as usize];
  if *v || map[r][c] == b'*' {
    return;
  }
  *v = true;

  let dirs = match map[r][c] {
    b'.' => vec![d],
    b'/' => vec![match d {
        Up => Right,
        Right => Up,
        Down => Left,
        Left => Down,
      }],
    b'\\' => vec![match d {
        Up => Left,
        Right => Down,
        Down => Right,
        Left => Up,
      }],
    b'-' => match d {
        Up | Down => vec![Left, Right],
        _ => vec![d],
      },
    b'|' => match d {
        Left | Right => vec![Up, Down],
        _ => vec![d],
      },
    _ => {
      panic!("Unknown char: {} {}", map[r][c], map[r][c] as char);
    }
  };

  for d in dirs {
    let mut rn = r;
    let mut cn = c;

    match d {
      Up => rn -= 1,
      Down => rn += 1,
      Right => cn += 1,
      Left => cn -= 1,
    }

    visit(d, rn, cn, map, visited);
  }
}

fn count(visited: &Vec<Vec<Vec<bool>>>) -> usize {
  visited.iter().map(|row| 
      row.iter().filter(|c| 
        c.iter().any(|&v| v)
      ).count() 
  ).sum()
}

fn _dump(visited: &Vec<Vec<Vec<bool>>>) {
  for row in visited {
    for c in row {
      if c.iter().filter(|&&v| v).count() > 0 {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }
}

fn count_for(d: Dir, r: usize, c: usize, map: &Vec<Vec<u8>>, has_count: &mut Vec<Vec<Vec<bool>>>) -> usize {
  if has_count[r][c][d as usize] {
    return 0;
  }

  let w = map[1].len();
  let mut visited = vec![vec![vec![false; 4]; w]; map.len()];

  visit(d, r, c, &map, &mut visited);

  for r in 0..map.len() {
    for c in 0..map[0].len() {
      for d in 0..4 {
        has_count[r][c][d] = has_count[r][c][d] || visited[r][c][d];
      }
    }
  }

  count(&visited)
}

fn read_map_with_guard(input: &str, g: u8) -> Vec<Vec<u8>> {
  let mut map = once("").chain(input.lines().chain(once(""))).map(|line| 
    once(g).chain(line.bytes().chain(once(g))).collect::<Vec<_>>()
  ).collect::<Vec<_>>();

  let w = map[1].len();

  *map.first_mut().unwrap() = vec![g; w ];
  *map.last_mut().unwrap() = vec![g; w ];
  return map
}

fn solve(input: &str) -> usize {
  let map = read_map_with_guard(input, b'*');

  let nc = map[0].len();
  let nr = map.len();

  let mut has_count = vec![vec![vec![false; 4]; nc]; nr];

  let mut starts = vec![];
  for ci in 1..nc-1 {
    starts.push((Down, 1, ci));
    starts.push((Up, nr-2, ci));
  }
  for ri in 1..nr-1 {
    starts.push((Right, ri, 1));
    starts.push((Left, ri, nc-2));
  }

  starts.iter().map (|&(d, r, c)|
    count_for(d, r, c, &map, &mut has_count)
  ).max().expect("No solution found")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn small() {
    assert_eq!(51, solve(".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."));
  }
}