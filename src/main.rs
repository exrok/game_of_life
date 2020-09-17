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
                let split_add = |a,b,c| (a^b^c, a&b | a&c | b&c);
                let (a, b)   = split_add(above, row, below);
                let (ax, ay) = split_add(a << 1, above ^ below, a >> 1);
                let (bx, by) = split_add(b << 1, above & below, b >> 1);
                return (ax | row) & (ay ^ bx) & !by;
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
        let tail_mask = edge_mask | !(!0u64 >> (CLUSTER_LEN - (self.width-1) % CLUSTER_LEN));  
        let mut columns = self.grid.chunks_exact_mut(self.height);
        let mut prev = columns.next().unwrap(); 

        // Storing the next and prev cell of the adjacent clusters of each column into
        // the temporary cells in each cluster. And progress to next state w/ tick_column
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
        print!("┌");
        for _ in 0..self.width {
            print!("─");
        }
        print!("┐\n");
        for y in 0..self.height {
            print!("│");
            for x in 0..self.width {
                print!("{}", if self.is_alive(x,y) {
                    "█" 
                } else {" "})
            }
            print!("│");
            print!("\n");
        }
        print!("└");
        for _ in 0..self.width {
            print!("─");
        }
        print!("┘\n");
    }
}

fn bench(width:usize,height:usize) {
    let mut game = GameOfLife::new(width,height);
    let mut rng = oorandom::Rand64::new(0xdeadbeaf);
    for cluster in game.grid.iter_mut() {
        *cluster = rng.rand_u64();
    }
    let steps = 100;
    eprintln!("== Benchmarking {}x{} with {} iterations ==", width, height, steps);
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
    let mut game = GameOfLife::new(80,20);
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

    game.grid[0] |= 0b01000000000000000000000000000000100000000000000000000000000; //glider
    game.grid[1] |= 0b01010000000000000000000000000000100000000000000000000000000;
    game.grid[2] |= 0b01100000000000000000000000000000100000000000000000000000000;

    for _ in 0..100 {
        game.print();
        game.tick();
        std::thread::sleep(std::time::Duration::from_millis(66));
    }
}

fn main() {
    bench(100,100);
    bench(1000,1000);
    bench(10000,10000);

   //example();
}
