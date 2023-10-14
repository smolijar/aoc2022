use std::{
    collections::{hash_map::DefaultHasher, BTreeSet, HashMap, HashSet},
    hash::{Hash, Hasher},
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Valve {
    name: String,
    rate: u32,
    links: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct State<'a> {
    t: u8,
    at: &'a str,
    opened: BTreeSet<&'a str>,
    visited: BTreeSet<&'a str>,
    path: Vec<&'a str>,
    total_pressure: u32,
    actions: Vec<String>,
}

impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        State {
            at: "AA",
            opened: BTreeSet::new(),
            visited: BTreeSet::new(),
            actions: Vec::new(),
            path: Vec::new(),
            t: 0,
            total_pressure: 0,
        }
    }
    pub fn key(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (self.t, &self.opened, &self.path).hash(&mut hasher);
        hasher.finish()
    }

    pub fn current_pressure(&mut self, valves: &HashMap<String, Valve>) -> u32 {
        let cp = self
            .opened
            .iter()
            .map(|o| valves.get(*o).unwrap().rate)
            .sum::<u32>();
        self.actions.push(format!("Releasing: {cp:}"));
        cp
    }

    pub fn open(&mut self) {
        self.actions.push(format!("Opening: {}", self.at));
        self.path.clear();
        self.opened.insert(self.at);
    }

    pub fn visit(&mut self, dest: &'a str) {
        self.actions.push(format!("Moving to : {dest:}"));
        self.path.push(dest);
        self.at = dest;
        self.visited.insert(dest);
    }
}

pub fn valves(input: &str) -> usize {
    lazy_static! {
        static ref RE_VALVE: Regex =
            Regex::new(r"Valve (.+) has flow rate=(.+); \w+ \w+ to \w+ (.+)").unwrap();
    }
    let valves = input
        .split("\n")
        .map(|line| {
            let cap = RE_VALVE.captures_iter(line).nth(0).unwrap();
            let name = cap[1].to_string();
            let valve = Valve {
                name: name.clone(),
                rate: cap[2].parse().unwrap(),
                links: cap[3].split(", ").map(|x| x.into()).collect(),
            };
            (name, valve)
        })
        .collect::<HashMap<_, _>>();

    let mut initial_state = State::new();

    let mut states = HashMap::new();
    let mut final_states = HashMap::new();
    states.insert(initial_state.key(), initial_state);

    while !states.is_empty() {
        let mut new_states = HashMap::new();
        let mut ddd = false;
        for (_, mut state) in states {
            if valves.len() == state.visited.len()
                || state.t == 30
                || state.opened.len() == valves.len()
            {
                let rem_time = 30 - state.t;
                state.t += rem_time;
                state.total_pressure += rem_time as u32 * state.current_pressure(&valves);
                final_states.insert(state.key(), state);
                // done
                continue;
            }
            state.t += 1;
            state.total_pressure += state.current_pressure(&valves);
            let mut options = valves
                .get(state.at)
                .unwrap()
                .links
                .into_iter()
                .filter(|l| !state.path.contains(l))
                .map(|v| {
                    let mut new_state = state.clone();
                    new_state.visit(v);
                    new_state
                })
                .collect_vec();
            if !state.opened.contains(&state.at) && valves.get(state.at).unwrap().rate > 0 {
                let mut new_state = state.clone();
                new_state.open();
                options.push(new_state)
            }
            for o in options {
                // let PFX = "Releasing: 0/Moving to : DD/Releasing: 0/Opening: DD/Releasing: 20/Moving to : AA/Releasing: 20/Moving to : BB/Releasing: 20/Opening: BB/Releasing: 33/Moving to : AA/Releasing: 33/Moving to : II/Releasing: 33/Moving to : JJ/Releasing: 33/Opening: JJ/Releasing: 54/Moving to : II/Releasing: 54/Moving to : AA/Releasing: 54/Moving to : DD/Releasing: 54/Moving to : EE/Releasing: 54/Moving to : FF/Releasing: 54/Moving to : GG/Releasing: 54/Moving to : HH/Releasing: 54/Opening: HH/Releasing: 76/Moving to : GG/Releasing: 76/Moving to : FF/Releasing: 76/Moving to : EE/Releasing: 76/Opening: EE/Releasing: 79/Moving to : DD/Releasing: 79";
                // if state.actions.join("/") == PFX {
                //     // println!("{}", o.key());
                //     // println!("{:#?}", &o);
                //     // println!("{:}", s.actions.join("/"));
                // }
                // if o.key() == 17003793315770626314 {
                //     let current = new_states.get(&o.key());
                //     // println!("{:#?}", current);
                //     // println!("{:#?}", o);
                // }
                match new_states.get(&o.key()) {
                    None => {
                        new_states.insert(o.key(), o);
                    }
                    Some(same) if same.total_pressure < o.total_pressure => {
                        new_states.insert(o.key(), o);
                    }
                    _ => {}
                }
            }
        }
        for new in new_states.values().clone() {
            let PFX = "Releasing: 0/Moving to : DD/Releasing: 0/Opening: DD/Releasing: 20/Moving to : AA/Releasing: 20/Moving to : BB/Releasing: 20/Opening: BB/Releasing: 33/Moving to : AA/Releasing: 33/Moving to : II/Releasing: 33/Moving to : JJ/Releasing: 33/Opening: JJ/Releasing: 54/Moving to : II/Releasing: 54/Moving to : AA/Releasing: 54/Moving to : DD/Releasing: 54/Moving to : EE/Releasing: 54/Moving to : FF/Releasing: 54/Moving to : GG/Releasing: 54/Moving to : HH/Releasing: 54/Opening: HH/Releasing: 76/Moving to : GG/Releasing: 76/Moving to : FF/Releasing: 76/Moving to : EE/Releasing: 76/Opening: EE/Releasing: 79/Moving to : DD/Releasing: 79";
            if new.actions.join("/").starts_with(PFX) {
                println!("{:#?}", new);
                // println!("{:#?}", &o);
                // println!("{:}", s.actions.join("/"));
            }
        }
        states = new_states;
    }

    let res = final_states.values().map(|s| s.total_pressure).max();

    for s in final_states
        .values()
        .sorted_by(|a, b| a.total_pressure.cmp(&b.total_pressure))
    {
        // println!("{:#?}", s);
        let PFX = "Releasing: 0/Moving to : DD/Releasing: 0/Opening: DD/Releasing: 20/Moving to : AA/Releasing: 20/Moving to : BB/Releasing: 20/Opening: BB/Releasing: 33/Moving to : AA/Releasing: 33/Moving to : II/Releasing: 33/Moving to : JJ/Releasing: 33/Opening: JJ/Releasing: 54/Moving to : II/Releasing: 54/Moving to : AA/Releasing: 54/Moving to : DD/Releasing: 54/Moving to : EE/Releasing: 54/Moving to : FF/Releasing: 54/Moving to : GG/Releasing: 54/Moving to : HH/Releasing: 54/Opening: HH/Releasing: 76/Moving to : GG/Releasing: 76/Moving to : FF/Releasing: 76/Moving to : EE/Releasing: 76/Opening: EE";
        if s.actions.join("/").starts_with(PFX) {
            // println!("{:}", s.actions.join("/"));
        }
    }

    println!("{:?}", res);
    println!("{:?}", valves);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(valves(&inputs::demo_input(16)), 1);
    }
}
