; Calculates sqrt with Newton method
; Input:  6-bit number on %000a
; Output: sqrt of that number on a

mov #32 a
cal 1202
jmp 3d3d

nop
nop
nop
nop
nop
nop
nop

;; Subroutine: calculate sqrt of 6-bit number with Newton's method
; Inputs:
;   a: Number
; Locals:
;   bl, cl
; Outputs:
;   a: √ of input

:start = 1202

; b = input / 2
mov a bl
lsr bl

:loop = start+10 = 1c02

; c = (b + input / b) / 2
mov a cl 
div bl cl
add bl cl
lsr cl

mov cl bl@-04

mov bl a
ret

:end
nop
