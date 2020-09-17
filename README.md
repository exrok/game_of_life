# Fast bit-twiddling, Conway's Game of Life 


A fast implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in rust, optimized for x86_64. Designed for interactive use, of large grids.  The performance features in this implementation include:
* Compressed state representation: Each 8 byte cell cluster stores the state of 62 cells.
* Vectorized update function: Using bit-twiddling, 62 cells are update simultaneously.
* Single pass: The majority of the update function runs a single pass across the memory.
* Linear access: The update function preforms linearly accesses across a single continuous buffer. 
* In place update: The update functions works in-place and uses no auxiliary memory. 
* Branchless logic: Leading to extremely predictable branches; 
* Simple: ~100 source lines of code (SLOC).


## Benchmarks
All benchmarks are run with a single thread. 
    
| Cpu                                         |  1000x1000 Grid   | 10000x10000 Grid |
| ------------------------------------------- | ----------------- | ---------------- |
| Intel i5-6300U (2.9GHz)                     | ~0.03ms/iteration | ~4.5ms/iteration |
| Intel i5-6300U (2.9GHz, -target-cpu=native) | ~0.03ms/iteration | ~3.2ms/iteration |

The current benchmark initializes the grid with an pseudo-random initial state and then performs 100 iterations in a loop. 

For context, a more typical implementation can be found [here](https://github.com/bbli/fast_game_of_life/tree/8bcbaf6b737d3862ac6abe35e534f1007ef9827f) and, even with multi-threading, takes over 1000ms/iteration on
the 10000x10000 grid. Algorthims like [hashlife](https://en.wikipedia.org/wiki/Hashlife), can be faster if you want skip a large number of iterations or when sparse or repeated patterns are present. However, hashlife speed depends heavily on the input and is slower if you iterate one step at a time. Moreover, Hashlife is also significantly more complex to implement. For example, it needs a garbage collector to remove unused nodes from the cache system.

This implementation preforms well indeed, below is a `perf stat` of running 100 iterations on a 10000x10000 grid using the i5-6300U machine, with `-C target-cpu=native`.

The total amount of cells updates that occur is 10,000x10,000x100=10,000,000,000. Thus the average number of instructions to update a single cell was 1,902,534,023/10,000,000,000 = **0.190 instructions/cell**... hence the vectorization is working well. 
```sh
> perf stat target/release/game_of_life
== Benchmarking 10000x10000 with 100 iterations ==
end state parity:CF29537D34FF8F1C
== Benchmark complete, avg tick: 3.15995015 ms ==

 Performance counter stats for 'target/release/game_of_life':

            327.65 msec task-clock:u              #    0.999 CPUs utilized          
                 0      context-switches:u        #    0.000 K/sec                  
                 0      cpu-migrations:u          #    0.000 K/sec                  
             3,242      page-faults:u             #    0.010 M/sec                  
       954,302,109      cycles:u                  #    2.913 GHz                    
     1,902,534,023      instructions:u            #    1.99  insn per cycle         
        55,864,257      branches:u                #  170.500 M/sec                  
            48,373      branch-misses:u           #    0.09% of all branches        

       0.328127864 seconds time elapsed

       0.323334000 seconds user
       0.003327000 seconds sys
```

## Todo
* Add Tests.
* Convert into libary.
* More benchmarks.

## Picture
<p align="center">
<img
  src="https://raw.githubusercontent.com/exrok/game_of_life/master/media/example.gif"
  alt="Game of life simulation."
  width=200
/>
</p>
