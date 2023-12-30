use std::{collections::HashMap, fmt::Display};

// Idea: instead of making so many structs we could just use tuples:
// e.g. type Something = (String, HashSet<String, (String, char, char, usize, String)>).

type Workflows = HashMap<String, Workflow>;
type Parts = Vec<Part>;
type Expressions = Vec<Expression>;

#[derive(Debug)]
struct Context {
    workflows: Workflows,
    parts: Parts,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    expressions: Expressions, // a < 2006 : qkq
    default: Action,          // Reject
}

impl Display for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> ", self.name)?;
        for ex in &self.expressions {
            write!(f, "{}", Into::<char>::into(&ex.rating))?;
            write!(f, "{}", Into::<char>::into(&ex.rating))?;
            write!(f, "{:?}", ex.value)?;
            write!(f, "={:?}", ex.action)?;
            write!(f, ",")?;
        }
        write!(f, ":{:?}", self.default)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Expression {
    rating: Rating,     // x
    operator: Operator, // <
    value: usize,       // 1000
    action: Action,     // GoTo -> xyz
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
enum Rating {
    ExtremelyCoolLooking, // x
    Musical,              // m
    Aerodynamic,          // a
    Shiny,                // s
}

impl TryFrom<char> for Rating {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Rating::ExtremelyCoolLooking),
            'm' => Ok(Rating::Musical),
            'a' => Ok(Rating::Aerodynamic),
            's' => Ok(Rating::Shiny),
            _ => return Err(format!("could not parse rating from {value}")),
        }
    }
}

impl From<&Rating> for char {
    fn from(value: &Rating) -> Self {
        match value {
            Rating::ExtremelyCoolLooking => 'x',
            Rating::Musical => 'm',
            Rating::Aerodynamic => 'a',
            Rating::Shiny => 's',
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    LessThan,
    GreaterThan,
}

impl From<&Operator> for char {
    fn from(value: &Operator) -> Self {
        match value {
            Operator::LessThan => '<',
            Operator::GreaterThan => '>',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Accept,
    Reject,
    GoTo(String),
}

#[derive(Debug)]
struct Part {
    ratings: Vec<(Rating, usize)>,
}
impl Part {
    fn total(&self) -> usize {
        self.ratings.iter().map(|x| x.1).sum::<usize>()
    }
}

// 423929 too high
// 318388 too low
// 328526 too low
fn main() {
    println!("Starting Process");
    let now = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("Finished in {:?}", now.elapsed());
    println!("Solution: {:?}", output);
}

fn process(input: &str) -> usize {
    let (_, ctx) = parsing::parse(input).unwrap();

    for (_, x) in &ctx.workflows {
        println!("{x}");
    }

    let in_workflow = ctx.workflows.get("in").expect("should have 'in' workflow");

    ctx.parts
        .iter()
        .filter_map(|part| {
            if is_part_accepted(&part, &in_workflow, &ctx.workflows) {
                return Some(part.total());
            }
            None
        })
        .sum::<usize>()
}

fn is_part_accepted(part: &Part, workflow: &Workflow, workflows: &Workflows) -> bool {
    let mut current_workflow = workflow;
    loop {
        match run_part_through_workflow(part, current_workflow) {
            Action::Accept => return true,
            Action::Reject => return false,
            Action::GoTo(next_workflow_name) => {
                let next_workflow = workflows
                    .get(&next_workflow_name)
                    .expect("next workflow should be valid");
                current_workflow = next_workflow;
            }
        };
    }
}

fn run_part_through_workflow(part: &Part, workflow: &Workflow) -> Action {
    // For every part's rating run it through the matching expression,
    // if it's successful return the action, otherwise try the other ratings.
    for (rating, value) in &part.ratings {
        // Sometimes there are duplicates – cbp{m>3589:R,x<3159:A,x>3716:R,A}
        let expressions = workflow.expressions.iter().filter(|x| x.rating == *rating);
        for expr in expressions {
            println!(
                "{} --- {:?} = {} {} {} -> {:?}",
                workflow.name,
                rating,
                value,
                Into::<char>::into(&expr.operator),
                expr.value,
                expr.action
            );
            // Return the first successful expression.
            if let Some(action) = run_expression(*value, expr) {
                return action;
            }
        }
    }
    // When all else fails just return the default action.
    workflow.default.clone()
}

fn run_expression(value: usize, expr: &Expression) -> Option<Action> {
    if match expr.operator {
        Operator::LessThan => value < expr.value,
        Operator::GreaterThan => value > expr.value,
    } {
        Some(expr.action.clone())
    } else {
        None
    }
}

// Parsing is half the challenge.
mod parsing {
    use std::collections::HashMap;

    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, anychar, digit1, newline},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult,
    };

    use crate::{Action, Context, Expression, Operator, Part, Parts, Rating, Workflow, Workflows};

    pub(crate) fn parse(input: &str) -> IResult<&str, Context> {
        map(
            separated_pair(parse_workflows, tag("\n\n"), parse_parts),
            |(workflows, ratings)| Context {
                workflows,
                parts: ratings,
            },
        )(input)
    }

    fn parse_workflows(input: &str) -> IResult<&str, Workflows> {
        let (input, workflows) = separated_list1(newline, parse_workflow)(input)?;

        let mut map = HashMap::new();
        for workflow in workflows {
            map.insert(workflow.name.to_owned(), workflow);
        }

        Ok((input, map))
    }

    fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
        use nom::character::complete::char;
        let (input, (name, (expressions, default))) = tuple((
            alpha1,
            delimited(
                char('{'),
                pair(many1(terminated(parse_expression, char(','))), parse_action),
                char('}'),
            ),
        ))(input)?;

        Ok((
            input,
            Workflow {
                name: name.to_owned(),
                expressions,
                default,
            },
        ))
    }

    // a>1716:R
    fn parse_expression(input: &str) -> IResult<&str, Expression> {
        let (input, rating) = anychar(input)?;
        let rating = rating.try_into().map_err(|_| {
            nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Char))
        })?;
        let (input, operator) = anychar(input)?;
        let operator = match operator {
            '<' => Operator::LessThan,
            '>' => Operator::GreaterThan,
            _ => {
                return Err(nom::Err::Error(nom::error::make_error(
                    input,
                    nom::error::ErrorKind::Char,
                )))
            }
        };
        let (input, value) = map(digit1, |v: &str| v.parse::<usize>().unwrap())(input)?;
        let (input, action) = preceded(tag(":"), parse_action)(input)?;
        Ok((
            input,
            Expression {
                rating,
                operator,
                value,
                action,
            },
        ))
    }

    fn parse_action(input: &str) -> IResult<&str, Action> {
        let (input, action) = alpha1(input)?;
        let action = match action {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::GoTo(action.to_owned()),
        };
        Ok((input, action))
    }

    fn parse_parts(input: &str) -> IResult<&str, Parts> {
        use nom::character::complete::char;
        separated_list1(newline, delimited(char('{'), parse_part_values, char('}')))(input)
    }

    fn parse_part_values(input: &str) -> IResult<&str, Part> {
        use nom::character::complete::char;
        let (input, ratings) = separated_list1(char(','), parse_part_value)(input)?;
        Ok((input, Part { ratings }))
    }

    fn parse_part_value(input: &str) -> IResult<&str, (Rating, usize)> {
        use nom::character::complete::char;
        let (input, (rating, value)) = separated_pair(
            anychar,
            char('='),
            map(digit1, |v: &str| v.parse::<usize>().unwrap()),
        )(input)?;

        let rating = rating.try_into().map_err(|_| {
            nom::Err::Failure(nom::error::make_error(input, nom::error::ErrorKind::Char))
        })?;

        Ok((input, (rating, value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day19_2023_part1() {
        let input = "in{x<5:A,x>6:R,x>7:R,R}

{x=4,m=0,a=0,s=0}";
        assert_eq!(process(input), 4);

        let input = "in{x<5:R,x>6:A,x>7:R,R}

{x=7,m=0,a=0,s=0}";
        assert_eq!(process(input), 7);

        let input = "in{x<5:R,x>6:R,x>7:A,R}

{x=8,m=0,a=0,s=0}";
        assert_eq!(process(input), 0);

        let input = "jk{m>2673:pv,m<2405:A,R}
in{m<2:px,jk}
px{a>2:A,R}
pv{s>1:A,R}

{x=1,m=2,a=3,s=4}";

        assert_eq!(process(input), 10);

        let input = "px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(process(input), 19114);
    }
}
