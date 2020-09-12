
// 62x32 grid 
struct GameOfLife {
    width: usize,
    height: usize,
    grid: Box<[u64]>
}

impl GameOfLife {
    fn new(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            width,
            height,
            grid: vec![0;((width+61)/62)*height].into()
        }
    }

    fn extend_edges(&mut self) {
        const MASK:u64 = 0x8000_0000_0000_0001;
        if self.width <= 62 {
            let mask = MASK| (!((!0u64) >> (63- ((self.width%62)))));
            for f in self.grid.iter_mut() {
                *f ^= (*f) & mask; 
            }
            return;
        }

        let (first, right) = self.grid.split_at_mut(self.height); // First Column
        for (f,r) in first.iter_mut().zip(right.iter()) {
            *f ^= (((r << 62)) ^ *f) & MASK; 
        }

        let mut start = 0;
        let stop = self.grid.len() - 2*self.height;
        while start < stop {// Middle Columns
            let blk = &mut self.grid[start..];
            start += self.height;
            let (left, remaining) = blk.split_at_mut(self.height);
            let (middle, right) = remaining.split_at_mut(self.height);

            for ((l,m),r) in left.iter().zip(middle.iter_mut()).zip(right.iter()) {
                *m ^= (((l >> 62) | (r << 62)) ^ *m) & MASK; 
            }
        }
        { // Last Columns
            let mask = MASK | (!((!0u64) >> (63- ((self.width%62))))); // Truncate extra width
            let (left, end) = self.grid[start..].split_at_mut(self.height);
            for (l,e) in left.iter().zip(end.iter_mut()) {
                *e ^= (((l >> 62) &MASK) ^ *e) & mask; 
            }
        }
    }

    fn tick(&mut self) {
        self.extend_edges();

        // Computes the next state of the mid row 
        fn compute_next(top:u64, mid: u64, bot: u64) -> u64 {
            let a2 = top ^ mid ^ bot;
            let b2 = (top & mid & bot) | ((top | mid | bot) & !a2);
            let a1 = a2<<1;
            let a3 = a2>>1;
            let b1 = b2<<1;
            let b3 = b2>>1;

            let a2 = top ^ bot;
            let b2 = (top | bot) & !a2;

            let x3 = (!(b1 | b2 | b3) & (a1 & a2 & a3)) |
            (((b1 ^ b2 ^ b3) & !(b1 & b2 &b3)) & ((a1 ^ a2 ^ a3) & !(a1 & a2 &a3))) ;

            let x2=( !(b1 | b2 | b3) & (!(a1 ^ a2 ^ a3) & (a1 | a2 |a3))) |
            (((b1 ^ b2 ^ b3) & !(b1 & b2 &b3)) & !(a1 | a2 | a3));

            return x3 | (mid & x2);
        }

        for column in self.grid.chunks_mut(self.height) {
            let mut top = 0;
            for i in 0..column.len()-1 { // Sadly as of RUST 1.46, the index checks
                let tmp = column[i];     // are not removed, so performance suffers 
                                         // 20% on my system on my system.
                column[i] =  compute_next(top, tmp, column[i+1]); 
                top = tmp;
            }
            let len = column.len();
            column[len-1] = compute_next(top, column[ column.len()-1],  0); 
        }
    }

    fn is_alive(&self, x: usize, y: usize) -> bool {
        ((self.grid[((x /62)* self.height + y)] >> ((x %62 + 1))) & 0b1) == 1
    }
}

impl GameOfLife {
    fn print(&self) {
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
    let mut game = GameOfLife::new(32,32);
    game.grid[4] = 0b001110001110000;
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
    for _ in 0..100 {
        game.print();
        game.tick();
    }


    // BENCHMARK
    // let mut game = GameOfLife::new(10000,10000);
    // for _ in 0..100 {
    //     game.tick();
    // }
}
