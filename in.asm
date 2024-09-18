proc start -> {
  mov a64, hello_world 
  call print
  halt
}

proc print -> {
  push b64
  push c64
  mov c64, 0
print_loop:
  mov b8, [a64]
  inc a64
  outc b8
  jacz b64, c64, end 
  jmp print_loop
end:
  pop c64
  pop b64
  ret
}

const hello_world -> { 
  "Hello, World!\n\0"
}

const second -> { 
  "Second, Message\n\0"
}
