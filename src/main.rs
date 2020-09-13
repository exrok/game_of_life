// On the boundary of the grid all cells out side the grid are treat as dead for
// the neighbour counts.
pub struct GameOfLife {
    width: usize,
    height: usize,
    grid: Box<[CellCluster]> 
}

// Each cell cluster stores the state 62 cells. The most and least signficant
// bits are used as temporaries to store a copy of the next and prev cell in
// the row of the adjacent clusters. These temporaries are called the edges,  
// and are undefined outside of the tick functions.  
type CellCluster = u64;
const CLUSTER_LEN: usize = 62;

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            width,
            height,
            grid: vec![0;((width+(CLUSTER_LEN - 1))/CLUSTER_LEN)*height].into()
        }
    }
    
    pub fn tick(&mut self) {
        fn tick_column(column: &mut [CellCluster]) {
            // Computes the next state for the CLUSTER_LEN cells in row
            fn compute_next(above: CellCluster, row: CellCluster, below: CellCluster) -> u64 {
                let a = above ^ row ^ below; // parity in a column
                let b = (above & row & below) | ((above | row | below) & !a); //three_or_two in a column

                let (a1, a2, a3) = (a << 1, above ^ below, a >> 1);
                let a_odd  = a1 ^ a2 ^ a3;
                let a_all  = a1 & a2 & a3;
                let a_some = a1 | a2 | a3;
                
                let (b1, b2, b3) = (b << 1, above & below, b >> 1);
                let b_odd  = b1 ^ b2 ^ b3;
                let b_all  = b1 & b2 & b3;
                let b_some = b1 | b2 | b3;

                // Logic Example: if !b_some[1], all three columns of neighbours for cell [1], have 
                //  either 1 or 0 live cells. Further, if a_all[1] each column has an odd number of 
                //  neighbours. Thus we can conclude that for each of the 3 columns there exactly 
                //  one live neighbour, hence cell [1] has 3 live neighbours. This is computed the 
                //  by the mask below, (a_all & !b_some).

                //live neighbour count masks
                let three = (a_all & !b_some) | (a_odd & b_odd & !a_all & !b_all); 
                let two   = (!a_odd & a_some & !b_some) | ( b_odd & !b_all & !a_some);

                return three | (row & two);
            }

            let mut clusters = column.iter_mut();
            let mut above = 0;
            let mut curr = clusters.next().unwrap(); //chunks_mut returns non-empty slices

            for below in clusters {
                *curr = compute_next(above, {above = *curr; *curr}, *below); 
                curr = below;
            }
            *curr = compute_next(above, *curr, 0);
        }

        let edge_mask = 0x8000_0000_0000_0001;
        //tail_mask is used to zero extra width in the last rowsumn
        let tail_mask = edge_mask | !(!0u64 >> ((CLUSTER_LEN+1) - self.width % CLUSTER_LEN));  

        let mut columns = self.grid.chunks_exact_mut(self.height);
        let mut prev = columns.next().unwrap(); 

        // Stores the next and prev cell of the adjacent clusters of each column into
        //  the temporary cells in each cluster. Further, the horizontal boundary 
        //  behaviour is provided by zeroing the first and last cells of each row. 
        //
        //  TAAA...AAAT, TBBB...BBBT, TCCC...CCCT
        // =>
        //  0AAA...AAAB, ABBB...BBBC, BCCC...CCC0
        //  
        if let Some(mut curr) = columns.next() {
            for (first, second) in prev.iter_mut().zip(curr.iter()) {
                *first ^= ((second << CLUSTER_LEN) ^ *first) & edge_mask; 
            }

            for next in columns {
                for ((left, mid), right) in prev.iter().zip(curr.iter_mut()).zip(next.iter()) {
                    *mid ^= (((left >> CLUSTER_LEN) | (right << CLUSTER_LEN)) ^ *mid) & edge_mask
                }
                tick_column(prev);
                prev = curr;
                curr = next;
            }

            for (left, last) in prev.iter().zip(curr.iter_mut()) {
                *last ^= ((left >> CLUSTER_LEN) ^ *last) & tail_mask; 
            }
            tick_column(curr);
        } else {
            for f in prev.iter_mut() { //Update bounds on the single column
                *f &= !tail_mask; 
            }
        }
        tick_column(prev);
    }

    #[inline]
    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        let index = (x / CLUSTER_LEN)* self.height + y;
        let offset = (x % CLUSTER_LEN) + 1;
        ((self.grid[index] >> offset) & 0b1) == 1
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

fn bench(size:usize) {
    let mut game = GameOfLife::new(size,size);
    let mut rng = oorandom::Rand64::new(0xdeadbeaf);
    for cluster in game.grid.iter_mut() {
        *cluster = rng.rand_u64();
    }
    let steps = 100;
    eprintln!("== Benchmarking {}x{} with {} steps ==", size, size, steps);
    let start_time = std::time::Instant::now();
    for _ in 0..steps {
        game.tick();
    }
    let elapsed = start_time.elapsed();
    let parity = game.grid.iter().fold(0u64,|x,&y| y ^ x );
    eprintln!("end state parity:{:X}",parity);
    eprintln!("== Benchmark complete, avg tick: {} ms ==\n", (elapsed.as_secs_f64()*1000.0)/(steps as f64));
}

fn example() {
    // EXAMPLE
    let mut game = GameOfLife::new(89,32);
    game.grid[4] = 0b001110001110000; //star thing
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
}

fn main() {
    bench(100);
    bench(1000);
    bench(10000);
   // example();
}
