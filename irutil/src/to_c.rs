use core::advance::ir::{IR, IROp};

pub fn ir_to_c(ir: &IR) -> String {
    let IR { pointer, opcode, source_range: _ } = ir;
    match opcode {
        IROp::Breakpoint => format!("// breakpoint"),
        IROp::Add(v) => format!("mem[{pointer}] {}= {};", if (*v as i8) < 0 { '-' } else { '+' }, (*v as i8).abs()),
        IROp::Set(v) => format!("mem[{pointer}] = {v};"),
        IROp::Shift(size) => format!("while (mem[{pointer}]) {{ mem {}= {}; }}", if *size < 0 { '-' } else { '+' }, size.abs()),
        IROp::MulAndSetZero(dests) => {
            let mut str = String::new();
            str.push_str(&format!("if (mem[{pointer}]) {{ "));
            for (dest_p, dest_v) in dests {
                str.push_str(&format!("mem[{dest_p}] += mem[{pointer}] * {dest_v}; "));
            }
            str.push_str(&format!("mem[{pointer}] = 0; }}"));
            str
        }
        IROp::MovesAndSetZero(dests) => {
            let mut str = String::new();
            str.push_str(&format!("if (mem[{pointer}]) {{ "));
            for (dest_p, dest_positive) in dests {
                str.push_str(&format!("mem[{dest_p}] {}= mem[{pointer}]; ", if *dest_positive { '+' } else { '-' }));
            }
            str.push_str(&format!("mem[{pointer}] = 0; }}"));
            str
        }
        IROp::In => format!("mem[{pointer}] = getchar();"),
        IROp::Out => format!("putchar(mem[{pointer}]);"),
        IROp::LoopStart(_) => format!("while (mem[{pointer}]) {{"),
        IROp::LoopEnd(_) => format!("}}"),
        IROp::LoopEndWithOffset(_, size) => format!("mem {}= {}; }}", if *size < 0 { '-' } else { '+' }, size.abs()),
        IROp::End => format!("return;"),
    }
}

pub fn irs_to_c(irs: &[IR]) -> String {
    let mut str = String::new();
    str.push_str(&format!("void main(void) {{\n"));
    str.push_str(&format!("    char buf[65536];\n"));
    str.push_str(&format!("    char* mem = buf;\n"));
    let mut indent = 1usize;
    for ir in irs {
        if let IROp::LoopEnd(..) = ir.opcode { indent -= 1; }
        if let IROp::LoopEndWithOffset(..) = ir.opcode { indent -= 1; }

        str.push_str(&"    ".repeat(indent));
        str.push_str(&ir_to_c(ir));
        str.push('\n');

        if let IROp::LoopStart(..) = ir.opcode { indent += 1; }
    }
    str.push('}');

    str
}
