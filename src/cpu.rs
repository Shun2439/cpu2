pub(crate) const MOV: i16 = 0;
pub(crate) const ADD: i16 = 1;
pub(crate) const SUB: i16 = 2;
pub(crate) const AND: i16 = 3;
pub(crate) const OR: i16 = 4;
pub(crate) const SL: i16 = 5;
pub(crate) const SR: i16 = 6;
pub(crate) const SRA: i16 = 7;
pub(crate) const LDL: i16 = 8;
pub(crate) const LDH: i16 = 9;
pub(crate) const CMP: i16 = 10;
pub(crate) const JE: i16 = 11;
pub(crate) const JMP: i16 = 12;
pub(crate) const LD: i16 = 13;
pub(crate) const ST: i16 = 14;
pub(crate) const HLT: i16 = 15;
pub(crate) const REG0: i16 = 0;
pub(crate) const REG1: i16 = 1;
pub(crate) const REG2: i16 = 2;
pub(crate) const REG3: i16 = 3;
pub(crate) const REG4: i16 = 4;
pub(crate) const REG5: i16 = 5;
pub(crate) const REG6: i16 = 6;
pub(crate) const REG7: i16 = 7;

pub(crate) static mut REG: [i16; 8] = [0; 8];
pub(crate) static mut ROM: [i16; 256] = [0; 256];
pub(crate) static mut RAM: [i16; 256] = [0; 256];

pub fn assembler() {
    unsafe {
        ROM[0] = ldh(REG0, 0);
        ROM[1] = ldl(REG0, 0);
        ROM[2] = ldh(REG1, 0);
        ROM[3] = ldl(REG1, 1);
        ROM[4] = ldh(REG2, 0);
        ROM[5] = ldl(REG2, 0);
        ROM[6] = ldh(REG3, 0);
        ROM[7] = ldl(REG3, 10);
        ROM[8] = add(REG2, REG1);
        ROM[9] = add(REG0, REG2);
        ROM[10] = st(REG0, 64);
        ROM[11] = cmp(REG2, REG3);
        ROM[12] = je(14); 
        ROM[13] = jmp(8);
        ROM[14] = hlt();
    }
}

pub fn mov(ra: i16, rb: i16) -> i16 {
    (MOV << 11) | (ra << 8) | (rb << 5)
}

pub fn add(ra: i16, rb: i16) -> i16 {
    (ADD << 11) | (ra << 8) | (rb << 5)
}
pub fn sub(ra: i16, rb: i16) -> i16 {
    (SUB << 11) | (ra << 8) | (rb << 5)
}

pub fn and(ra: i16, rb: i16) -> i16 {
    (AND << 11) | (ra << 8) | (rb << 5)
}

pub fn or(ra: i16, rb: i16) -> i16 {
    (OR << 11) | (ra << 8) | (rb << 5)
}

pub fn sl(ra: i16) -> i16 {
    (SL << 11) | (ra << 8)
}

pub fn sr(ra: i16) -> i16 {
    (SR << 11) | (ra << 8)
}

pub fn sra(ra: i16) -> i16 {
    (SRA << 11) | (ra << 8)
}

pub fn ldl(ra: i16, ival: i16) -> i16 {
    (LDL << 11) | (ra << 8) | (ival & 0x00ff)
}

pub fn ldh(ra: i16, ival: i16) -> i16 {
    (LDH << 11) | (ra << 8) | (ival & 0x00ff)
}

pub fn cmp(ra: i16, rb: i16) -> i16 {
    (CMP << 11) | (ra << 8) | (rb << 5)
}

pub fn je(addr: i16) -> i16 {
    (JE << 11) | (addr & 0x00ff)
}

pub fn jmp(addr: i16) -> i16 {
    (JMP << 11) | (addr & 0x00ff)
}

pub fn ld(ra: i16, addr: i16) -> i16 {
    (LD << 11) | (ra << 8) | (addr & 0x00ff)
}

pub fn st(ra: i16, addr: i16) -> i16 {
    (ST << 11) | (ra << 8) | (addr & 0x00ff)
}

pub fn hlt() -> i16 {
    HLT << 11
}
pub fn op_code(ir: i16) -> i16 {
    ir >> 11
}

pub fn op_reg_a(ir: i16) -> i16 {
    (ir >> 8) & 0x0007
}

pub fn op_reg_b(ir: i16) -> i16 {
    (ir >> 5) & 0x0007
}

pub fn op_data(ir: i16) -> i16 {
    ir & 0x00ff
}

pub fn op_addr(ir: i16) -> i16 {
    ir & 0x00ff
}