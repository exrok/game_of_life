
// Each u64 in the grid represented 62 cells, the lowest and highest bit are used
// to merge the most outer neighbour cell. The merging is preformed in extend edges. 
//
// On the boundary of the grid all cells out side the grid are treat as dead for
// the neighbour counts.
pub struct GameOfLife {
    width: usize,
    height: usize,
    grid: Box<[u64]> 
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            width,
            height,
            grid: vec![0;((width+61)/62)*height].into()
        }
    }

    pub fn tick(&mut self) {
        self.extend_edges();

        // Computes the next state for the 62 cells in row
        fn compute_next(above:u64, row: u64, below: u64) -> u64 {
            let a = above ^ row ^ below;
            let b = (above & row & below) | ((above | row | below) & !a);

            let (a1, a2, a3) = (a << 1, above ^ below, a >> 1);
            let a_xor = a1 ^ a2 ^ a3;
            let a_and = a1 & a2 & a3;
            let a_or  = a1 | a2 | a3;
            
            let (b1, b2, b3) = (b << 1, (above | below) & !a2, b >> 1);
            let b_xor = b1 ^ b2 ^ b3;
            let b_and = b1 & b2 & b3;
            let b_or =  b1 | b2 | b3;

            //live neighbour count masks
            let three = (!b_or & a_and) | (b_xor & !b_and & a_xor & !a_and); 
            let two   = (!b_or & !a_xor & a_or) | (b_xor & !b_and & !a_or);

            return three | (row & two);
        }

        for column in self.grid.chunks_mut(self.height) {
            let mut top = 0;
            for i in 0..column.len()-1 { // Sadly as of RUST 1.46, the bound checks
                let tmp = column[i];     // are not removed, performance suffers a bit
                column[i] = compute_next(top, tmp, column[i+1]); 
                top = tmp;
            }
            if let Some(last) = column.last_mut() {
                *last = compute_next(top, *last,  0)
            }
        }
    }

    // Places the neighbouring cells on edges of columns on the adjacent columns
    fn extend_edges(&mut self) {
        let edge_mask = 0x8000_0000_0000_0001;
        //tail_mask is used to zero extra width in the last column
        let tail_mask = edge_mask | !(!0u64 >> (63 - self.width%62));  
        if self.width <= 62 {
            for f in self.grid.iter_mut() {
                *f ^= (*f) & tail_mask; 
            }
            return;
        }

        let (first, right) = self.grid.split_at_mut(self.height); // First Column
        for (f,r) in first.iter_mut().zip(right.iter()) {
            *f ^= (((r << 62)) ^ *f) & edge_mask; 
        }

        let mut start = 0;
        while start < self.grid.len() - 2*self.height {           // Middle Columns
            let (left, remaining) = self.grid[start..].split_at_mut(self.height);
            let (middle, right) = remaining.split_at_mut(self.height);

            for ((l, m), r) in left.iter().zip(middle.iter_mut()).zip(right.iter()) {
                *m ^= (((l >> 62) | (r << 62)) ^ *m) & edge_mask
            }

            start += self.height;
        }
        
        let (left, end) = self.grid[start..].split_at_mut(self.height); // Last Column
        for (l, e) in left.iter().zip(end.iter_mut()) {
            *e ^= ((l >> 62) ^ *e) & tail_mask;
        }
    }
    fn is_alive(&self, x: usize, y: usize) -> bool {
        ((self.grid[((x /62)* self.height + y)] >> (x % 62 + 1)) & 0b1) == 1
    }
}

impl GameOfLife {
    fn print(&self) {
        //not optmized just for proof of concept
        for _ in 0..self.width+2 {
            print!("_");
        }
        print!("\n");
        for y in 0..self.height {
            print!("|");
            for x in 0..self.width {
                print!("{}", if self.is_alive(x,y) {
                    'X'
                } else {' '})
            }
            print!("|");
            print!("\n");
        }
        for _ in 0..self.width+2 {
            print!("-");
        }
        print!("\n");
    }
}

fn main() {
    // EXAMPLE
    let mut game = GameOfLife::new(83,32);
    game.grid[4] = 0b001110001110000; //xor
    game.grid[5] = 0;
    game.grid[6] = 0b100001010000100;
    game.grid[7] = 0b100001010000100;
    game.grid[8] = 0b100001010000100;
    game.grid[9] = 0b001110001110000;
    game.grid[10] = 0;
    game.grid[11] = 0b001110001110000;
    game.grid[12] = 0b100001010000100;
    game.grid[13] = 0b100001010000100;
    game.grid[14] = 0b100001010000100;
    game.grid[15] = 0;
    game.grid[16] = 0b001110001110000;

    game.grid[4] |= 0b01000000000000000000000000000000000000000000000000000000; //glider
    game.grid[5] |= 0b01010000000000000000000000000000000000000000000000000000;
    game.grid[6] |= 0b01100000000000000000000000000000000000000000000000000000;
    for _ in 0..100 {
        game.print();
        game.tick();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    // // BENCHMARK
    // // Generate random initla state // I removed the crate for random for now
    // for _ in 0..100 {
    //     game.tick();
    // }
}
