proc start -> {
   call main$main
   exit a64
}
proc main$main -> {
   enter 32
   mov a64, 10
   mov [sp + 0], a64
   mov a64, 20
   mov [sp + 8], a64
   mov a64, 10
   mov [sp + 16], a64
   mov a64, [sp + 0]
   arg 0, a64
   mov a64, [sp + 8]
   arg 1, a64
   mov a64, 5
   arg 2, a64
   call main$add
   mov [sp + 24], a64
   mov a64, [sp + 24]
   leave
   ret
}
proc main$add -> {
   enter 32
   larg a64, 0
   mov [sp + 0], a64
   larg a64, 1
   mov [sp + 8], a64
   larg a64, 2
   mov [sp + 16], a64
   mov a64, [sp + 0]
   mov b64, [sp + 8]
   add a64, b64
   mov b64, [sp + 16]
   add a64, b64
   mov [sp + 24], a64
   mov a64, [sp + 24]
   mov b64, 10
   add a64, b64
   mov [sp + 24], a64
   mov a64, [sp + 24]
   leave
   ret
}
proc main$another_fn -> {
   enter 0
   mov a64, 69
   leave
   ret
}
