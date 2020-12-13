// BoatVM - still a better love story than kubernetes

#[derive(Debug, PartialEq, Eq)]
struct BoatNavigation {
    waypoint_x: isize,
    waypoint_y: isize,
    ship_x: isize,
    ship_y: isize,
}

impl Default for BoatNavigation {
    fn default() -> Self {
        Self {
            waypoint_x: 10,
            waypoint_y: 1,
            ship_x: 0,
            ship_y: 0,
        }
    }
}

impl BoatNavigation {
    fn manhattan(&self) -> usize {
        (self.ship_x.abs() + self.ship_y.abs()) as usize
    }

    fn run_one(&mut self, s: &str) -> Option<()> {
        let command = s.get(..1)?;
        let value = s.get(1..)?.parse::<isize>().ok()?;

        match command {
            "N" => self.waypoint_y += value,
            "S" => self.waypoint_y -= value,
            "E" => self.waypoint_x += value,
            "W" => self.waypoint_x -= value,
            "F" => {
                self.ship_x += value * self.waypoint_x;
                self.ship_y += value * self.waypoint_y;
            },
            // left or right rotate
            x if x == "L" || x == "R" => {
                let rot_val = if x == "L" { value } else { -value };
                let x = self.waypoint_x;
                let y = self.waypoint_y;
                match (rot_val + 360) % 360 {
                    90 => {
                        self.waypoint_x = -y;
                        self.waypoint_y = x;
                    }
                    180 => {
                        self.waypoint_x = -x;
                        self.waypoint_y = -y;
                    }
                    270 => {
                        self.waypoint_x = y;
                        self.waypoint_y = -x;
                    }
                    // none of these should show up
                    x => panic!("invalid rotate: {}", x),
                }
            },
            _ => panic!("invalid instruction {}", s),
        }
        Some(())
    }
}

fn main() {
    const DATA: &str = include_str!("../inputs/day12.txt");

    let mut boat = BoatNavigation::default();

    for command in DATA.lines() {
        if boat.run_one(command).is_none() {
            println!("failed to update boat on `{}`", command);
        }
    }

    println!("{:?}", boat);
    println!("manhattan distance: {}", boat.manhattan());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one_command() {
        let mut boat = BoatNavigation::default();

        boat.run_one("F10").unwrap();
        assert_eq!(
            boat,
            BoatNavigation {
                waypoint_x: 10,
                waypoint_y: 1,
                ship_x: 100,
                ship_y: 10,
            }
        );

        boat.run_one("N3").unwrap();
        assert_eq!(
            boat,
            BoatNavigation {
                waypoint_x: 10,
                waypoint_y: 4,
                ship_x: 100,
                ship_y: 10,
            }
        );

        boat.run_one("F7").unwrap();
        assert_eq!(
            boat,
            BoatNavigation {
                waypoint_x: 10,
                waypoint_y: 4,
                ship_x: 170,
                ship_y: 38,
            }
        );

        boat.run_one("R90").unwrap();
        assert_eq!(
            boat,
            BoatNavigation {
                waypoint_x: 4,
                waypoint_y: -10,
                ship_x: 170,
                ship_y: 38,
            }
        );

        boat.run_one("R180").unwrap();
        assert_eq!(
            boat,
            BoatNavigation {
                waypoint_x: -4,
                waypoint_y: 10,
                ship_x: 170,
                ship_y: 38,
            }
        );

        boat.run_one("L180").unwrap();
        assert_eq!(
            boat,
            BoatNavigation {
                waypoint_x: 4,
                waypoint_y: -10,
                ship_x: 170,
                ship_y: 38,
            }
        );

        boat.run_one("F11").unwrap();
        assert_eq!(
            boat,
            BoatNavigation {
                waypoint_x: 4,
                waypoint_y: -10,
                ship_x: 214,
                ship_y: -72,
            }
        );

        assert_eq!(boat.manhattan(), 286);
    }
}
