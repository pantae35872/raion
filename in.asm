start:
  mov b64, 1000000000
  mov sp, 0xFFFE
  push b64
  mov b64, 254
  pop b64 ;aaa
end:
  halt
