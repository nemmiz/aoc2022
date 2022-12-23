use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl Resources {
    fn new(ore: u8, clay: u8, obsidian: u8, geode: u8) -> Resources {
        Resources {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn ge(&self, other: &Resources) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn add(&self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }

    fn update(&self, cost: &Resources, produce: &Resources) -> Resources {
        Resources {
            ore: self.ore - cost.ore + produce.ore,
            clay: self.clay - cost.clay + produce.clay,
            obsidian: self.obsidian - cost.obsidian + produce.obsidian,
            geode: self.geode - cost.geode + produce.geode,
        }
    }

    fn add_ore(&self) -> Resources {
        Resources {
            ore: self.ore + 1,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,
        }
    }
    fn add_clay(&self) -> Resources {
        Resources {
            ore: self.ore,
            clay: self.clay + 1,
            obsidian: self.obsidian,
            geode: self.geode,
        }
    }
    fn add_obsidian(&self) -> Resources {
        Resources {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian + 1,
            geode: self.geode,
        }
    }
    fn add_geode(&self) -> Resources {
        Resources {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode + 1,
        }
    }
}

impl Blueprint {
    fn new(line: &str) -> Blueprint {
        let nums: Vec<u8> = line
            .split_ascii_whitespace()
            .filter_map(|x| x.parse::<u8>().ok())
            .collect();
        Blueprint {
            ore_robot_cost: Resources::new(nums[0], 0, 0, 0),
            clay_robot_cost: Resources::new(nums[1], 0, 0, 0),
            obsidian_robot_cost: Resources::new(nums[2], nums[3], 0, 0),
            geode_robot_cost: Resources::new(nums[4], 0, nums[5], 0),
        }
    }

    fn simulate(&self, minutes: usize) -> usize {
        let mut queue = VecDeque::new();

        let costs = [
            &self.ore_robot_cost,
            &self.clay_robot_cost,
            &self.obsidian_robot_cost,
            &self.geode_robot_cost,
        ];

        let max_ore_robots = costs.iter().map(|cost| cost.ore).max().unwrap();
        let max_clay_robots = costs.iter().map(|cost| cost.clay).max().unwrap();
        let max_obsidian_robots = costs.iter().map(|cost| cost.obsidian).max().unwrap();

        queue.push_back((Resources::new(1, 0, 0, 0), Resources::new(0, 0, 0, 0), 0));

        let mut most_geodes: Vec<u8> = vec![0; minutes + 1];

        while let Some((robots, resources, minute)) = queue.pop_front() {
            if minute > minutes {
                continue;
            }

            if resources.geode < most_geodes[minute] {
                continue;
            }

            most_geodes[minute] = resources.geode;

            if minute == minutes {
                continue;
            }

            if resources.ge(&self.geode_robot_cost) {
                queue.push_back((
                    robots.add_geode(),
                    resources.update(&self.geode_robot_cost, &robots), //.sub(&self.geode_robot_cost).add(&robots),
                    minute + 1,
                ));
                continue;
            }

            if robots.obsidian < max_obsidian_robots && resources.ge(&self.obsidian_robot_cost) {
                queue.push_back((
                    robots.add_obsidian(),
                    resources.update(&self.obsidian_robot_cost, &robots),
                    minute + 1,
                ));
            }

            if robots.clay < max_clay_robots && resources.ge(&self.clay_robot_cost) {
                queue.push_back((
                    robots.add_clay(),
                    resources.update(&self.clay_robot_cost, &robots),
                    minute + 1,
                ));
            }

            if robots.ore < max_ore_robots && resources.ge(&self.ore_robot_cost) {
                queue.push_back((
                    robots.add_ore(),
                    resources.update(&self.ore_robot_cost, &robots),
                    minute + 1,
                ));
            }

            queue.push_back((robots.clone(), resources.add(&robots), minute + 1));
        }
        most_geodes[minutes].into()
    }
}

fn main() {
    let input = include_str!("../../../input/19.txt");
    let blueprints: Vec<Blueprint> = input.lines().map(|line| Blueprint::new(line)).collect();

    let mut part1 = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let most = blueprint.simulate(24);
        part1 += (i + 1) * most;
    }
    println!("{}", part1);

    let mut part2 = 1;
    for blueprint in blueprints.iter().take(3) {
        let most = blueprint.simulate(32);
        part2 *= most;
    }
    println!("{}", part2);
}
