use std::{cmp::max, collections::{HashSet, HashMap, BinaryHeap}, ops::RangeInclusive};

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
  let mut supporters = vec![vec![]; bricks.len()];

  for brick in bricks {
    let mut newminz = 0;
    for (base, baseminz) in &mut settled {
      if xyoverlap(&brick,&base) {
        newminz = max(newminz, *baseminz + base.height());
      }
    }
    settled.push((brick.clone(),newminz));

    let ztop = newminz + brick.height();
    let si = settled.len()-1;
    with_z_top.entry(ztop).or_insert(Vec::new()).push(si);
  }

    // calculate supporters
  for si in 0..settled.len() {
    let (brick, minz) = &settled[si];

    if let Some(bricks) = with_z_top.get(&minz) {
      supporters[si] = bricks.iter().filter(|&&bi| xyoverlap(&brick,&settled[bi].0)).collect::<Vec<_>>();
    }
  }

  let mut max_single_supporter_parent = vec![None; settled.len()];
  let mut was = vec![false; settled.len()];

  let mut single_supporters = HashSet::new();

    // for each brick find the largest brick that is the only supporter of it
  for si in (0..settled.len()).rev() {
    if !was[si] {
      was[si] = true;
      let mut headset = HashSet::new();
      let mut head = BinaryHeap::new();
      head.push(si);
      while let Some(i) = head.pop() {
        if i != si && head.is_empty() {
          max_single_supporter_parent[si] = Some(i);
          single_supporters.insert(i);
        } else {
          if supporters[i].len() == 0 {
            // arrived on ground
            break;
          } else {
            for &bi in &supporters[i] {
              if !headset.contains(bi) {
                head.push(*bi);
                headset.insert(*bi);
              }
            }
          }
        }
      }
    }
  }

    // calculate the count of bricks falling if we remove the single supporter bricks
  let mut counts = vec![0; settled.len()];
  for si in (0..settled.len()).rev() {
    if let Some(i) = max_single_supporter_parent[si] {
      counts[i] += counts[si] + 1;
    }
  }

  single_supporters.iter().map(|si| counts[*si]).sum::<Num>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(7, solve("1,0,1~1,2,1
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
    assert_eq!(7, solve("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
11,11,11~11,11,11"));
  }

  #[test]
  fn example3() {
    assert_eq!(false, overlap(&(1..=1),&(11..=11)));
    assert_eq!(true, overlap(&(1..=2),&(2..=3)));
    assert_eq!(false, xyoverlap(
      &Brick{x:1..=1, y:1..=1, z:1..=1},
      &Brick{x:11..=11, y:11..=11, z:2..=2}
    ));
    assert_eq!(8, solve("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
11,11,11~11,11,11
11,11,12~11,11,12"));
  }

  #[test]
  fn example4() {
    assert_eq!(false, overlap(&(1..=1),&(11..=11)));
    assert_eq!(true, overlap(&(1..=2),&(2..=3)));
    assert_eq!(false, xyoverlap(
      &Brick{x:1..=1, y:1..=1, z:1..=1},
      &Brick{x:11..=11, y:11..=11, z:2..=2}
    ));
    assert_eq!(9+4+3+2, solve("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
1,1,10~5,5,10
1,1,11~1,1,11
2,2,11~2,2,11"));
  }

  #[test]
  fn example5() {
    assert_eq!(5, solve("0,0,0~10,10,0
0,0,1~3,3,1
4,4,1~7,7,1
0,0,2~1,1,2
2,2,2~4,4,2"));
  }

  #[test]
  fn example6() {
    assert_eq!(1, solve("0,0,0~1,1,0
10,10,0~10,10,0
0,0,1~10,10,1
2,2,2~4,4,2"));
  }

  #[test]
  fn example1_with_base() {
    assert_eq!(7+7, solve("1,0,0~1,2,0
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"));
  }

  #[test]
  fn fullinput() {
    assert_eq!(89727, solve(include_str!("in.txt"))); 
  }

}