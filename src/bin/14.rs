use advent_of_code::util::{bbox::BoundingBox2D, point::Point2D};
use nom::{
    bytes::complete::tag,
    character::complete::i64 as parse_i64,
    character::complete::newline,
    combinator::{complete, map},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use rayon::iter::{ParallelBridge, ParallelIterator as _};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u64> {
    part_one_constrained(
        input,
        BoundingBox2D::new(Point2D::new(0, 0), Point2D::new(100, 102)),
    )
}

pub fn part_one_constrained(input: &str, space: BoundingBox2D<i64>) -> Option<u64> {
    let (_, mut robots) = parse_input(input).unwrap();
    advance_time(&mut robots, &space, 100);
    Some(safety_factor(robots.into_iter(), &space))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, robots) = parse_input(input).unwrap();
    let space = BoundingBox2D::new(Point2D::new(0, 0), Point2D::new(100, 102));

    let safeties = (0..=10000).par_bridge().map(|time| {
        let robots = advance_time_iter(robots.iter(), &space, time);
        (time, safety_factor(robots, &space))
    });

    let (time, _) = safeties.min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

    Some(time)
}

#[allow(dead_code)]
fn print_robots(robots: &[Robot], space: &BoundingBox2D<i64>) {
    let mut grid = vec![vec!['.'; space.upper().x() as usize + 1]; space.upper().y() as usize + 1];
    for robot in robots {
        grid[robot.position.y() as usize][robot.position.x() as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn advance_time(robots: &mut [Robot], space: &BoundingBox2D<i64>, time: u64) {
    let wrap = BoundingBox2D::new(Point2D::new(0, 0), *space.upper() + Point2D::new(1, 1));
    for robot in robots {
        robot.position = robot
            .position
            .wrapped_add(&robot.velocity.multiply(time as i64), &wrap);
    }
}

fn advance_time_iter<'a>(
    robots: impl Iterator<Item = &'a Robot> + 'a,
    space: &'a BoundingBox2D<i64>,
    time: u64,
) -> impl Iterator<Item = Robot> + 'a {
    let wrap = BoundingBox2D::new(Point2D::new(0, 0), *space.upper() + Point2D::new(1, 1));
    robots.map(move |robot| Robot {
        position: robot
            .position
            .wrapped_add(&robot.velocity.multiply(time as i64), &wrap),
        velocity: robot.velocity,
    })
}

fn safety_factor(robots: impl Iterator<Item = Robot>, space: &BoundingBox2D<i64>) -> u64 {
    assert!(space.lower().x() == 0 && space.lower().y() == 0);

    let midpoint = Point2D::new(space.upper().x() / 2, space.upper().y() / 2);
    let quadrants = [
        BoundingBox2D::new(Point2D::new(0, 0), midpoint - Point2D::new(1, 1)),
        BoundingBox2D::new(
            Point2D::new(midpoint.x() + 1, 0),
            Point2D::new(space.upper().x(), midpoint.y() - 1),
        ),
        BoundingBox2D::new(
            Point2D::new(0, midpoint.y() + 1),
            Point2D::new(midpoint.x() - 1, space.upper().y()),
        ),
        BoundingBox2D::new(
            midpoint + Point2D::new(1, 1),
            Point2D::new(space.upper().x(), space.upper().y()),
        ),
    ];

    let mut quadrant_scores = [0; 4];

    for robot in robots {
        for (i, quadrant) in quadrants.iter().enumerate() {
            if quadrant.contains(&robot.position) {
                quadrant_scores[i] += 1;
                continue;
            }
        }
    }

    quadrant_scores.iter().product()
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    complete(many1(terminated(parse_robot, newline)))(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(
        tuple((tag("p="), parse_point, tag(" v="), parse_point)),
        |(_, position, _, velocity)| Robot { position, velocity },
    )(input)
}

fn parse_point(input: &str) -> IResult<&str, Point2D<i64>> {
    map(tuple((parse_i64, tag(","), parse_i64)), |(x, _, y)| {
        Point2D::new(x, y)
    })(input)
}

type Input = Vec<Robot>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Robot {
    position: Point2D<i64>,
    velocity: Point2D<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let (_, input) =
            parse_input(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(input.len(), 12);
    }

    #[test]
    fn test_advance_time() {
        let mut robots = vec![Robot {
            position: Point2D::new(2, 4),
            velocity: Point2D::new(2, -3),
        }];
        let space = BoundingBox2D::new(Point2D::new(0, 0), Point2D::new(10, 6));

        advance_time(&mut robots, &space, 5);

        assert_eq!(robots[0].position, Point2D::new(1, 3));
    }

    #[test]
    fn test_part_one() {
        let result = part_one_constrained(
            &advent_of_code::template::read_file("examples", DAY),
            BoundingBox2D::new(Point2D::new(0, 0), Point2D::new(10, 6)),
        );
        assert_eq!(result, Some(12));
    }
}
