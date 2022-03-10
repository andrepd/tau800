;;;;;;;;;;;;;;;;;;;;;;
; Common subroutines ;
;;;;;;;;;;;;;;;;;;;;;;

;; Subroutine: set one digit ;;
; Inputs:
;   a = current bit mask (2 ^ digit)
;   bl = current digit
; Locals:
;   x = current segment

:digit

	mov #00 x

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
	cmp #04 bl   ; Se for 4
	beq +3
	bit #01 bl  ; Ou ímpar
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
;   a = current bit mask (2 ^ digit)
; Locals:
;   x = current segment

:clear

	sec
	not a
	mov #06 x

	and a %1400,x
	dec x
	bne -3
	and a %1400,x

	clc
	not a
	ret

;; Subroutine: decimal digits → 6-bit number
; Inputs:
;   bh,bl = digits
; Locals:
;   bh
; Outputs:
;   a = bh×10+bl
;   flag C: 1 if overflows

:dec2num

	mov bh a
	clc
	mul #0a a
	add bl a    ; Overflow in add: C set here
	muh #0a bh  ; Overflow in mul: C set here
	beq +1
	sec
	ret

;; Subroutine: 12-bit number → decimal digits
; Inputs:
;   bh,bl = number
; Locals:
;   ch,cl
; Outputs:
;   a: rightmost digit (number % 10)
;   bh,bl: rest of number (number / 10)

:num2dec

	; Literally long division.
	; 10 = 0×64¹ + 10×64⁰

	; bhbl / 10: h = bh / 10, l = bh%10+bl / 10

	clc

	mov bh ch    
    mov bl cl
    mov #00 bl

	div #0a bh  ; Divide first digit
    mod #0a ch  ; Put remainder in high byte of ch

    ;; :num2dec_loop_start
    ;; bne +1
    ;; jmp num2dec_loop_end
    ;; inc bl
    ;; sec
    ;; sub #0a cl
    ;; sub #00 ch
    ;; jmp num2dec_loop_start
    ;; :num2dec_loop_end

    ;; :num2dec_loop_start
    ;; bne +1
    ;; jmp num2dec_loop_end
    ;; inc bl
    ;; sec
    ;; sub #0a cl
    ;; sub #00 ch
    ;; mov bl bl@-4
    ;; mov cl cl@-5
    ;; mov ch ch@-6
    ;; :num2dec_loop_end

    ; Now we want to divide chcl by 10. Each unit in ch contributes 6 to the divisor and 4 to the remainder

    mov ch bl   ; lower digit = 6×ch ...
    mul #06 bl

    mov ch a
    mul #04 a
    add cl a
    mov a cl
    mod #0a a
    div #0a cl

    clc         ; ... + 4×ch+cl / 10
    add cl bl

	;; clc
	;; 
    ;; ; Here ch is 0, re-use
	;; mov cl a    ; Divide second digit
	;; mod #0a a   ; Remainder
    ;; div #0a cl
    ;; add cl bl  

	ret
