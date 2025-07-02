	// This file is part of www.nand2tetris.org
	// and the book "The Elements of Computing Systems"
	// by Nisan and Schocken, MIT Press.
	// File name: projects/4/Fill.asm

	// Runs an infinite loop that listens to the keyboard input.
	// When a key is pressed (any key), the program blackens the screen,
	// i.e. writes "black" in every pixel. When no key is pressed,
	// the screen should be cleared.
	(LOOP)
	// initialize count
	@count
	M=0

	// Check if keyboard is pressed
	@KBD
	D=M
	@FILL
	D;JNE
	@EMPTY
	D;JEQ

	(FILL)
	// If count == 8192 jump back to the loop
	@count
	D=M
	@8192
	D=D-A
	@LOOP
	D;JEQ

	// Darken screen at RAM[SCREEN + COUNT]
	@SCREEN
	D=A
	@count
	A=D+M
	M=-1

	// Increment count
	@count
	M=M+1

	// Loop back
	@FILL
	0;JMP

	(EMPTY)
	// If count == 8192 jump back to the loop
	@count
	D=M
	@8192
	D=D-A
	@LOOP
	D;JEQ

	// darken screen at RAM[SCREEN + COUNT]
	@SCREEN
	D=A
	@count
	A=D+M
	M=0

	// Increment count
	@count
	M=M+1

	// Loop back
	@EMPTY
	0;JMP
