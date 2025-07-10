mod cpu;

use crate::cpu::*;

fn main() {
    let mut pc;
    let mut ir: i16;
    let mut flag_eq: bool;

    assembler();

    pc = 0;
    flag_eq = false;

    loop {
        unsafe {
            ir = ROM[pc];
            println!(
                "{:?} {:?} {:?} {:?} {:?} {:?}",
                pc, ir, REG[0], REG[1], REG[2], REG[3]
            );
        }

        pc += 1;

        unsafe {
            match op_code(ir) {
                MOV => REG[op_reg_a(ir) as usize] = REG[op_reg_b(ir) as usize],
                ADD => REG[op_reg_a(ir) as usize] += REG[op_reg_b(ir) as usize],
                SUB => REG[op_reg_a(ir) as usize] -= REG[op_reg_b(ir) as usize],
                AND => REG[op_reg_a(ir) as usize] &= REG[op_reg_b(ir) as usize],
                OR => REG[op_reg_a(ir) as usize] |= REG[op_reg_b(ir) as usize],
                SL => REG[op_reg_a(ir) as usize] = REG[op_reg_a(ir) as usize] << 1,
                SR => REG[op_reg_a(ir) as usize] = REG[op_reg_a(ir) as usize] >> 1,
                SRA => {
                    REG[op_reg_a(ir) as usize] = (REG[op_reg_a(ir) as usize] & 0x8000u16 as i16)
                        | (REG[op_reg_a(ir) as usize] >> 1)
                }
                LDL => {
                    REG[op_reg_a(ir) as usize] =
                        (REG[op_reg_a(ir) as usize] & 0xff00u16 as i16) | (op_data(ir) & 0x00ff)
                }
                LDH => {
                    REG[op_reg_a(ir) as usize] =
                        (op_data(ir) << 8) | (REG[op_reg_a(ir) as usize] & 0x00ff);
                }
                CMP => flag_eq = REG[op_reg_a(ir) as usize] == REG[op_reg_b(ir) as usize],
                JE => {
                    if flag_eq {
                        pc = op_addr(ir) as usize;
                    }
                }
                JMP => pc = op_addr(ir) as usize,
                LD => REG[op_reg_a(ir) as usize] = RAM[op_addr(ir) as usize],
                ST => RAM[op_addr(ir) as usize] = REG[op_reg_a(ir) as usize],
                _ => {} // Handle any other op_code values (e.g., HLT or unknown)
            }
        }

        if op_code(ir) == HLT {
            break;
        }
    }
    unsafe {
        println!("ram[64]: {}", RAM[64]);
    }
}
