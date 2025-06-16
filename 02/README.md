## Chapter 2 : Boolean Arithmetic

### Reading notes:
- This chapter focuses on building an Arithmetic Logic Unit (ALU)
- The ALU will be used later to build the CPU
- We start from the logic gates built on chapter 1
- We build a half-adder, full-adder, an adder and finally the ALU
- Implementing addition and using the Two Complement method for signed integers is enough to represent all arithmetic operations.
- Notes: for simplification we only use integers (no floating-point arithmetic) and we ignore integer overflow.

### Half-adder :

The half-adder computes the sum of two 1-bit inputs, producing a *sum* and a *carry*.  

From the truth table we can see that the sum is logically equivalent to an Xor gate while the carry is logically equivalent to an AND gate.

