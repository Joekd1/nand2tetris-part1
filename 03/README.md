## Chapter 3 : Memory

### Reading notes:
- Our computer needs to store values and "remember" them later.
- This can be done using a new set of *memory chips*
- In order to implement this functionality, it is necessary to model time.
- Our model of time will be discrete. We ignore continuous change and care about the state of the world only during successive cycles
- The discrete model of time helps in ignoring the randomness of communication and synchronizing the operation of multiple chips.
- These cycles are achieved with the help of a *clock* and lower level Data-flip flop (DFF) chip.
- The duration of the clock cycle should be slightly longer than the most time consuming operation.
- The clock signal, realized using a binary signal, is broadcasted to every memory chip.
- Adding the dimension of time transforms our *classical logic* into *sequential logic*.
- The family of memeory chips will be implemented gradually: DFF, Bit chip, Register chip, and successive RAM implementation of different sizes.

### Bit :

The Bit chip is expected to remember and emit its state over time. This can be implemented using a DFF and a multiplexer, with the *load* bit of the multiplexer being fed to the *sel* bit of the chip. This has the effect of either outputting the *in* value or the *currentVal* fed previously by the DFF, depending on the *load* bit. 
