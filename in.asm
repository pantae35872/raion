section text
start:
  mov a64, hello_world
  mov c64, 0
print:
  mov b8, [a64]
  inc a64
  outc b8
  jacz b64, c64, end 
  jmp print
end:
  halt

section data
hello_world: "Hello, World!\n\0"
