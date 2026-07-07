use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::nfa::NFA;

#[derive(Debug, Clone)]
pub struct DFA {
    pub start: usize,
    pub transitions: Vec<BTreeMap<char, usize>>,
    pub accept_states: BTreeSet<usize>,
    pub state_count: usize,
    pub alphabet: Vec<char>,
}

impl DFA {
    pub fn from_nfa(nfa: &NFA, alphabet: &[char]) -> Self {
        if alphabet.is_empty() {
            let start_accepts = nfa
                .epsilon_closure(&BTreeSet::from([nfa.start]))
                .contains(&nfa.accept);
            let accept_states = if start_accepts {
                BTreeSet::from([0])
            } else {
                BTreeSet::new()
            };
            return DFA {
                start: 0,
                transitions: vec![BTreeMap::new()],
                accept_states,
                state_count: 1,
                alphabet: vec![],
            };
        }

        let start_set = nfa.epsilon_closure(&BTreeSet::from([nfa.start]));

        let mut dfa_transitions: Vec<BTreeMap<char, usize>> = Vec::new();
        let mut dfa_accept: BTreeSet<usize> = BTreeSet::new();
        let mut set_to_id: BTreeMap<BTreeSet<usize>, usize> = BTreeMap::new();
        let mut queue: VecDeque<BTreeSet<usize>> = VecDeque::new();

        let start_id = 0;
        set_to_id.insert(start_set.clone(), start_id);
        dfa_transitions.push(BTreeMap::new());
        if start_set.contains(&nfa.accept) {
            dfa_accept.insert(start_id);
        }
        queue.push_back(start_set);

        while let Some(current_set) = queue.pop_front() {
            let from_id = set_to_id[&current_set];

            for &c in alphabet {
                let mut move_set = BTreeSet::new();
                for &s in &current_set {
                    for (symbol, target) in &nfa.transitions[s] {
                        if *symbol == Some(c) {
                            move_set.insert(*target);
                        }
                    }
                }
                let target_set = nfa.epsilon_closure(&move_set);

                if target_set.is_empty() {
                    continue;
                }

                let to_id = if let Some(&id) = set_to_id.get(&target_set) {
                    id
                } else {
                    let id = dfa_transitions.len();
                    set_to_id.insert(target_set.clone(), id);
                    dfa_transitions.push(BTreeMap::new());
                    if target_set.contains(&nfa.accept) {
                        dfa_accept.insert(id);
                    }
                    queue.push_back(target_set);
                    id
                };

                dfa_transitions[from_id].insert(c, to_id);
            }
        }

        let sink = dfa_transitions.len();
        let needs_sink = dfa_transitions.iter().any(|t| t.len() < alphabet.len());

        if needs_sink {
            dfa_transitions.push(BTreeMap::new());
            for &c in alphabet {
                dfa_transitions[sink].insert(c, sink);
            }
            for t in &mut dfa_transitions {
                for &c in alphabet {
                    t.entry(c).or_insert(sink);
                }
            }
        }

        let state_count = dfa_transitions.len();

        DFA {
            start: start_id,
            transitions: dfa_transitions,
            accept_states: dfa_accept,
            state_count,
            alphabet: alphabet.to_vec(),
        }
    }

    pub fn dump(&self) {
        let mut acc: Vec<_> = self.accept_states.iter().copied().collect();
        acc.sort();
        print!(
            "DFA: {} 个状态, 开始: q{}, 接受: {{",
            self.state_count, self.start
        );
        for (i, s) in acc.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("q{}", s);
        }
        println!("}}");
        for s in 0..self.state_count {
            for (&c, &to) in &self.transitions[s] {
                println!("  q{} --{}--> q{}", s, c, to);
            }
        }
    }

    pub fn display_string(&self) -> String {
        let mut s = String::new();
        let mut acc: Vec<_> = self.accept_states.iter().copied().collect();
        acc.sort();
        s.push_str(&format!(
            "DFA: {} 个状态, 开始: q{}, 接受: {{",
            self.state_count, self.start
        ));
        for (i, st) in acc.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&format!("q{}", st));
        }
        s.push('}');
        for st in 0..self.state_count {
            for (&c, &to) in &self.transitions[st] {
                s.push_str(&format!("\n  q{} --{}--> q{}", st, c, to));
            }
        }
        s
    }

    pub fn minimize(&self) -> DFA {
        let accepting: BTreeSet<usize> = self.accept_states.clone();
        let non_accepting: BTreeSet<usize> = (0..self.state_count)
            .filter(|s| !accepting.contains(s))
            .collect();

        let mut blocks: Vec<BTreeSet<usize>> = Vec::new();
        if !non_accepting.is_empty() {
            blocks.push(non_accepting);
        }
        if !accepting.is_empty() {
            blocks.push(accepting);
        }

        if blocks.is_empty() {
            return DFA {
                start: 0,
                transitions: vec![BTreeMap::new()],
                accept_states: BTreeSet::new(),
                state_count: 1,
                alphabet: self.alphabet.clone(),
            };
        }

        let mut state_block: Vec<usize> = vec![0; self.state_count];
        for (block_id, block) in blocks.iter().enumerate() {
            for &s in block {
                state_block[s] = block_id;
            }
        }

        let mut splitters: VecDeque<(usize, char)> = VecDeque::new();
        for block_id in 0..blocks.len() {
            for &c in &self.alphabet {
                splitters.push_back((block_id, c));
            }
        }

        while let Some((splitter_block, c)) = splitters.pop_front() {
            if splitter_block >= blocks.len() || blocks[splitter_block].is_empty() {
                continue;
            }

            let splitter_states = blocks[splitter_block].clone();

            let mut affected: BTreeMap<usize, (BTreeSet<usize>, BTreeSet<usize>)> = BTreeMap::new();

            for (state, &blk) in state_block.iter().enumerate() {
                if blk >= blocks.len() || blocks[blk].is_empty() {
                    continue;
                }
                let target = self.transitions[state].get(&c).copied();
                let in_splitter = target.is_some_and(|t| splitter_states.contains(&t));

                let entry = affected.entry(blk).or_default();
                if in_splitter {
                    entry.0.insert(state);
                } else {
                    entry.1.insert(state);
                }
            }

            for (blk, (inside, outside)) in affected {
                if inside.is_empty() || outside.is_empty() {
                    continue;
                }

                let new_blk = blocks.len();
                let (smaller, larger) = if inside.len() <= outside.len() {
                    for &s in &inside {
                        state_block[s] = new_blk;
                    }
                    blocks[blk] = outside;
                    (inside, blocks[blk].clone())
                } else {
                    for &s in &outside {
                        state_block[s] = new_blk;
                    }
                    blocks[blk] = inside;
                    (outside, blocks[blk].clone())
                };
                blocks.push(smaller);

                for &c2 in &self.alphabet {
                    splitters.push_back((new_blk, c2));
                }
                let _ = larger;
            }
        }

        let alive_blocks: Vec<usize> = (0..blocks.len())
            .filter(|&b| !blocks[b].is_empty())
            .collect();

        let block_to_new: BTreeMap<usize, usize> = alive_blocks
            .iter()
            .enumerate()
            .map(|(i, &b)| (b, i))
            .collect();

        let new_start = block_to_new[&state_block[self.start]];

        let new_accept: BTreeSet<usize> = alive_blocks
            .iter()
            .filter(|&&b| blocks[b].iter().any(|s| self.accept_states.contains(s)))
            .map(|&b| block_to_new[&b])
            .collect();

        let new_state_count = block_to_new.len();
        let mut new_transitions: Vec<BTreeMap<char, usize>> =
            vec![BTreeMap::new(); new_state_count];

        for (&old_block, &new_id) in &block_to_new {
            let rep = *blocks[old_block].iter().next().unwrap();
            for &c in &self.alphabet {
                let target = self.transitions[rep][&c];
                let target_block = state_block[target];
                let target_new_id = block_to_new[&target_block];
                new_transitions[new_id].insert(c, target_new_id);
            }
        }

        DFA {
            start: new_start,
            transitions: new_transitions,
            accept_states: new_accept,
            state_count: new_state_count,
            alphabet: self.alphabet.clone(),
        }
    }

    pub fn is_equivalent_to(&self, other: &DFA) -> bool {
        let mut alphabet: Vec<char> = self.alphabet.clone();
        for &c in &other.alphabet {
            if !alphabet.contains(&c) {
                alphabet.push(c);
            }
        }
        alphabet.sort();

        let dfa1 = self.complete(&alphabet);
        let dfa2 = other.complete(&alphabet);

        let mut visited = BTreeSet::new();
        let mut stack = VecDeque::new();
        stack.push_back((dfa1.start, dfa2.start));

        while let Some((s1, s2)) = stack.pop_front() {
            if !visited.insert((s1, s2)) {
                continue;
            }

            let acc1 = dfa1.accept_states.contains(&s1);
            let acc2 = dfa2.accept_states.contains(&s2);
            if acc1 != acc2 {
                return false;
            }

            for &c in &alphabet {
                let t1 = dfa1.transitions[s1][&c];
                let t2 = dfa2.transitions[s2][&c];
                stack.push_back((t1, t2));
            }
        }

        true
    }

    fn complete(&self, alphabet: &[char]) -> DFA {
        let needs_sink = self.transitions.iter().any(|t| t.len() < alphabet.len());

        if !needs_sink && self.alphabet == alphabet {
            return self.clone();
        }

        let mut trans: Vec<BTreeMap<char, usize>> = self.transitions.to_vec();
        let sink = trans.len();

        trans.push(BTreeMap::new());
        for &c in alphabet {
            trans[sink].insert(c, sink);
        }

        for t in &mut trans {
            for &c in alphabet {
                t.entry(c).or_insert(sink);
            }
        }

        DFA {
            start: self.start,
            transitions: trans,
            accept_states: self.accept_states.clone(),
            state_count: sink + 1,
            alphabet: alphabet.to_vec(),
        }
    }
}
