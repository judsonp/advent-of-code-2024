use advent_of_code::util::point::Point2D;
use nom::{
    bytes::complete::tag,
    character::complete::i64 as parse_i64,
    character::complete::newline,
    combinator::{complete, map},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, input) = parse_input(input).unwrap();
    Some(input.machines.iter().filter_map(coins).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut input) = parse_input(input).unwrap();
    for machine in &mut input.machines {
        machine.prize = machine.prize + Point2D::new(10000000000000, 10000000000000);
    }
    Some(input.machines.iter().filter_map(coins).sum())
}

fn coins(machine: &ClawMachine) -> Option<u64> {
    let button_a = machine.button[0];
    let button_b = machine.button[1];
    let prize = machine.prize;
    let det = button_a.x() * button_b.y() - button_a.y() * button_b.x();
    let a = (prize.x() * button_b.y() - prize.y() * button_b.x()) / det;
    let b = (button_a.x() * prize.y() - button_a.y() * prize.x()) / det;
    if (
        button_a.x() * a + button_b.x() * b,
        button_a.y() * a + button_b.y() * b,
    ) == (prize.x(), prize.y())
    {
        Some((a * 3 + b) as u64)
    } else {
        None
    }
}

struct Input {
    machines: Vec<ClawMachine>,
}

struct ClawMachine {
    button: [Button; 2],
    prize: Point2D<i64>,
}

type Button = Point2D<i64>;

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        complete(separated_list1(newline, parse_claw_machine)),
        |v| Input { machines: v },
    )(input)
}

fn parse_claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    map(
        tuple((parse_button("A"), parse_button("B"), parse_prize)),
        |(a, b, prize)| ClawMachine {
            button: [a, b],
            prize,
        },
    )(input)
}

fn parse_button<'a>(name: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, Button> {
    move |input| {
        let (input, (_, _, _, x, _, y, _)) = complete(tuple((
            tag("Button "),
            tag(name),
            tag(": X+"),
            parse_i64,
            tag(", Y+"),
            parse_i64,
            newline,
        )))(input)?;
        Ok((input, Point2D::new(x, y)))
    }
}

fn parse_prize(input: &str) -> IResult<&str, Point2D<i64>> {
    let (input, (_, x, _, y, _)) = complete(tuple((
        tag("Prize: X="),
        parse_i64,
        tag(", Y="),
        parse_i64,
        newline,
    )))(input)?;
    Ok((input, Point2D::new(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (_, input) =
            parse_input(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(input.machines.len(), 4);
        assert_eq!(input.machines[0].button[0], Point2D::new(94, 34));
        assert_eq!(input.machines[0].button[1], Point2D::new(22, 67));
        assert_eq!(input.machines[0].prize, Point2D::new(8400, 5400));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
