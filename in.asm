proc start -> {
  call main
  halt
}


;proc main():u64 -> {
;  u64 aaa = 10;
;  u64 bbb = 20;
;  u64 result = add(aaa, bbb, 5);
;  std.print("Hello World!");
;  return 0;
;}
proc main -> {
  ; create a stack frame
  enter 24 ; allocate 48 byte on a stack
  ;u64 aaa = 10;
  mov a64, 10
  mov [sp + 16], a64
  ;u64 bbb = 20;
  mov a64, 20
  mov [sp + 8], a64
  ;u64 result = add(aaa, bbb, 5); 

  mov a64, [sp + 16]
  arg 0, a64  
  mov a64, [sp + 8]
  arg 1, a64  
  mov a64, 5
  arg 2, a64
  call main_add
  mov [sp + 0], a64
  
  ; print("Hello World!");
  mov a64, const_1
  arg 0, a64
  call std_print

  mov a64, 5
  mov a64, a64
  leave ; collapse the stack frame
  ret
}

;proc add(u64 num1, u64 num2, u64 num3):u64 -> {
; u64 x = num1 + num2 + num3 + another_fn() + num3;
; return x;
;}
proc main_add -> {
  enter 8
  ; u64 x = num1 + num2 + num3 + another_fn() + num3;
  larg a64, 0
  larg b64, 1
  add a64, b64
  larg b64, 2
  add a64, b64
  savr a64 ; save the a64
  call another_fn
  mov b64, a64
  restr a64 ; restore the a64
  add a64, b64
  ; stores the value
  mov [sp + 0], a64 
  ; return x
  mov a64, [sp + 0]
  leave
  ret
}

proc another_fn -> {
  mov a64, 69 ; return 69;
  ret
}

proc std_print -> {
  mov c64, 0
  mov b64, 0
  larg a64, 0
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
