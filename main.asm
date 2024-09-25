proc start -> {
   call main$main
   exit a64
}
proc main$part1 -> {
   enter 20
   mov a32, 1
   mov [sp + 0], a32
   mov a32, 2
   mov [sp + 4], a32
   mov a32, 3
   mov [sp + 8], a32
   mov a32, 4
   mov [sp + 12], a32
   mov a32, 5
   mov [sp + 16], a32
   mov a32, [sp + 0]
   arg 0, a64
   mov a32, [sp + 4]
   arg 1, a64
   mov a32, [sp + 8]
   arg 2, a64
   mov a32, [sp + 12]
   arg 3, a64
   mov a32, [sp + 16]
   arg 4, a64
   call main$sum
   leave
   ret
}
proc main$main -> {
   enter 20
   call main$part1
   mov [sp + 0], a32
   call main$part2
   mov [sp + 4], a32
   call main$part3
   mov [sp + 8], a32
   call main$part4
   mov [sp + 12], a32
   mov a32, [sp + 0]
   mov b32, [sp + 4]
   add a64, b64
   mov b32, [sp + 8]
   add a64, b64
   mov b32, [sp + 12]
   add a64, b64
   mov [sp + 16], a32
   mov a32, [sp + 16]
   leave
   ret
}
proc main$part2 -> {
   enter 20
   mov a32, 6
   mov [sp + 0], a32
   mov a32, 7
   mov [sp + 4], a32
   mov a32, 8
   mov [sp + 8], a32
   mov a32, 9
   mov [sp + 12], a32
   mov a32, 10
   mov [sp + 16], a32
   mov a32, [sp + 0]
   arg 0, a64
   mov a32, [sp + 4]
   arg 1, a64
   mov a32, [sp + 8]
   arg 2, a64
   mov a32, [sp + 12]
   arg 3, a64
   mov a32, [sp + 16]
   arg 4, a64
   call main$sum
   leave
   ret
}
proc main$part3 -> {
   enter 20
   mov a32, 11
   mov [sp + 0], a32
   mov a32, 12
   mov [sp + 4], a32
   mov a32, 13
   mov [sp + 8], a32
   mov a32, 14
   mov [sp + 12], a32
   mov a32, 15
   mov [sp + 16], a32
   mov a32, [sp + 0]
   arg 0, a64
   mov a32, [sp + 4]
   arg 1, a64
   mov a32, [sp + 8]
   arg 2, a64
   mov a32, [sp + 12]
   arg 3, a64
   mov a32, [sp + 16]
   arg 4, a64
   call main$sum
   leave
   ret
}
proc main$another_fn -> {
   enter 8
   mov a32, 10
   mov [sp + 0], a32
   mov a32, 20
   mov [sp + 4], a32
   mov a32, [sp + 0]
   mov b32, [sp + 4]
   add a64, b64
   leave
   ret
}
proc main$part4 -> {
   enter 20
   mov a32, 16
   mov [sp + 0], a32
   mov a32, 17
   mov [sp + 4], a32
   mov a32, 18
   mov [sp + 8], a32
   mov a32, 19
   mov [sp + 12], a32
   mov a32, 20
   mov [sp + 16], a32
   mov a32, [sp + 0]
   arg 0, a64
   mov a32, [sp + 4]
   arg 1, a64
   mov a32, [sp + 8]
   arg 2, a64
   mov a32, [sp + 12]
   arg 3, a64
   mov a32, [sp + 16]
   arg 4, a64
   call main$sum
   leave
   ret
}
proc main$sum -> {
   enter 28
   larg a64, 0
   mov [sp + 0], a32
   larg a64, 1
   mov [sp + 4], a32
   larg a64, 2
   mov [sp + 8], a32
   larg a64, 3
   mov [sp + 12], a32
   larg a64, 4
   mov [sp + 16], a32
   mov a32, [sp + 0]
   mov b32, [sp + 4]
   add a64, b64
   mov b32, [sp + 8]
   add a64, b64
   mov b32, [sp + 12]
   add a64, b64
   mov b32, [sp + 16]
   add a64, b64
   mov [sp + 20], a32
   mov a32, [sp + 20]
   arg 0, a64
   call main$modify_result
   mov [sp + 24], a32
   mov a32, [sp + 24]
   leave
   ret
}
proc main$modify_result -> {
   enter 8
   larg a64, 0
   mov [sp + 0], a32
   mov a32, [sp + 0]
   savr a64
   mov a32, 2
   mov b64, a64
   restr a64
   mul a64, b64
   mov [sp + 4], a32
   mov a32, [sp + 4]
   savr a64
   mov a32, 10
   mov b64, a64
   restr a64
   add a64, b64
   mov [sp + 4], a32
   mov a32, [sp + 4]
   savr a64
   mov a32, 3
   mov b64, a64
   restr a64
   div a64, b64
   mov [sp + 4], a32
   mov a32, [sp + 4]
   savr a64
   call main$extra_modifier
   mov b64, a64
   restr a64
   mul a64, b64
   mov [sp + 4], a32
   mov a32, [sp + 4]
   leave
   ret
}
proc main$extra_modifier -> {
   enter 0
   mov a32, 7
   leave
   ret
}
