
use std::arch::asm;
fn print_stack_trace() {
    println!("=== Stack trace from inline asm ===");

    unsafe {
        let mut rbp: usize;
        asm!("mov {}, rbp", out(reg) rbp);

        let max_depth = 64;
        for _ in 0..max_depth {
            if rbp == 0 || rbp % std::mem::size_of::<usize>() != 0 {
                break;
            }

            // return address 是 rbp + 8
            let pc = *((rbp + 8) as *const usize);
            let sp = rbp + 16; // 模擬調用後進入 callee 時的 SP（再下去是 local var 區）

            println!("Program counter: 0x{:016x}", pc);
            println!("Stack pointer:   0x{:016x}", sp);
            println!();

            // 下一層的 frame base pointer
            rbp = *(rbp as *const usize);
        }
    }

    println!("=== End ===\n");
}

fn level3() {
    print_stack_trace();
}

fn level2() {
    level3();
}

fn level1() {
    level2();
}

fn main() {
    println!("main function");
    print_stack_trace();
    println!("level3");
    level1();
}
