mov #02 $000a
mov #03 $010a

add $000a@+2 $010a   ; Ele vê $000a@+2, faz bootstrap com $000a@+0 (= #02) por exemplo?
mov #04 $000a        ; Continua a emular até chegar a t+2
nop                  ; Chegando aqui, vê que $000a é #04, volta ao t+0. Vê que o valor
                     ;   foi diferente daquilo que assumiu como initial guess, então faz
                     ;   guess = #04 e volta a repetir o processo. 
                     ; Voltando aqui, vê que $000a é #04, que é igual ao guess, então é
                     ;   ponto fixo e acabou. Encontrou o ponto fixo, pode executar o add
                     ;   e daí para a frente. 
