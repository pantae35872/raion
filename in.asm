proc start -> {
  call main
  halt
}

proc main -> {
  ; create a stack frame
  sub sp, 56
  ;u64 aaa = 10;
  mov [sp + 48], 10
  ;u64 bbb = 20;
  mov [sp + 32], 20
  ;u64 result = add(aaa, bbb); 
  mov [sp + 24], [sp + 32], 8
  mov [sp + 16], [sp + 48], 8
  call main_add
  mov [sp + 8], a64
  
  ; print("Hello World!");
  mov [sp + 0], const_1
  call print

  ; collapse a stack frame
  add sp, 56 
  ret
}

proc main_add -> {
  mov a64, [sp + 0]
  mov b64, [sp + 8]
  add a64, b64
  ret
}

proc print -> {
  mov c64, 0
  mov b64, 0
  mov a64, [sp + 0]
PRINT_LOOP:
  mov b8, [a64]
  inc a64
  outc b8
  jacz b64, c64, end
  jmp PRINT_LOOP
end:
  ret
}

proc const_1 -> {
  "Hello World!\0"
}
