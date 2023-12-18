type Num = i64;
use core::ops::Bound::Included;
use std::collections::{BTreeSet, BTreeMap};

fn main() {
  let input = include_str!("in_1.txt");
  let output = solve(input);
  dbg!(output);
}

fn area(vlines: &VLines) -> Num {
  let mut vlines_by_h = BTreeMap::<Num, (Num, BTreeSet::<Num>)>::new();
  for (f,t,c) in vlines {
    vlines_by_h.entry(*f).or_insert_with(||(*t,BTreeSet::new())).1.insert(
      *c
    );
  }  
  vlines_by_h.iter().map(|(f,(t,s))| -> Num
    {
      let mut pc = *s.first().unwrap();

      let cs = s.iter().enumerate().map(|(i,c)| -> Num {
        if i % 2 == 0 {
          pc = *c;
          0
        } else {
          *c-pc
        }
      }
      ).sum::<Num>();
      (t-f) * cs
    }
  ).sum()
}

type VLines = Vec<(Num, Num, Num)>;

fn split(lines: &VLines, pois: &BTreeSet<Num>) -> VLines {
  let mut result = vec![];
  lines.iter().for_each(|(f,t,c)|
    {
      let mut pf = *f;
      for &at in pois.range((Included(f), Included(t))) {
        if pf != at {
          result.push((pf, at, *c));
        }
        pf = at
      }
      if pf != *t {
        result.push((pf, *t, *c)); 
      }
    }
  );
  result  
}

fn walk(digs: &Vec<(&str, Num)>) -> (VLines, Num) {
  let mut r = 0;
  let mut c = 0;

  let mut vlines = vec![];

  let mut len: Num = 0;

  digs.iter().for_each(|(dir, num)| {
    let (dr,dc) = match *dir {
      "R" => (0, 1),
      "L" => (0, -1),
      "U" => (-1, 0),
      "D" => (1, 0),
      _ => unreachable!(),
    };

    if *dir == "D" {
      vlines.push((r,r + dr * num, c));
    } else if *dir == "U" {
      vlines.push((r+dr * num,r,c));
    }

    len += num;
    (r, c) = (r + dr * num, c + dc * num);
  });
  return (vlines, len);
}

fn dig_area(digs: &Vec<(&str, Num)>) -> Num {
  let (vlines, len) = walk(digs);

  let pois = vlines.iter().flat_map(|(f,t,_c)| vec![*f,*t]).collect::<BTreeSet<_>>();

  let mut vlines = split(&vlines, &pois);
  
  vlines.sort();

  area(&vlines) + len/2 + 1 /* 1 = (#cw-#ccw)/4 */
}

fn solve(input: &str) -> Num {
  let digs = input.lines().map(|line| {
    let (_old_dir, rest) = line.split_once(" ").unwrap();
    let (_old_num, rest) = rest.split_once(" ").unwrap();
    
    let num = Num::from_str_radix(&rest[2..rest.len()-2], 16).unwrap() as i64;
    let _old_num = _old_num.parse::<Num>().unwrap();
    let dir = match &rest.as_bytes()[rest.len()-2] {
      b'0'=> "R",
      b'2' => "L",
      b'1' => "D",
      b'3' => "U",
      _ => unreachable!(),
    };
    (dir, num)
  }
  ).collect::<Vec<_>>();

  dig_area(&digs)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn square_9x9() {
    assert_eq!(100, solve("R 3 (#000090)
D 3 (#000091)
L 3 (#000092)
U 3 (#000093)"));
  }


  #[test]
  fn example() {
    assert_eq!(952408144115, solve("R 6 (#70c710)
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