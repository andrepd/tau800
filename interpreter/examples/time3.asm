; Trivial write to past

mov #02 %000a         ;t=1
mov #03 %010a         ;t=2
nop                   ;t=3
add %000a %010a       ;t=4
nop                   ;t=5
mov #11 %020a@-03     ;t=6
nop                   ;t=7
