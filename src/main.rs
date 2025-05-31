mod types;

mod mem;
use mem::Memory;

mod cpu;
use cpu::{Cpu, Flag::*, Ins::*};

// fn bits_match(bin: u8, mut pat_spec: u32) -> bool {
//     let mut ignore = 0u8;
//     let mut pat = 0u8;
//     for i in 0..8 {
//         if i > 0 {
//             pat_spec <<= 4;
//             ignore <<= 1;
//             pat <<= 1;
//         }
//         match 0xf000_0000 & pat_spec {
//             0xf000_0000 => ignore |= 1,
//             0x0000_0000 => {}, // pat |= 0
//             0x1000_0000 => pat |= 1,
//             x => panic!("bit specification not supported: 0x{:x}", x >> 28),
//         }
//     }
//     (bin & ignore) == pat
// }

fn main() -> Result<(), u8> {
    let mut mem = Memory::new();
    mem.write_bytes(0xfffc, &[0x00, 0xc0]);
    // Write the reset routine (currently in big endian ??)
    mem.write_bytes(0xfce2, vec![
        ins!(LDX_IMM), 0xff,
        ins!(SEI),
        ins!(TXS),
        ins!(CLD),
        // ins!(JSR), 0x02, 0xfd,
        // ins!(BNE), 0xef, 0xfc,
        // ins!(JMP_ABS), 0x00, 0x80,
        ins!(STX_ABS), 0x16, 0xd0,
        ins!(JSR), 0xa3, 0xfd,
        ins!(JSR), 0x50, 0xfd,
        ins!(JSR), 0x15, 0xfd,
        ins!(JSR), 0x5b, 0xff,
        ins!(CLI),
        ins!(JMP_IND), 0xfc, 0xff
    ]);

    let mut cpu = Cpu::new();
    cpu.flag_set(Carry);
    cpu.flag_clear(Carry);
    cpu.execute(&mut mem)?;

    println!(
        "Flags = C:{} Z:{} I:{} D:{} B:{} V:{} N:{}",
        cpu.flag_get(Carry),
        cpu.flag_get(Zero),
        cpu.flag_get(InterruptDisable),
        cpu.flag_get(Decimal),
        cpu.flag_get(Break),
        cpu.flag_get(Overflow),
        cpu.flag_get(Negative)
    );
    println!("PC = {:#06x}", cpu.pc);
    println!("SP = {:#04x}", cpu.sp);
    println!("A = {:#04x}", cpu.a);
    println!("X = {:#04x}", cpu.x);
    println!("Y = {:#04x}", cpu.y);

    Ok(())
}
