start:
  mov sp, 0xFFFE
  mov a64, 0
  mov b64, 1000000000
loop:
  inc a64
  jacn a64, b64, loop
end:
  halt
