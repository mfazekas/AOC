use core::panic;
use std::collections::HashMap;

type Num = u32;

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

type Part = HashMap<Dim,Num>;

fn run(part: &Part, workflows: &HashMap<String,Vec<(Condition,&str)>>)  -> &'static str {
  let mut act = "in";

  loop {
    let workflow = workflows.get(act).unwrap();
    for (condition, action) in workflow {
      let ok = 
        match condition {
          Less(field, right) => part[field] < *right,
          Greater(field, right) => part[field] > *right,
          Always => true,
        };

      if ok {
        match *action {
          "R" => return "R",
          "A" => return "A",
          _ => {
            act = action;
            break;
          }
        }
      }
    }
  }
} 

fn solve(input: &str) -> Num {
  let mut workflow_part = true;

  let mut workflows = HashMap::new();

  let mut sum = 0;

  for line in input.lines() {
    if line == "" {
      workflow_part = false;
    } else if workflow_part {
      let (name, rest) = line.split_once("{").unwrap();
      let (conditions, _rest) = rest.split_once("}").unwrap();
      let conditions = conditions.split(",").map(|c| {
        if let Some((condition, action)) = c.split_once(":") {
          if let Some((field, value)) = condition.split_once("<") {
            (Less(field.bytes().next().unwrap(), value.parse().unwrap()),action)
          } else if let Some((field, value)) = condition.split_once(">") {
            (Greater(field.bytes().next().unwrap(), value.parse().unwrap()),action) 
          } else {
            panic!("Invalid condition: {}", condition);
          }
        } else {
          (Always, c)
        }
      }).collect::<Vec<_>>();
      workflows.insert(name.to_string(), conditions);
    } else {
      let mut part: Part = HashMap::new();
      let (_rest, data) = line.split_once("{").unwrap();
      let (data, _rest) = data.split_once("}").unwrap();
      for kv in data.split(",") {
        let (k,v) = kv.split_once("=").unwrap();
        let v = v.parse::<Num>().unwrap();
        part.insert(k.bytes().next().unwrap(), v);
      }

      if run(&part, &workflows) == "A" {
        sum += "xmas".bytes().map(|d| part[&d]).sum::<Num>();
      }
    }
  }
  
  sum
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn something() {
    assert_eq!(19114, solve("px{a<2006:qkq,m>2090:A,rfg}
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
}