use std::collections::{HashMap, HashSet};

pub struct NFA {
    transitions: HashMap<(u32, Option<char>), HashSet<u32>>,
    start_state: u32,
    accept_states: HashSet<u32>,
    current_state: u32,
}

impl NFA {
    // Create a new NFA
    pub(crate) fn new() -> Self {
        NFA {
            transitions: HashMap::new(),
            start_state: 0,
            accept_states: HashSet::new(),
            current_state: 1,
        }
    }

    // Simulate the NFA to see if it accepts the given input
    pub(crate) fn is_accepted(&self, input: &str) -> bool {
        let mut current_states: HashSet<u32> = HashSet::new();
        current_states.insert(self.start_state);

        for symbol in input.chars() {
            let mut next_states: HashSet<u32> = HashSet::new();
            for &state in &current_states {
                if let Some(transitions) = self.transitions.get(&(state, Option::from(symbol))) {
                    next_states.extend(transitions);
                }
            }
            current_states = next_states;
        }

        current_states.iter().any(|state| self.accept_states.contains(state))
    }

     pub(crate) fn add_transition(&mut self, from_state: u32, symbol: Option<char>, to_state: u32) {
        self.transitions
            .entry((from_state, symbol))
            .or_insert_with(HashSet::new)
            .insert(to_state);
    }

    pub(crate)  fn add_accept_state(&mut self, state: u32) {
        self.accept_states.insert(state);
    }


    // Create a simple NFA for a single character
    fn from_char(&mut self, c: char) -> (u32, u32) {
        let start = self.current_state;
        let accept = self.current_state + 1;
        self.current_state += 2;

        self.add_transition(start, Some(c), accept);

        (start, accept)
    }

    // Create an NFA for concatenation
    fn concatenate(&mut self, nfa1: (u32, u32), nfa2: (u32, u32)) -> (u32, u32) {
        self.add_transition(nfa1.1, None, nfa2.0);
        (nfa1.0, nfa2.1)
    }

    // Create an NFA for union (a|b)
    fn union(&mut self, nfa1: (u32, u32), nfa2: (u32, u32)) -> (u32, u32) {
        let start = self.current_state;
        let accept = self.current_state + 1;
        self.current_state += 2;

        self.add_transition(start, None, nfa1.0);
        self.add_transition(start, None, nfa2.0);
        self.add_transition(nfa1.1, None, accept);
        self.add_transition(nfa2.1, None, accept);

        (start, accept)
    }

    // Create an NFA for Kleene Star (a*)
    fn kleene_star(&mut self, nfa: (u32, u32)) -> (u32, u32) {
        let start = self.current_state;
        let accept = self.current_state + 1;
        self.current_state += 2;

        self.add_transition(start, None, nfa.0);
        self.add_transition(start, None, accept);
        self.add_transition(nfa.1, None, nfa.0);
        self.add_transition(nfa.1, None, accept);

        (start, accept)
    }

    // Convert a regex string into an NFA using Thompson's construction
    fn from_regex(&mut self, regex: &str) -> (u32, u32) {
        let mut nfa_stack: Vec<(u32, u32)> = Vec::new();

        let mut i = 0;
        while i < regex.len() {
            let c = regex.chars().nth(i).unwrap();
            match c {
                '*' => {
                    let nfa = nfa_stack.pop().expect("Stack underflow for Kleene star");
                    nfa_stack.push(self.kleene_star(nfa));
                }
                '|' => {
                    let nfa2 = nfa_stack.pop().expect("Stack underflow for union");
                    let nfa1 = nfa_stack.pop().expect("Stack underflow for union");
                    nfa_stack.push(self.union(nfa1, nfa2));
                }
                _ => {
                    let mut nfa = self.from_char(c);
                    if i + 1 < regex.len() && regex.chars().nth(i + 1).unwrap() != '*' {
                        while i + 1 < regex.len()
                            && regex.chars().nth(i + 1).unwrap() != '|'
                            && regex.chars().nth(i + 1).unwrap() != '*'
                        {
                            i += 1;
                            let next_c = regex.chars().nth(i).unwrap();
                            let next_nfa = self.from_char(next_c);
                            nfa = self.concatenate(nfa, next_nfa);
                        }
                    }
                    nfa_stack.push(nfa);
                }
            }
            i += 1;
        }

        nfa_stack.pop().expect("Invalid regex")
    }

}