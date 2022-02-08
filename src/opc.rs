#[derive(Copy, Clone)]
pub struct OpMode(pub Inst, pub AddressMode, pub u8);

const OPS: [Option<OpMode>; 0x10] = [
    // 0x00
    Some(OpMode(Inst::Brk, AddressMode::IMPL, 0)),
    Some(OpMode(Inst::Ora, AddressMode::INDX, 6)),
    None,
    None,
    None,
    Some(OpMode(Inst::Ora, AddressMode::ZPG, 3)),
    Some(OpMode(Inst::ASL, AddressMode::ZPG, 0)),
    None,
    Some(OpMode(Inst::Php, AddressMode::IMPL, 0)),
    Some(OpMode(Inst::Ora, AddressMode::IMM, 2)),
    Some(OpMode(Inst::ASL, AddressMode::ACC, 0)),
    None,
    None,
    Some(OpMode(Inst::Ora, AddressMode::ABS, 4)),
    Some(OpMode(Inst::ASL, AddressMode::ABS, 0)),
    None,
    // 0x10
];

pub const fn get_op_mode(opc: u8) -> Option<OpMode> {
    OPS[opc as usize]
}

#[derive(Copy, Clone)]
pub enum Inst {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

#[derive(Copy, Clone)]
pub enum AddressMode {
    ACC,
    ABS,
    ABSX,
    ABSY,
    IMM,
    IMPL,
    IND,
    INDX,
    INDY,
    REL,
    ZPG,
    ZPGX,
    ZPGY,
}
