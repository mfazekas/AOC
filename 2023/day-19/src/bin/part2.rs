use core::panic;
use std::{collections::HashMap, ops::Range};

type Num = i64;

fn main() {
  let input = include_str!("in_1.txt");
  let output = solve(input);
  dbg!(output);
}

type Dim = u8;

#[derive(Debug)]
enum Condition {
  Less(Dim, Num),
  Greater(Dim, Num),
  Always
}

use Condition::*;

type Workflows = HashMap<String,Vec<(Condition,String)>>;

type Parts = HashMap<Dim,Range<Num>>;

fn sum_accepted(act: &str, si: usize, parts: &Parts, workflows: &Workflows) -> Num {
  let mut parts = parts.clone();
  let mut si = si;
  let mut act = act;

  let mut badsum = 0;
  loop {
    let workflow = workflows.get(act).unwrap();
    for i in si..workflow.len() {
      let (condition, action) = &workflow[i];

      let (ok, bad) = match *condition {
        Less(dim, right) => {
          let left = &parts[&dim];
          if left.end <= right {
            (Some(parts), None)
          } else if left.start < right {
            let mut leftok = parts.clone();
            let mut leftbad = parts.clone();
            leftok.extend([(dim, left.start .. right)]);
            leftbad.extend([(dim, right .. left.end)]);
            (Some(leftok), Some(leftbad))
          } else {
            (None, Some(parts))
          }
        }
        Greater(dim, right) => {
          let left = &parts[&dim];

          if left.start > right {
            (Some(parts), None)
          } else if left.end > right {
            let mut leftok = parts.clone();
            let mut leftbad = parts.clone();
            leftok.extend([(dim, right+1 .. left.end)]);
            leftbad.extend([(dim, left.start .. right+1)]);
           (Some(leftok), Some(leftbad))
          } else {
            (None, Some(parts))
          }
        }
        Always => {
          (Some(parts), None)
        }
      };

      if let (Some(_ok), Some(bad)) = (&ok,&bad) {
        let badparts = bad.clone();
        badsum += sum_accepted(act, i+1, &badparts, workflows);
      }

      if let Some(ok) = ok {
        parts = ok;
        match action.as_str() {
          "R" => {
            return badsum;
          },
          "A" => {
            
            return badsum + parts.values().map(|r| r.end - r.start).product::<Num>();
          },
          _ => {
            act = &action;
            si = 0;
            break;
          }
        }
      } else {
        parts = bad.unwrap();
      }
    }
  }
}

fn solve(input: &str) -> Num {
  let mut workflow_part = true;

  let mut workflows = HashMap::new();


  for line in input.lines() {
    if line == "" {
      workflow_part = false;
    } else if workflow_part {
      let (name, rest) = line.split_once("{").unwrap();
      let (conditions, _rest) = rest.split_once("}").unwrap();
      let conditions = conditions.split(",").map(|c| {
        if let Some((condition, action)) = c.split_once(":") {
          if let Some((field, value)) = condition.split_once("<") {
            (Less(field.bytes().next().unwrap(), value.parse().unwrap()),action.to_string())
          } else if let Some((field, value)) = condition.split_once(">") {
            (Greater(field.bytes().next().unwrap(), value.parse().unwrap()),action.to_string()) 
          } else {
            panic!("Invalid condition: {}", condition);
          }
        } else {
          (Always, c.to_string())
        }
      }).collect::<Vec<_>>();
      workflows.insert(name.to_string(), conditions);
    }
  }

  let parts = "xmas".bytes().map(|b| (b, 1..4001)).collect::<HashMap<_,_>>();

  sum_accepted("in", 0, &parts, &workflows)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(167409079868000, solve("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"));
  }
  

  #[test]
  fn small_split() {
    assert_eq!(1, solve("in{a>1:R,s>1:R,m>1:R,x>1:R,A}

"));

    assert_eq!(3999, solve("in{a>1:R,s>3999:R,m>1:R,x>1:R,A}

    "));
  }

}