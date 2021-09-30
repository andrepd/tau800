; Inputs: %10 %11 %12 %13
; Outputs: %15 ... 2b

; Digito 1
mov #01 a
mov %1300 bl
cal 1703

; Digito 2
mov #02 a
mov %1200 bl
cal 1703

; Digito 3
mov #04 a
mov %1100 bl
cal 1703

; Digito 4
mov #08 a
mov %1000 bl
cal 1703

nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e
nop;e

;; Subroutine ;;
; Inputs:
;   a = current bit mask
;   bl = current digit
; Locals:
;   x = current segment

mov #00 x

; First digit

; If number not 1237, then set segment 1
sec
cmp #01 bl
beq +31
sec
cmp #02 bl
beq +23
sec
cmp #03 bl
beq +15
sec
cmp #07 bl
beq +7
or a %1500,x

clc
add #01 x

; If number not 14, then set segment 2
sec
cmp #01 bl
beq +15
sec
cmp #04 bl
beq +7
or a %1500,x

clc
add #01 x

; If number not 56, then set segment 3
sec
cmp #05 bl
beq +15
sec
cmp #06 bl
beq +7
or a %1500,x

clc
add #01 x

; If number is 0268, then set segment 4 (clever)
sec
cmp #04 bl   ; Se for 4
beq +14
bit #01 bl  ; Ou ímpar
bne +7
or a %1500,x

clc
add #01 x

; If number not 147, then set segment 5
sec
cmp #01 bl
beq +23
sec
cmp #04 bl
beq +15
sec
cmp #07 bl
beq +7
or a %1500,x

clc
add #01 x

; If number not 2, then set segment 6
sec
cmp #02 bl
beq +7
or a %1500,x

clc
add #01 x

; If number not 017, then set segment 7
sec
cmp #00 bl
beq +23
sec
cmp #01 bl
beq +15
sec
cmp #07 bl
beq +7
or a %1500,x

ret

; 0d 20 __ __ 15 __ __