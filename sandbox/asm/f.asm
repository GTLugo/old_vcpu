  .org $8000

reset:
  lda #$ff
  sta $6002

loop:
  lda #$55
  sta $6000

  jmp loop

  .org $fffc
  .word reset
  .word $6969
