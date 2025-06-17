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

### Full-adder:

The addition of two binary integers with more than one digit may result in a carry, requiring us to compute the sum of three binary digits. 
The full-adder computes the sum of three 1-bit inputs, producing a sum and a carry. 
Addition is a binary operation, so in order to compute the sum of a, b and c we proceed this way:
- First compute the sum of a and b using a hal-adder. This produces the sum S1 and carry C1
- We use a second half-adder to compute the sun of S1 and c. This produces S2 and C2
- Note that S2 is the rightmost digit of the entire sum so it is the *sum* of the full-adder
- The leftmost digit C is 1 only if either C1 or C2 is 1. C1 and C2 cannot be both 1.
- C can be obtained by an Or gate
- Therefore the full-adder can be implemented with two half-adders and one Or gate.

More details about the entire process can be found in *Discrete Mathematics with Applications* by *Susanna Epp* (5th ed) pages 97 - 99

