#!/bin/zsh

# set -x

masked_eq() {
    local mask="$1"; shift
    local bin="$1"; shift
    while [ $# -gt 0 ]; do
        # printf "(( ($bin & $mask) == $1 ))"
        (( ($bin & $mask) == $1 )) && return 0
        shift
    done
    return 1
}

ln_eq() {
    masked_eq 0x0f "$@"
    return $?
}

un_eq() {
    masked_eq 0xf0 "$@"
    return $?
}

eq() {
    masked_eq 0xff "$@"
    return $?
}

bitpat() {
    local bin="$1" pat="$2"
    local mask="$(printf "$pat" | sed 's/[^x]/1/g; s/x/0/g')"
    echo " bin=$bin"
    echo " pat=$pat"
    echo "mask=$mask"
}

bitpat 1000_0000 0xx00000
exit

awk '$NF{print}' ins-bin.txt | while read name hex bin arg addr_type; do
# while read name hex bin arg addr_type; do
    echo "$name : codes=0x$hex,0b$bin arg=$arg"
    if ln_eq "0b$bin" 0x8 0xa || eq "0b$bin" 0x00 0x40 0x60; then
    ### xxxx1000
    # PHP : $08 : 0000 1000
    # CLC : $18 : 0001 1000
    # PLP : $28 : 0010 1000
    # SEC : $38 : 0011 1000
    # PHA : $48 : 0100 1000
    # CLI : $58 : 0101 1000
    # PLA : $68 : 0110 1000
    # SEI : $78 : 0111 1000
    # DEY : $88 : 1000 1000
    # TYA : $98 : 1001 1000
    # TAY : $a8 : 1010 1000
    # CLV : $b8 : 1011 1000
    # INY : $c8 : 1100 1000
    # CLD : $d8 : 1101 1000
    # INX : $e8 : 1110 1000
    # SED : $f8 : 1111 1000

    ### xxxx1010
    # ASL : $0a : 0000 1010
    # ROL : $2a : 0010 1010
    # LSR : $4a : 0100 1010
    # ROR : $6a : 0110 1010
    # TXA : $8a : 1000 1010
    # TXS : $9a : 1001 1010
    # TAX : $aa : 1010 1010
    # TSX : $ba : 1011 1010
    # DEX : $ca : 1100 1010
    # NOP : $ea : 1110 1010

    ### 0xx00000 && !00100000
    # BRK : $00 : 0 00 00000
    # RTI : $40 : 0 10 00000
    # RTS : $60 : 0 11 00000

        guess_arg=0
    elif ln_eq "0b$bin" 0x1 0x4 0x5 0x6; then
    ### xxxx0001
    # ORA : $01 : 0000 0001
    # ORA : $11 : 0001 0001
    # AND : $21 : 0010 0001
    # AND : $31 : 0011 0001
    # EOR : $41 : 0100 0001
    # EOR : $51 : 0101 0001
    # ADC : $61 : 0110 0001
    # ADC : $71 : 0111 0001
    # STA : $81 : 1000 0001
    # STA : $91 : 1001 0001
    # LDA : $a1 : 1010 0001
    # LDA : $b1 : 1011 0001
    # CMP : $c1 : 1100 0001
    # CMP : $d1 : 1101 0001
    # SBC : $e1 : 1110 0001
    # SBC : $f1 : 1111 0001

    ### xxxx0100
    #     : $04 : 0000 0100 : NOT USED
    #     : $14 : 0001 0100 : NOT USED
    # BIT : $24 : 0010 0100
    #     : $34 : 0011 0100 : NOT USED
    #     : $44 : 0100 0100 : NOT USED
    #     : $54 : 0101 0100 : NOT USED
    #     : $64 : 0110 0100 : NOT USED
    #     : $74 : 0111 0100 : NOT USED
    # STY : $84 : 1000 0100
    # STY : $94 : 1001 0100
    # LDY : $a4 : 1010 0100
    # LDY : $b4 : 1011 0100
    # CPY : $c4 : 1100 0100
    #     : $d4 : 1101 0100 : NOT USED
    # CPX : $e4 : 1110 0100
    #     : $f4 : 1110 0100 : NOT USED

    ### xxxx0101
    # ORA : $05 : 0000 0101
    # ORA : $15 : 0001 0101
    # AND : $25 : 0010 0101
    # AND : $35 : 0011 0101
    # EOR : $45 : 0100 0101
    # EOR : $55 : 0101 0101
    # ADC : $65 : 0110 0101
    # ADC : $75 : 0111 0101
    # STA : $85 : 1000 0101
    # STA : $95 : 1001 0101
    # LDA : $a5 : 1010 0101
    # LDA : $b5 : 1011 0101
    # CMP : $c5 : 1100 0101
    # CMP : $d5 : 1101 0101
    # SBC : $e5 : 1110 0101
    # SBC : $f5 : 1111 0101

    ### xxxx0110
    # ASL : $06 : 0000 0110
    # ASL : $16 : 0001 0110
    # ROL : $26 : 0010 0110
    # ROL : $36 : 0011 0110
    # LSR : $46 : 0100 0110
    # LSR : $56 : 0101 0110
    # ROR : $66 : 0110 0110
    # ROR : $76 : 0111 0110
    # STX : $86 : 1000 0110
    # STX : $96 : 1001 0110
    # LDX : $a6 : 1010 0110
    # LDX : $b6 : 1011 0110
    # DEC : $c6 : 1100 0110
    # DEC : $d6 : 1101 0110
    # INC : $e6 : 1110 0110
    # INC : $f6 : 1111 0110

    ### xxx10000
    # BPL : $10 : 000 10000
    # BMI : $30 : 001 10000
    # BVC : $50 : 010 10000
    # BVS : $70 : 011 10000
    # BCC : $90 : 100 10000
    # BCS : $b0 : 101 10000
    # BNE : $d0 : 110 10000
    # BEQ : $f0 : 111 10000

    ### 1xx00000
    #     : $80 : 1 00 00000 : NOT USED
    # LDY : $a0 : 1 01 00000
    # CPY : $c0 : 1 10 00000
    # CPX : $e0 : 1 11 00000

    ### xxx01001
    # ORA : $09 : 000 01001
    # AND : $29 : 001 01001
    # EOR : $49 : 010 01001
    # ADC : $69 : 011 01001
    #     : $89 : 100 01001 : NOT USED
    # LDA : $a9 : 101 01001
    # CMP : $c9 : 110 01001
    # SBC : $e9 : 111 01001

    ### 10100010
    # LDX : $a2 : 10100010

        guess_arg=b

    elif ln_eq "0b$bin" 0xc 0xd 0xe; then
    ### xxx11001
    # ORA : $19 : 000 11001
    # AND : $39 : 001 11001
    # EOR : $59 : 010 11001
    # ADC : $79 : 011 11001
    # STA : $99 : 100 11001
    # LDA : $b9 : 101 11001
    # CMP : $d9 : 110 11001
    # SBC : $f9 : 111 11001

    ### xxxx1100
    #     : $0c : 0000 1100 : NOT USED
    #     : $1c : 0001 1100 : NOT USED
    # BIT : $2c : 0010 1100
    #     : $3c : 0011 1100 : NOT USED
    # JMP : $4c : 0100 1100
    #     : $5c : 0101 1100 : NOT USED
    # JMP : $6c : 0110 1100
    #     : $7c : 0111 1100 : NOT USED
    # STY : $8c : 1000 1100
    #     : $9c : 1001 1100 : NOT USED
    # LDY : $ac : 1010 1100
    # LDY : $bc : 1011 1100
    # CPY : $cc : 1100 1100
    #     : $dc : 1101 1100 : NOT USED
    # CPX : $ec : 1110 1100
    #     : $fc : 1111 1100 : NOT USED

    ### xxxx1101
    # ORA : $0d : 0000 1101
    # ORA : $1d : 0001 1101
    # AND : $2d : 0010 1101
    # AND : $3d : 0011 1101
    # EOR : $4d : 0100 1101
    # EOR : $5d : 0101 1101
    # ADC : $6d : 0110 1101
    # ADC : $7d : 0111 1101
    # STA : $8d : 1000 1101
    # STA : $9d : 1001 1101
    # LDA : $ad : 1010 1101
    # LDA : $bd : 1011 1101
    # CMP : $cd : 1100 1101
    # CMP : $dd : 1101 1101
    # SBC : $ed : 1110 1101
    # SBC : $fd : 1111 1101

    ### xxxx1110
    # ASL : $0e : 0000 1110
    # ASL : $1e : 0001 1110
    # ROL : $2e : 0010 1110
    # ROL : $3e : 0011 1110
    # LSR : $4e : 0100 1110
    # LSR : $5e : 0101 1110
    # ROR : $6e : 0110 1110
    # ROR : $7e : 0111 1110
    # STX : $8e : 1000 1110
    #     : $9e : 1001 1110 : NOT USED
    # LDX : $ae : 1010 1110
    # LDX : $be : 1011 1110
    # DEC : $ce : 1100 1110
    # DEC : $de : 1101 1110
    # INC : $ee : 1110 1110
    # INC : $fe : 1111 1110

    ### 00100000
    # JSR $20 : 00100000
        guess_arg=w
    else
        guess_arg=UNKNOWN
    fi
    echo "                      guess_arg=$guess_arg"
done
# done <<HERE
# ROL 2a 00101010 0 acc
# INC fe 11111110 w abs_x
# HERE
