use std::{cmp::max, collections::{HashSet, HashMap}, ops::RangeInclusive};

type Num = u32;

fn main() {
  let input = include_str!("in.txt");
  let output = solve(input);
  dbg!(output);
}

type R = RangeInclusive<Num>;

#[derive(Debug, Clone)]
struct Brick { x: R, y: R, z: R }

impl Brick {
  fn height(self: &Brick) -> Num {
    return self.z.end() - self.z.start() + 1 as Num;
  }
}

fn overlap<T: std::cmp::PartialOrd>(af: &RangeInclusive<T>, bg: &RangeInclusive<T>) -> bool
where T: std::cmp::PartialOrd {
  return !(af.end() < bg.start() || bg.end() < af.start());
}

fn xyoverlap(af: &Brick, bg: &Brick) -> bool {
  overlap(&af.x,&bg.x) && overlap(&af.y,&bg.y)
}


fn solve(input: &str) -> Num {
    // read bricks
  let mut bricks = input.lines().map(|line| {
    let (from, to) = line.split_once('~').unwrap();

    let xyz = from.split(',').map(|n| n.parse::<Num>().unwrap()).zip(
      to.split(',').map(|n| n.parse::<Num>().unwrap())
    ).map(|(a,b)| if a < b { a..=b } else { b..=a }).collect::<Vec<_>>();

    Brick { x: xyz[0].clone(), y: xyz[1].clone(), z: xyz[2].clone() }
  }).collect::<Vec<_>>();

  bricks.sort_by(|a,b| a.z.start().cmp(&b.z.start()));

    // let them fall
  let mut settled: Vec<(Brick,Num)> = Vec::new();
  let mut with_z_top = HashMap::<Num, Vec<usize>>::new();

  for brick in bricks {
    let mut newminz = 0;
    for (base, baseminz) in &mut settled {
      if xyoverlap(&brick,&base) {
        newminz = max(newminz, *baseminz + base.height());
      }
    }
    settled.push((brick.clone(),newminz));

    let ztop = newminz + brick.height();
    with_z_top.entry(ztop).or_insert(Vec::new()).push(settled.len()-1);
  }

    // find bricks that are only supported by a single other brick, mark the supporting as bad as it cannot be removed
  let mut bad = HashSet::new();
  for (brick, minz) in &settled {
    if let Some(brick_bellow) = with_z_top.get(&minz) {
      let mut firstbi = None;
      let mut count = 0;
      for &bi in brick_bellow {
        if xyoverlap(&brick,&settled[bi].0) {
          count += 1;
          if firstbi == None {
            firstbi = Some(bi)
          }
        }
      }
      if count == 1 {
        bad.insert(firstbi);
      }
    }
  }
  (settled.len() - bad.len()) as Num
}


#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn example1() {
    assert_eq!(5, solve("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"));
  }

  #[test]
  fn example2() {
    assert_eq!(false, overlap(&(1..=1),&(11..=11)));
    assert_eq!(false, xyoverlap(
      &Brick{x:1..=1, y:1..=1, z:1..=1},
      &Brick{x:11..=11, y:11..=11, z:2..=2}
    ));
    assert_eq!(6, solve("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
11,11,11~11,11,11"));
  }

  #[test]
  fn fullinput() {
    assert_eq!(463, solve(include_str!("in.txt")));
  }
}