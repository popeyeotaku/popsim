        .OFF
        .ORG    0
POINT1  .DS     2
        .ORG    $FE
EOFPRT  .DS     1               ; RETURNS $FE ON EOF
PORT    .DS     1               ; I/O PORT

        .ON
        .ORG    $C000
IRQ
NMI
        RTI

; OUTPUT THE STRING IN (POINT1). A '%' CHARACTER READS STANADRD INPUT AND
; PLACES IT ONTO STANDARD OUTPUT.
PRINT   LDY     #0
PRINT1  LDA     (POINT1),Y
        BEQ     PRINT2
        CMP     #'%'
        BNE     PRINT3
        JSR     CAT
        JMP     PRINT4
PRINT3  STA     PORT
PRINT4  INY
        BNE     PRINT1
PRINT2  RTS

CAT     LDA     PORT
        BNE     CAT1
        BIT     EOFPRT
        BPL     CAT1
        RTS
CAT1    STA     PORT
        JMP     CAT

MSG     .BYTE   CR,LF,CR,LF,'HELLO, %!',CR,LF,0

CR      =       13
LF      =       10

RESET   LDA     #<MSG
        STA     POINT1
        LDA     #>MSG
        STA     POINT1+1
        JSR     PRINT
        JMP     *

        .DS     $FFFA-*
        .ASSERT *=$FFFA
VECTOR  .WORD   NMI,RESET,IRQ
