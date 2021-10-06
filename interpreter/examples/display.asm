; Inputs: %10 %11 %12 %13
; Outputs: %14 ... 1a

;; Main ;;

mov #03 ch
cal 2b02
mov #00 ch
cal 2b02
jmp 0002

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

;; Subroutine: do one sweep of all digits ;;
; Inputs:
;   ch: direction (0 l→r, 3 r→l)
; Locals:
;   cl: increment
;   x: offset

clc
mov ch cl
add #01 cl  ; Estas 3 linhas fazem 0 → -1 e 3 → +1 
lsr cl
add #3f cl
clc
not cl
add #01 cl

; Digito 1
mov ch x
mov #01 a
mov %1000,x bl
cal 0009
cal 1805

clc
add cl ch 

; Digito 2
mov ch x
mov #02 a
mov %1000,x bl
cal 0009
cal 1805

clc
add cl ch 

; Digito 3
mov ch x
mov #04 a
mov %1000,x bl
cal 0009
cal 1805

clc
add cl ch 

; Digito 4
mov ch x
mov #08 a
mov %1000,x bl
cal 0009
cal 1805

ret

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

;; Subroutine: set one digit ;;
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
or a %1400,x

clc
add #01 x

; If number not 14, then set segment 2
sec
cmp #01 bl
beq +15
sec
cmp #04 bl
beq +7
or a %1400,x

clc
add #01 x

; If number not 56, then set segment 3
sec
cmp #05 bl
beq +15
sec
cmp #06 bl
beq +7
or a %1400,x

clc
add #01 x

; If number is 0268, then set segment 4 (clever)
sec
cmp #04 bl   ; Se for 4
beq +14
bit #01 bl  ; Ou ímpar
bne +7
or a %1400,x

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
or a %1400,x

clc
add #01 x

; If number not 2, then set segment 6
sec
cmp #02 bl
beq +7
or a %1400,x

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
or a %1400,x

ret

nop
nop
nop
nop
nop
nop

;; Subroutine: clear one digit ;;
; Inputs:
;   a = current bit mask
; Locals:
;   x = current segment

sec
not a

mov #06 x
and a %1400,x
sub #01 x
bne -14
and a %1400,x

clc
not a
ret

; 0d 20 __ __ 15 __ __
