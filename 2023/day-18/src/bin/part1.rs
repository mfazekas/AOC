type Num = u32;

fn main() {
  let input = include_str!("in_1.txt");
  let output = solve(input);
  dbg!(output);
}
 
fn fill(map: &Vec<Vec<u8>>) -> Num {
  let mut sum = 0;
  for r in 0..map.len() {
    let mut inside = false;
    for c in 0..map[0].len() {
      if map[r][c] == b'U' {
        inside = true;
        sum += 1;
      } else if map[r][c] == b'D' {
        inside = false;
        sum +=1
      } else if inside || map[r][c] != b'.' {
        sum +=1;
      }
    }
  }
  sum
}

fn solve(input: &str) -> Num {
  let digs = input.lines().map(|line| {
    let (dir, rest) = line.split_once(" ").unwrap();
    let (num, _rest) = rest.split_once(" ").unwrap();
    let num = num.parse::<Num>().unwrap();

    (dir.bytes().next().unwrap(), num)
  }
  ).collect::<Vec<_>>();

  let mr = 3000;
  let mc = 3000;
  let s = (1000,1000);
  let mut map = vec![vec![b'.'; mc]; mr];

  let mut r = s.0;
  let mut c = s.1;

  digs.iter().for_each(|(dir, num)| {
    let (dr, dc, s) = match dir {
      &b'R' => (0, 1, b'-'),
      &b'L' => (0, -1, b'-'),
      &b'U' => (-1, 0, b'|'),
      &b'D' => (1, 0, b'|'),
      _ => unreachable!(),
    };
    dbg!(dir, num);

    if s == b'|' {
      map[r][c] = *dir;
    }

    for _i in 0..*num {
      r = (r as i32 + dr as i32) as usize;
      c = (c as i32 + dc as i32) as usize;
      map[r][c] = *dir;
    }
  });

  fill(&map)
}


fn _dump(map: &Vec<Vec<u8>>) {
  for r in 0..map.len() {
    for c in 0..map[0].len() {
      print!("{}", map[r][c] as char);
    }
    println!();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn something1() {
    assert_eq!(16, solve("R 3 (#70c710)
D 3 (#0dc571)
L 3 (#5713f0)
U 3 (#d2c081)"));
  }

  #[test]
  fn something0() {
    assert_eq!(62, solve("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"));
  }
}