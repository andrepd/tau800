; Inputs: %10 %11 %12 %13
; Outputs: %14 ... 1a

;; Main ;;

:start = 0002

	mov #03 ch      ; r→l
	cal 2303:sweep
	mov #03 ch
	cal 1510:sqrt

	mov #00 ch      ; l→r
	cal 2303:sweep
	mov #00 ch
	cal 1510:sqrt

	jmp 0002:start

nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop




;; Subroutine: do one sweep of all digits ;;
; Inputs:
;   ch: direction (0 l→r, 3 r→l)
; Locals:
;   cl: increment
;   x: offset

:sweep = 2303

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
	cal 1a0b:clear
	cal 3806:digit

	clc
	add cl ch 

	; Digito 2
	mov ch x
	mov #02 a
	mov %1000,x bl
	cal 1a0b:clear
	cal 3806:digit

	clc
	add cl ch 

	; Digito 3
	mov ch x
	mov #04 a
	mov %1000,x bl
	cal 1a0b:clear
	cal 3806:digit

	clc
	add cl ch 

	; Digito 4
	mov ch x
	mov #08 a
	mov %1000,x bl
	cal 1a0b:clear
	cal 3806:digit

	ret

nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop




;; Subroutine: set one digit ;;
; Inputs:
;   a = current bit mask
;   bl = current digit
; Locals:
;   x = current segment

:digit = 3806

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
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
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

:clear = 1a0b

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

nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
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
;   bl: guess_i
;   cl: guess_i+1
; Outputs:
;   a: √ of input

:newton = 3f0c

	; Initial guess: b = input / 2
	mov a bl
	lsr bl

	; If b == 0, then a == 0,1 therefore √a = a
	bne +1
	ret

	; Improved guess: c = (b + input / b) / 2
	clc
	mov a cl 
	div bl cl
	add bl cl
	lsr cl

	; Corner case, if input is 1 less than a perfect square, cl will oscillate rather than fixpoint
	cmp bl cl  ; If cl<bl, nop. If cl>bl, make cl=bl. I.e. cl = min(bl,cl)
	bmi +08
	mov bl cl
	bne +02
	nop  ; Needs these nop to encher chouriças to ensure same time of execution of both branches
	nop

	; Put back the improved guess as the initial guess
	mov cl bl@-10

	; Return result through a
	mov bl a
	ret

nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop




;; Subroutine: decimal digits → 6-bit number
; Inputs:
;   bh,bl: number
; Outputs:
;   a: bh×10+bl

:decimal = 020f

	mov bh a
	clc
	mul #0a a
	add bl a
	ret

nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop




;; Subroutine: calculate sqrt of hh, of mm, and write to display
; Inputs:
;   ch: direction (0 l→r, 3 r→l)
;   bh,bl: number
; Outputs:
;   a: √ of input

:sqrt = 1510

	;cmp #00 ch
	beq +4
	cal 0a14:sqrt_rl
	ret
	cal 1f11:sqrt_lr
	ret

nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop




;; Subroutine: calculate sqrt of hh, of mm, and write to display
; Inputs:
;   bh,bl: number

:sqrt_lr = 1f11

	mov %1000 bh
	mov %1100 bl
	cal 020f:decimal
	cal 3f0c:newton

	mov a bl
	mov #02 a
	cal 1a0b:clear
	cal 3806:digit
	mov #00 bl
	mov #01 a
	cal 1a0b:clear
	cal 3806:digit

	mov %1200 bh
	mov %1300 bl
	cal 020f:decimal
	cal 3f0c:newton

	mov a bl
	mov #08 a
	cal 1a0b:clear
	cal 3806:digit
	mov #00 bl
	mov #04 a
	cal 1a0b:clear
	cal 3806:digit

	ret

nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop




;; Subroutine: calculate sqrt of hh, of mm, and write to display
; Inputs:
;   bh,bl: number

:sqrt_rl = 0a14

	mov %1300 bh
	mov %1200 bl
	cal 020f:decimal
	cal 3f0c:newton

	mov a bl
	mov #02 a
	cal 1a0b:clear
	cal 3806:digit
	mov #00 bl
	mov #01 a
	cal 1a0b:clear
	cal 3806:digit

	mov %1100 bh
	mov %1000 bl
	cal 020f:decimal
	cal 3f0c:newton

	mov a bl
	mov #08 a
	cal 1a0b:clear
	cal 3806:digit
	mov #00 bl
	mov #04 a
	cal 1a0b:clear
	cal 3806:digit

	ret

nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop

