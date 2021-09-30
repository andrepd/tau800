; Calculates sqrt with Newton method
; Input:  6-bit number on %000a
; Output: sqrt of that number on a

mov #32 %000a

;START

; b = input / 2
mov %000a bl
lsr bl     ; Div by 2

;LOOP = START+8

; c = (b + input / b) / 2
mov %000a cl 
div b cl
add b cl
lsr cl

cmp bl cl  ; if c<b goto END
bpl +3
jmp 3d02
mov bl cl  ; else goto LOOP
jmp 1d02

;END = START+40
nop
