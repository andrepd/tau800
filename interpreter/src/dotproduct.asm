; Computes dot product of vectors starting %000a and %000b, with length %0001

; Não suporta labels para não complicar o assembler x) portanto tens de figure
; it out à mão. Gastavamos mais tempo a implementar do que a fazer à mão.

;dados
mov #02 %0001  ; len-1

mov #03 %000a
mov #04 %010a
mov #05 %020a

mov #01 %000b
mov #00 %010b
mov #02 %020b

;init
mov %0001 x
mov #00 bl
mov #00 bh

;loop
mov %000a,x cl
mov %000a,x ch
mul %000b,x cl   ; Lower bits of %000a × %000b
muh %000b,x ch   ; Upper bits of %000a × %000b
;clc              ; Clear carry flag
add cl bl        ; Add lower bits
add ch bh        ; Add upper bits + carry, so no need to jsr carry
sec              ; Set carry flag aka clear borrow flag
sub #01 x        ; sub sets NVZ flags
beq +4           ; So we can branch immediately
jmp 3b02         ; Calculado a mao lol

;move to display (assumi que são os dois words a começar em 0x30?)
mov bl %0030
mov bh %0130
nop
clc
bcc -4

;carry
add #01 bh
ret
