;; Main: sieve primes up to 255 ;;

:start

	mov #04 a
	cal init
	mov #04 a
	cal sieve
	cal output
	hcf



;; Subroutine: initialise sieve of Erathosthenes at 0020 ;;
; Inputs:
;   a = 64×size
; Locals:
;   x = counter

:init

	mov #00 x

	:loop_init
	mov #01 %0020,x  ; (1)
	inc x
	bne -3
	dec a
	beq +2
	inc %1a02  ; high word of addr in (1)
	jmp loop_init

	mov #20 %1a02
	ret



;; Subroutine: sieve ;;
; Inputs:
;   a = size / 64
; Locals:
;   %0001 = outer pointer
;   %0201 = inner pointer
;   x = value at outer pointer

:sieve

	clc
	add #20 a
	
	mov #02 x          ; x ← 2

	mov #00 %0001      ; outer pointer ← %0020
	mov #20 %0101

	; Loop through primes
	:sieve_outer
	mov %0001 %0201    ; inner pointer ← outer pointer
	mov %0101 %0301

	; Loop through multiples of this prime
	:sieve_inner
	clc
	add x %0201        ; Increment inner pointer (with carry) by x
	add #00 %0301
	sec
	cmp a %0301        ; If we reached the end of the table, exit inner loop
	beq +2
	mov #00 (%0201)    ; Otherwise, zero memory at inner pointer and repeat
	jmp sieve_inner

	; Find next prime
	:sieve_loop
	inc x              ; Increment outer pointer and x
	inc %0001
	bne +3             ; If outer pointer ≥ %0021, return
    sec
	sub #20 a
	ret
	bit #3f (%0001)    ; If not a prime, keep incrementing until we reach the next prime
    bne +1
    jmp sieve_loop

    jmp sieve_outer    ; Loop



;; Subroutine: output ;;
; Inputs:
;   a = 64×size (unused)
; Locals:
;   bh,bl = number
;   %0001 = pointer

:output
	
	mov #00 %0001
	mov #20 %0101

	:output_loop
	bit #3f (%0001)  ; If not a prime, continue
	bne +1
	jmp output_loop_tail

	mov #08 a        ; Clear digits
	cal clear
	mov #04 a
	cal clear
	mov #02 a
	cal clear
	mov #01 a
	cal clear

	mov %0001 bl     ; Write digits
	mov #00 bh ;;TODO
	clc
	add #02 bl
	add #00 bh

	cal num2dec      ; Digit 4
	mov bl cl
	mov a bl
	mov #08 a
	cal digit
	mov cl bl

	mov bl x         ; If rest of digits are 0 continue
	or bh x
	bne +1
	jmp output_loop_tail

	cal num2dec      ; Digit 3
	mov bl cl
	mov a bl
	mov #04 a
	cal digit
	mov cl bl

	mov bl x         ; If rest of digits are 0 continue
	or bh x
	bne +1
	jmp output_loop_tail

	cal num2dec      ; Digit 2
	mov bl cl
	mov a bl
	mov #02 a
	cal digit
	mov cl bl

	mov bl x         ; If rest of digits are 0 continue
	or bh x
	bne +1
	jmp output_loop_tail

	cal num2dec      ; Digit 1
	mov bl cl
	mov a bl
	mov #01 a
	cal digit
	mov cl bl

	:output_loop_tail
	clc              ; Increment pointer
	add #01 %0001
	add #00 %0101
	jmp output_loop

	ret
