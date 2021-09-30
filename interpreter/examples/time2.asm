mov #02 %000a
mov #03 %010a

add %000a@+4 %010a   ; Ele vê %000a@+4, faz bootstrap com %000a@+0 (= #02). 
cmp #06 %000a        ; Se %000a - 6 >= 0, 
bpl +5               ; salta o increment, senão incrementa
add #01 %000a        ; Soma 1
nop                  ; Chegando aqui, vê que %000a é #03, volta ao t+0. Vê que o valor
                     ;   foi diferente daquilo que assumiu como initial guess, então faz
                     ;   guess = #03 e volta a repetir o processo. 
                     ; Voltando aqui, vê que %000a é #04, volta ao t+0. Vê que o valor
                     ;   é outra vez diferente
                     ; Voltando aqui, vẽ que é #05 ≠ %04
                     ; Voltando aqui, já saltou o increment, vê que #05 = #05, então encontrou 
                     ;   o ponto fixo e pode substituir %00a@+4 por #05 em `add %000a@+4 %010a`, 
                     ;   e correr a sério. 
