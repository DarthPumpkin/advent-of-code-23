use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;


pub type Solution1 = u64;
pub type Solution2 = u64;

const TERMINAL_STATES: [&str; 2] = ["A", "R"];
const RATING_RANGE: RangeInclusive<u32> = 1..=4000;


pub fn solve_part1(input: &PuzzleInput) -> Solution1 {
    input.ratings.iter()
        .filter_map(|rating| {
            if traverse_single(rating, &input.first, &input.workflows) {
                Some(rating.sum() as Solution1)
            } else {
                None
            }
        }).sum()
}

pub fn solve_part2(input: &PuzzleInput) -> Solution2 {
    let starting_rect = Rectangle {
        x: RATING_RANGE.clone(),
        m: RATING_RANGE.clone(),
        a: RATING_RANGE.clone(),
        s: RATING_RANGE.clone()
    };
    let starting_union = RectangleUnion { rectangles: vec![starting_rect] };
    let final_union = traverse_union(&starting_union, &input.first, &input.workflows);
    final_union.area() as Solution2
}

fn traverse_single(rating: &Rating, start: &str, workflows: &HashMap<String, Workflow>) -> bool {
    let mut next_workflow_id = start;
    loop {
        let workflow = workflows.get(next_workflow_id).unwrap();
        let mut matched = false;
        'inner: for rule in workflow.rules.iter() {
            if rule.condition.matches(rating) {
                matched = true;
                next_workflow_id = &rule.direction;
                break 'inner;
            }
        }
        if !matched {
            next_workflow_id = &workflow.default;
        }
        if next_workflow_id == "A" {
            return true;
        }
        if next_workflow_id == "R" {
            return false;
        }
    }
}

fn traverse_union(union: &RectangleUnion, start: &str, workflows: &HashMap<String, Workflow>) -> RectangleUnion {
    let mut state: HashMap<&str, RectangleUnion> = [(start, union.clone())].into();
    while state.keys().any(|id| !TERMINAL_STATES.contains(id)) {
        let mut new_state = HashMap::new();
        for (id, region) in state.iter() {
            if TERMINAL_STATES.contains(id) {
                continue;
            }
            let workflow = workflows.get(&id.to_string()).unwrap();
            let mut remaining_region = region.clone();
            'rules: for rule in workflow.rules.iter() {
                let (mapped_region, remainder) = remaining_region.split(&rule.condition);
                remaining_region = remainder;
                if !mapped_region.is_empty() {
                    let next_region = new_state.entry(rule.direction.as_ref()).or_insert_with(RectangleUnion::new);
                    next_region.rectangles.extend(mapped_region.rectangles);
                }
                if remaining_region.is_empty() {
                    break 'rules;
                }
            }
            if !remaining_region.is_empty() {
                let next_region = new_state.entry(workflow.default.as_ref()).or_insert_with(RectangleUnion::new);
                next_region.rectangles.extend(remaining_region.rectangles);
            }
        }
        if let Some(accepted_region) = state.get("A") {
            let new_accepted_region = new_state.entry("A").or_insert_with(RectangleUnion::new);
            new_accepted_region.rectangles.extend(accepted_region.rectangles.iter().cloned());
        }
        state = new_state;
    }
    state["A"].clone()
}

#[derive(Clone, Debug)]
struct RectangleUnion {
    rectangles: Vec<Rectangle>
}

impl RectangleUnion {
    fn new() -> Self {
        RectangleUnion { rectangles: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.rectangles.iter().all(|rect| rect.is_empty())
    }

    fn split(&self, condition: &Condition) -> (RectangleUnion, RectangleUnion) {
        let mut matched = RectangleUnion::new();
        let mut remaining = RectangleUnion::new();
        for rect in self.rectangles.iter() {
            let (matching_rect, remaining_rect) = rect.split(condition);
            if !matching_rect.is_empty() {
                matched.rectangles.push(matching_rect);
            }
            if !remaining_rect.is_empty() {
                remaining.rectangles.push(remaining_rect);
            }
        }
        (matched, remaining)
    }

    fn area(&self) -> u64 {
        self.rectangles.iter().map(Rectangle::area).sum()
    }
}

#[derive(Clone, Debug)]
struct Rectangle {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>
}

impl Rectangle {
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }

    fn area(&self) -> u64 {
        if self.is_empty() { 0 } else {
            (self.x.end() - self.x.start() + 1) as u64
            * (self.m.end() - self.m.start() + 1) as u64
            * (self.a.end() - self.a.start() + 1) as u64
            * (self.s.end() - self.s.start() + 1) as u64
        }
    }

    fn split(&self, condition: &Condition) -> (Rectangle, Rectangle) {
        match condition {
            Condition::GreaterThan(Attribute::X, value) => {
                let matching = Rectangle { x: max(value + 1, *self.x.start())..=*self.x.end(), ..self.clone() };
                let remaining = Rectangle { x: *self.x.start()..=min(*value, *self.x.end()), ..self.clone() };
                (matching, remaining)
            }
            Condition::GreaterThan(Attribute::M, value) => {
                let matching = Rectangle { m: max(value + 1, *self.m.start())..=*self.m.end(), ..self.clone() };
                let remaining = Rectangle { m: *self.m.start()..=min(*value, *self.m.end()), ..self.clone() };
                (matching, remaining)
            }
            Condition::GreaterThan(Attribute::A, value) => {
                let matching = Rectangle { a: max(value + 1, *self.a.start())..=*self.a.end(), ..self.clone() };
                let remaining = Rectangle { a: *self.a.start()..=min(*value, *self.a.end()), ..self.clone() };
                (matching, remaining)
            }
            Condition::GreaterThan(Attribute::S, value) => {
                let matching = Rectangle { s: max(value + 1, *self.s.start())..=*self.s.end(), ..self.clone() };
                let remaining = Rectangle { s: *self.s.start()..=min(*value, *self.s.end()), ..self.clone() };
                (matching, remaining)
            }
            Condition::LessThan(Attribute::X, value) => {
                let matching = Rectangle { x: *self.x.start()..=min(value - 1, *self.x.end()), ..self.clone() };
                let remaining = Rectangle { x: max(*value, *self.x.start())..=*self.x.end(), ..self.clone() };
                (matching, remaining)
            }
            Condition::LessThan(Attribute::M, value) => {
                let matching = Rectangle { m: *self.m.start()..=min(value - 1, *self.m.end()), ..self.clone() };
                let remaining = Rectangle { m: max(*value, *self.m.start())..=*self.m.end(), ..self.clone() };
                (matching, remaining)
            }
            Condition::LessThan(Attribute::A, value) => {
                let matching = Rectangle { a: *self.a.start()..=min(value - 1, *self.a.end()), ..self.clone() };
                let remaining = Rectangle { a: max(*value, *self.a.start())..=*self.a.end(), ..self.clone() };
                (matching, remaining)
            }
            Condition::LessThan(Attribute::S, value) => {
                let matching = Rectangle { s: *self.s.start()..=min(value - 1, *self.s.end()), ..self.clone() };
                let remaining = Rectangle { s: max(*value, *self.s.start())..=*self.s.end(), ..self.clone() };
                (matching, remaining)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    workflows: HashMap<String, Workflow>,
    first: String,
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

impl Condition {
    fn matches(&self, rating: &Rating) -> bool {
        match self {
            Condition::GreaterThan(Attribute::X, value) => rating.x > *value,
            Condition::GreaterThan(Attribute::M, value) => rating.m > *value,
            Condition::GreaterThan(Attribute::A, value) => rating.a > *value,
            Condition::GreaterThan(Attribute::S, value) => rating.s > *value,
            Condition::LessThan(Attribute::X, value) => rating.x < *value,
            Condition::LessThan(Attribute::M, value) => rating.m < *value,
            Condition::LessThan(Attribute::A, value) => rating.a < *value,
            Condition::LessThan(Attribute::S, value) => rating.s < *value,
        }
    }
}

impl Rating {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
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
        let first = "in".to_string();
        Ok(PuzzleInput { workflows, first, ratings })
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
