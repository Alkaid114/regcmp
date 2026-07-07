use std::collections::BTreeSet;

use crate::regex::Regex;

#[derive(Debug, Clone)]
pub struct NFA {
    pub start: usize,
    pub accept: usize,
    pub transitions: Vec<Vec<(Option<char>, usize)>>,
    pub state_count: usize,
    pub state_names: Vec<String>,
}

impl NFA {
    pub fn from_regex(re: &Regex) -> Self {
        let mut builder = NFABuilder {
            transitions: Vec::new(),
            next_state: 0,
        };
        let frag = builder.build_fragment(re);
        let state_names: Vec<String> = (0..builder.next_state).map(|i| format!("q{}", i)).collect();
        NFA {
            start: frag.start,
            accept: frag.accept,
            transitions: builder.transitions,
            state_count: builder.next_state,
            state_names,
        }
    }

    fn state_name(&self, id: usize) -> &str {
        self.state_names.get(id).map_or("?", |s| s.as_str())
    }

    pub fn dump(&self) {
        println!(
            "NFA: {} 个状态, 开始: {}, 接受: {}",
            self.state_count,
            self.state_name(self.start),
            self.state_name(self.accept),
        );
        for s in 0..self.state_count {
            for (sym, to) in &self.transitions[s] {
                match sym {
                    Some(c) => println!(
                        "  {} --{}--> {}",
                        self.state_name(s),
                        c,
                        self.state_name(*to)
                    ),
                    None => println!("  {} --ε--> {}", self.state_name(s), self.state_name(*to)),
                }
            }
        }
    }

    pub fn display_string(&self) -> String {
        let mut s = format!(
            "NFA: {} 个状态, 开始: {}, 接受: {}",
            self.state_count,
            self.state_name(self.start),
            self.state_name(self.accept),
        );
        for state in 0..self.state_count {
            for (sym, to) in &self.transitions[state] {
                match sym {
                    Some(c) => s.push_str(&format!(
                        "\n  {} --{}--> {}",
                        self.state_name(state),
                        c,
                        self.state_name(*to)
                    )),
                    None => s.push_str(&format!(
                        "\n  {} --ε--> {}",
                        self.state_name(state),
                        self.state_name(*to)
                    )),
                }
            }
        }
        s
    }

    pub fn epsilon_closure(&self, states: &BTreeSet<usize>) -> BTreeSet<usize> {
        let mut closure = states.clone();
        let mut stack: Vec<usize> = states.iter().copied().collect();
        while let Some(s) = stack.pop() {
            for (symbol, target) in &self.transitions[s] {
                if symbol.is_none() && closure.insert(*target) {
                    stack.push(*target);
                }
            }
        }
        closure
    }

    pub fn collect_alphabet(&self) -> Vec<char> {
        let mut set = BTreeSet::new();
        for t in &self.transitions {
            for (sym, _) in t {
                if let Some(c) = sym {
                    set.insert(*c);
                }
            }
        }
        let mut chars: Vec<char> = set.into_iter().collect();
        chars.sort();
        chars
    }
}

struct Fragment {
    start: usize,
    accept: usize,
}

struct NFABuilder {
    transitions: Vec<Vec<(Option<char>, usize)>>,
    next_state: usize,
}

impl NFABuilder {
    fn new_state(&mut self) -> usize {
        let state = self.next_state;
        self.next_state += 1;
        self.transitions.push(Vec::new());
        state
    }

    fn add_transition(&mut self, from: usize, symbol: Option<char>, to: usize) {
        self.transitions[from].push((symbol, to));
    }

    fn build_fragment(&mut self, re: &Regex) -> Fragment {
        match re {
            Regex::EmptySet => {
                let start = self.new_state();
                let accept = self.new_state();
                Fragment { start, accept }
            }
            Regex::EmptyStr => {
                let start = self.new_state();
                let accept = self.new_state();
                self.add_transition(start, None, accept);
                Fragment { start, accept }
            }
            Regex::Char(c) => {
                let start = self.new_state();
                let accept = self.new_state();
                self.add_transition(start, Some(*c), accept);
                Fragment { start, accept }
            }
            Regex::Union(l, r) => {
                let f1 = self.build_fragment(l);
                let f2 = self.build_fragment(r);
                let start = self.new_state();
                let accept = self.new_state();
                self.add_transition(start, None, f1.start);
                self.add_transition(start, None, f2.start);
                self.add_transition(f1.accept, None, accept);
                self.add_transition(f2.accept, None, accept);
                Fragment { start, accept }
            }
            Regex::Concat(l, r) => {
                let f1 = self.build_fragment(l);
                let f2 = self.build_fragment(r);
                self.add_transition(f1.accept, None, f2.start);
                Fragment {
                    start: f1.start,
                    accept: f2.accept,
                }
            }
            Regex::Star(r) => {
                let f = self.build_fragment(r);
                let start = self.new_state();
                let accept = self.new_state();
                self.add_transition(start, None, f.start);
                self.add_transition(start, None, accept);
                self.add_transition(f.accept, None, f.start);
                self.add_transition(f.accept, None, accept);
                Fragment { start, accept }
            }
        }
    }
}
