; Inputs (hh:mm): %10 %11 %12 %13
; Outputs (display segments): %14 ... 1a

;; Main ;;

:start

	mov #03 ch      ; r→l
	cal sweep
	mov #03 ch
	cal sqrt

	mov #00 ch      ; l→r
	cal sweep
	mov #00 ch
	cal sqrt

	jmp start



;; Subroutine: do one sweep of all digits ;;
; Inputs:
;   ch = direction (0 l→r, 3 r→l)
; Locals:
;   cl = increment
;   x = offset

:sweep

	clc
	mov ch cl
	add #01 cl  ; These lines map 0 → -1 e 3 → +1 
	lsr cl      ;
	add #3f cl  ; 
	clc
	not cl
	add #01 cl

	; Digit 1
	mov ch x
	mov #01 a
	mov %1000,x bl
	cal clear
	cal digit

	clc
	add cl ch 

	; Digit 2
	mov ch x
	mov #02 a
	mov %1000,x bl
	cal clear
	cal digit

	clc
	add cl ch 

	; Digit 3
	mov ch x
	mov #04 a
	mov %1000,x bl
	cal clear
	cal digit

	clc
	add cl ch 

	; Digit 4
	mov ch x
	mov #08 a
	mov %1000,x bl
	cal clear
	cal digit

	ret



;; Subroutine: set one digit ;;
; Inputs:
;   a = current bit mask
;   bl = current digit
; Locals:
;   x = current segment

:digit

	mov #00 x

	; First digit

	; If number not 1237, then set segment 1
	sec
	cmp #07 bl
	beq +7
	sec
	cmp #01 bl  ; bl-1 < 0 ⇔ bl < 1
	bmi +3
	sec
	cmp bl #03  ; 3-bl ≥ 0 ⇔ bl ≤ 3
	bpl +1
	or a %1400,x

	inc x

	; If number not 14, then set segment 2
	sec
	cmp #01 bl
	beq +4
	sec
	cmp #04 bl
	beq +1
	or a %1400,x

	inc x

	; If number not 56, then set segment 3
	sec
	cmp #05 bl
	beq +4
	sec
	cmp #06 bl
	beq +1
	or a %1400,x

	inc x

	; If number is 0268, then set segment 4 (clever)
	sec
	cmp #04 bl  ; If 4
	beq +3
	bit #01 bl  ; Or odd
	bne +1
	or a %1400,x

	inc x

	; If number not 147, then set segment 5
	sec
	cmp #01 bl
	beq +7
	sec
	cmp #04 bl
	beq +4
	sec
	cmp #07 bl
	beq +1
	or a %1400,x

	inc x

	; If number not 2, then set segment 6
	sec
	cmp #02 bl
	beq +1
	or a %1400,x

	inc x

	; If number not 017, then set segment 7
	sec
	cmp bl #01  ; 1-bl ≥ 0 ⇔ bl ≤ 1
	bpl +4
	sec
	cmp #07 bl
	beq +1
	or a %1400,x

	ret



;; Subroutine: clear one digit ;;
; Inputs:
;   a = current bit mask
; Locals:
;   x = current segment

:clear

	sec
	not a
	mov #06 x

	and a %1400,x
	sub #01 x
	bne -3
	and a %1400,x

	clc
	not a
	ret



;; Subroutine: calculate sqrt of 6-bit number in O(1), with time-assisted Newton's method
; Inputs:
;   a = number
;   flag C = msb of number
; Locals:
;   bl = guess_{i}
;   cl = guess_{i+1}
;   bh = scratch space
;   ch = store carry bit
; Outputs:
;   a = √ of input

; Pseudocode:              | time
;   b ← input/2            | 0s
;   c ← (b + input/b) / 2  | 1s
;   b @ 0s ← c @ 2s        | 2s
;   return b               | 3s

:newton

	; Stash carry bit
	mov #00 ch
	bcc +1
	mov #01 ch

	; Initial guess: b = input' = input / 2
	mov a bl
	lsr bl     ; bl ← a ← a/2

	; √0 = 0, √1 = 1
	bne +1
	ret

	; Restore carry bit
	clc
	mov ch ch
	beq +1
	sec

	; Improved guess: c = (b + input / b) / 2 
	mov bl cl
	mov a bh
	div bl bh
	add bh cl
	lsr cl

	; c = min(b,c), needed for corner case where input is 1 less than a perfect square
	cmp bl cl
	bmi +2
	mov bl cl
	bne +2  ; Needs these nop to ensure same time of execution of both branches, 
	nop     ; and thus avoid temporal inconsistencies.
	nop     ; 

	; Put back the improved guess as the initial guess
	mov cl bl@-10

	; Return result through a
	mov bl a
	ret



;; Subroutine: decimal digits → 6-bit number
; Inputs:
;   bh,bl = number
; Locals:
;   bh
; Outputs:
;   a = bh×10+bl
;   flag C = 1 if overflows

:decimal

	mov bh a
	clc
	mul #0a a
	add bl a    ; Overflow in add: C set here
	muh #0a bh  ; Overflow in mul: C set here
	beq +1
	sec
	ret



;; Subroutine: calculate sqrt of hh, of mm, and write to display
; Inputs:
;   flag Z = direction (0 l→r, 1 r→l)

:sqrt

	; Self-modifying code, handle with care!
	beq +5
	mov #13 %1d09  ; l→r
	mov #12 %2209
	mov #11 %020a
	mov #10 %070a
	bne +4
	mov #10 %1d09  ; r→l
	mov #11 %2209
	mov #12 %020a
	mov #13 %070a

	mov %3f00 bh
	mov %3f00 bl
	cal decimal
	cal newton

	mov a bl
	mov #01 a
	cal clear
	mov #02 a
	cal clear
	cal digit

	mov %3f00 bh
	mov %3f00 bl
	cal decimal
	cal newton

	mov a bl
	mov #04 a
	cal clear
	mov #08 a
	cal clear
	cal digit

	ret
