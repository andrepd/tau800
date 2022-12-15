mov #00 %3000
mov #00 %3100
:loop
inc %3000
bne -2
inc %3100
jmp loop
