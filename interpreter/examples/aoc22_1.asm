; Time-assisted bubble-sort: 

; Input:
;   b: Pointer to start of array of 4-word values
;   a: Length

:bubble

  cmp 


  cmp a
  beq bubble:end
  jmp bubble:loop
  ret



; 4-byte swap
; Input:
;   b,c: Pointers to values to swap
; Clobbers:
;   a,x

;; Subroutine: do one sweep of all digits ;;
; Inputs:
;   ch = direction (0 l→r, 3 r→l)
; Locals:
;   cl = increment
;   x = offset

:swap4
  mov #0 x
  mov (b),x a
  mov (c),x (b),x
  mov a (c),x
  inc x
  mov (b),x a
  mov (c),x (b),x
  mov a (c),x
  inc x
  mov (b),x a
  mov (c),x (b),x
  mov a (c),x
  inc x
  mov (b),x a
  mov (c),x (b),x
  mov a (c),x
  ret
