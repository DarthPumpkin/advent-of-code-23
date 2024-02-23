use std::collections::HashMap;
// use std::fmt;
use std::str::FromStr;

// use crate::aux;


pub type Solution1 = u64;
pub type Solution2 = u64;


pub fn solve_part1(input: &PuzzleInput) -> Solution1 {
    dbg!(input);
    todo!()
}

pub fn solve_part2(input: &PuzzleInput) -> Solution2 {
    todo!()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    workflows: HashMap<String, Workflow>,
    ratings: Vec<Rating>
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Workflow {
    rules: Vec<Rule>,
    default: String
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    condition: Condition,
    direction: String
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Condition {
    GreaterThan(Attribute, u32),
    LessThan(Attribute, u32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Attribute {
    X, M, A, S
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

impl FromStr for PuzzleInput {
    type Err = String;

    fn from_str(s: &str) -> Result<PuzzleInput, Self::Err> {
        let mut parts = s.split("\n\n");
        let workflows_str = parts.next().ok_or("Missing workflows")?;
        let mut workflows = HashMap::new();
        for line in workflows_str.lines() {
            let mut parts = line.split("{");
            let name = parts.next().ok_or("Missing name")?;
            let rules_str = parts.next().ok_or("Missing rules")?;
            let rules_str = rules_str.strip_suffix("}").ok_or("Missing closing brace")?;
            let workflow: Workflow = rules_str.parse()?;
            workflows.insert(name.to_string(), workflow);
        }
        let ratings_str = parts.next().ok_or("Missing ratings")?;
        let ratings: Result<Vec<Rating>, _> = ratings_str.lines().map(|line| line.parse()).collect();
        let ratings = ratings?;
        Ok(PuzzleInput { workflows, ratings })
    }
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Workflow, Self::Err> {
        let rules_parts: Vec<_> = s.split(",").collect();
        let default = rules_parts[rules_parts.len() - 1].to_string();
        let rules: Result<Vec<Rule>, _> = rules_parts[..rules_parts.len() - 1].iter()
            .map(|rule| rule.parse())
            .collect();
        let rules = rules?;
        Ok(Workflow { rules, default })
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Rule, Self::Err> {
        let mut parts = s.split(":");
        let condition: Condition = parts.next().ok_or("Missing attribute")?.parse()?;
        let direction = parts.next().ok_or("Missing direction")?;
        Ok(Rule { condition, direction: direction.to_string() })
    }
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(s: &str) -> Result<Condition, Self::Err> {
        let symbol = if s.contains('<') {'<'} else {'>'};
        let mut parts = s.split(symbol);
        let attribute: Attribute = parts.next().ok_or("Missing attribute")?.parse()?;
        let value: u32 = parts.next().ok_or("Missing value")?.parse().map_err(|e| format!("Invalid value: {}", e))?;
        match symbol {
            '<' => Ok(Condition::LessThan(attribute, value)),
            '>' => Ok(Condition::GreaterThan(attribute, value)),
            _ => unreachable!("Invalid symbol: {}", symbol)
        }
    }
}

impl FromStr for Attribute {
    type Err = String;

    fn from_str(s: &str) -> Result<Attribute, Self::Err> {
        match s {
            "x" => Ok(Attribute::X),
            "m" => Ok(Attribute::M),
            "a" => Ok(Attribute::A),
            "s" => Ok(Attribute::S),
            _ => Err(format!("Invalid attribute: {}", s))
        }
    }
}

impl FromStr for Rating {
    type Err = String;

    fn from_str(s: &str) -> Result<Rating, Self::Err> {
        let s = s.strip_prefix("{").ok_or("Missing opening brace")?;
        let s = s.strip_suffix("}").ok_or("Missing closing brace")?;
        let mut parts = s.split(",");
        let x: u32 = parts.next().ok_or("Missing x")?[2..].parse().map_err(|e| format!("Invalid x: {}", e))?;
        let m: u32 = parts.next().ok_or("Missing m")?[2..].parse().map_err(|e| format!("Invalid m: {}", e))?;
        let a: u32 = parts.next().ok_or("Missing a")?[2..].parse().map_err(|e| format!("Invalid a: {}", e))?;
        let s: u32 = parts.next().ok_or("Missing s")?[2..].parse().map_err(|e| format!("Invalid s: {}", e))?;
        Ok(Rating { x, m, a, s })
    }
}

/////////////////////////////////////////////////////////////////////////////
// Pretty-printing
/////////////////////////////////////////////////////////////////////////////

// impl fmt::Display for PuzzleInput {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }
