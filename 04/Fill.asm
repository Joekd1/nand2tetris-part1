// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.
(LOOP)
@i
M=0
@KBD
D=M
@FILL
D;JNE
@EMPTY
D;JEQ

(FILL)
@i
D=M
@8192
D=D-A
@LOOP
D;JEQ
@SCREEN
D=A
@i
D=D+M
A=D
M=-1
@i
M=M+1
@FILL
0;JMP

(EMPTY)
@i
D=M
@8192
D=D-A
@LOOP
D;JEQ
@SCREEN
D=A
@i
D=D+M
A=D
M=0
@i
M=M+1
@EMPTY
0;JMP
