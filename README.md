# Fast bit-twiddling, Conway's Game of Life 


An extremely fast implementation [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) implemented in rust, optimized for x86_64.  Performance 
features of the this implementation include:
* Compressed state representation: Each 8 byte cell cluster stores the state of 62 cells.
* Vectorized update function: Using bit-twiddling, 62 cells are update simultaneously.
* Single pass: The majority of the update function runs a single pass across the memory.
* Linear access: The update function preforms linearly accesses across a single continuous buffer. 
* In place update: The update functions works in-place and uses no auxiliary memory. 
* Branchless logic: Leading to extremely predictable branches; 


## Benchmarks
All benchmarks are run with a single thread. For context, a more typical implementation claiming to be fast can be found [here](https://github.com/bbli/fast_game_of_life/tree/8bcbaf6b737d3862ac6abe35e534f1007ef9827f) and, even with multi-threading, takes over 1000ms/iteration on
the 10000x10000 grid.
    
| Cpu       |  1000x1000 Grid  | 10000x10000 Grid |
| --------  | ------------------- | --------------------- |
| Amd FX-8350 (4Ghz)                          | ~0.05ms/iteration      | ~6.4ms/iteration                | 
| Intel i5-6300U (2.4GHz)                     | ~0.05ms/iteration |  ~5.4ms/iteration           |
| Intel i5-6300U (2.4GHz, -target-cpu=native) | ~0.03ms/iteration |  ~3.6ms/iteration            |

Note: The current benchmark initializes the grid with an pseudo-random initial state and then performs 100 iterations in a loop. In a different environment performance would vary but the magnitude of 
the results should be consistent.

This implementation preforms very well indeed, below is a `perf stat` of running 100 iterations on a 10000x10000 random grid from the i5-6300U machine, with `-C target-cpu=native`.

The total amount of cells updates that occur is 10,000x10,000x100=10,000,000,000 the average number of instructions to update a single cell as 2,227,035,775/10,000,000,000 = **0.227 instructions/cell**... hence the vectorization is working quite well. 
```sh
> perf stat target/release/game_of_life
== Benchmarking 10000x10000 with 100 iterations ==
end state parity:CF29537D34FF8F1C
== Benchmark complete, avg tick: 3.57443589 ms ==

 Performance counter stats for 'target/release/game_of_life':

            368.65 msec task-clock:u              #    0.999 CPUs utilized          
             3,241      page-faults:u             #    0.009 M/sec                  
     1,075,910,313      cycles:u                  #    2.919 GHz                    
     2,227,035,775      instructions:u            #    2.07  insn per cycle         
        55,864,166      branches:u                #  151.539 M/sec                  
            57,707      branch-misses:u           #    0.10% of all branches        

       0.369122386 seconds time elapsed

       0.364494000 seconds user
       0.003354000 seconds sys
```
## Example
<p align="center">
<img
  src="https://raw.githubusercontent.com/exrok/game_of_life/master/media/example.gif"
  alt="Game of life simulation."
  width=200
/>
</p>