; Computes dot product of vectors starting %000a and %000b, with length %0001

; Não suporta labels para não complicar o assembler x) portanto tens de figure
; it out à mão. Gastavamos mais tempo a implementar do que a fazer à mão.

;dados
mov #03 %0001

mov #03 %000a
mov #04 %010a
mov #05 %020a

mov #01 %000b
mov #00 %010b
mov #02 %020b

;init
mov %0001 x
mov #0 bl
mov #0 bh

;loop
mov %00a0,x cl
mov %00a0,x ch
mul %00b0,x cl   ; Lower bits of %000a × %000b
muh %00b0,x ch   ; Upper bits of %000a × %000b
clc              ; Clear carry flag
add cl bl        ; Add lower bits
add ch bh        ; Add upper bits + carry, so no need to jsr carry
sub #1 x         ; sub sets NVZ flags
bne -34          ; So we can branch immediately

;move to display (assumi que são os dois words a começar em 0x40?)
mov bl %0040
mov bh %0140
nop
clc
bcc -4

;carry
add #1 bh
ret
