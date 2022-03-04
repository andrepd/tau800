;; Main: sieve primes up to 255 ;;

:start

	mov #04 a
	cal init
	mov #04 a
	cal sieve
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
;   a = 64×size
; Locals:
;   %0001 = outer pointer
;   %0200 = inner pointer
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
	bcc +1
	inc %0301
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
