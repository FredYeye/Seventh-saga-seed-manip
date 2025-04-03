#[derive(Default)]
pub struct Elnard {
    state: u16,
    lists: [Vec<u8>; 2],
    pub current_values: Vec<u8>,
    pub current_matches: u16,
    pub upcoming_values: Option<Vec<Vec<u8>>>
}

impl Elnard {
    fn update(&mut self) {
        self.state = (self.state | 1).wrapping_mul(0x383);
    }

    pub fn populate_list(&mut self) {
        let starting_states = [1, 5];

        for x in 0 .. 2 {
            self.state = starting_states[x];

            for _ in 0 .. 256 {
                self.update();

                self.lists[x].push(self.state.to_le_bytes()[1] & 0b11);
    
    
                if self.state == starting_states[x] {
                    break;
                }
            }

            self.lists[x].extend_from_within(0 .. 128);
        }
    }

    pub fn find_possible_matches(&mut self) {
        let mut indices = [Vec::new(), Vec::new()];

        for y in 0 .. 2 {
            'outer: for x in 0 .. 256 {
                let mut subslice_offset = 0;
    
                for subslice in self.current_values.split(|&n| n == 4) {
                    if self.lists[y][x + subslice_offset .. x + subslice_offset + subslice.len()] != *subslice {
                        continue 'outer;
                    }
    
                    subslice_offset += subslice.len() + 1; // + 1 to account for delimiter
                }
    
                indices[y].push(x);
            }
        }

        let maybe_idx = {
            if indices[0].len() == 1 && indices[1].is_empty() {
                Some(0)
            } else if indices[0].is_empty() && indices[1].len() == 1 {
                Some(1)
            } else {
                None
            }
        };

        if let Some(idx) = maybe_idx { // only one match
            let offset = indices[idx][0] + self.current_values.len();

            let lists: Vec<&[u8]> = self.lists[idx][offset .. offset + 26].split_inclusive(|&x| x == 3).collect();

            let mut lists_vec = Vec::new();
            for list in lists {
                lists_vec.push(list.to_vec());
            }
            lists_vec.pop();

            self.current_matches = (indices[0].len() + indices[1].len()) as u16;
            self.upcoming_values = Some(lists_vec);
        } else if indices[0].is_empty() && indices[1].is_empty() { // invalid input sequence. drop last input (silently atm, unfortunately)
            self.current_values.pop();
        } else {
            self.current_matches = (indices[0].len() + indices[1].len()) as u16;
            self.upcoming_values = None;
        }
    }
}
