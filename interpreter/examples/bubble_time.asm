; Idea: 
;   - loop through array (of 4-word values) from %0030 to %3f3f
;   - for each block: sum the values, put into 4-th element of array starting at %0020, bubble sort high to low

; Teste

; Array inicial: __ 10 __ __ / 01 10 __ __ / 3f 0f __ __ / 0a 10 __ __ 
; Array final:   0a 10 __ __ / 01 10 __ __ / __ 10 __ __ / 3f 0f __ __ 

mov #00 %0020
mov #10 %0120

mov #01 %0420
mov #10 %0520

mov #3f %0820
mov #0f %0920

mov #0a %0c20
mov #10 %0d20

mov #04 a
cal bubble
mov #04 a
cal bubble
mov #04 a
cal bubble
mov #04 a
cal bubble
mov #04 a
cal bubble

hcf



; Time-assisted bubble-sort: 

; Input:
;   ~~b: Pointer to start of array of 4-word values~~ hardcoded %0020
;   a: Length (up to 16)

:bubble

  ; Let x = (length-1) * 4
  mov a x
  dec x
  lsl x
  lsl x

  ; For each i from length-1 to 0 (i.e. while x is not 0)
  
  ; Compare word at ptr+x and ptr+x+4, high to low
  ;   ptr+x > ptr+x+4 : correct, go to next value
  ;   ptr+x < ptr+x+4 : incorrect, swap on input array
  ;   ptr+x = ptr+x+4 : check next word

  :loop
  bit #3f x  ; if x = 0 then ret
  bne +1
  ret
  
  dec x
  sec
  cmp %0020,x %0420,x  ;note: N flag set if ptr > ptr+4

  ; =
  bne +1
  jmp loop

  ; >
  bpl +2
  and #3c x  ; Round to lowest multiple of 4
  jmp loop

  ; <
  and #3c x  ; Round to lowest multiple of 4
  mov %0020,x %0020,x@-7
  mov %0120,x %0120,x@-8
  mov %0220,x %0220,x@-9
  mov %0320,x %0320,x@-10
  ret
