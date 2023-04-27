use crate::nbitnumber::{u9, NumberOperations, NBitNumber};
use crate::pic::PIC10F200;



pub fn HALT(pic: &mut PIC10F200)  {
    todo!()
    //halt the program counter
}

/* Miscellaneous */

pub fn NOP(pic: &mut PIC10F200)  {
    //Do nothing
    return;
}

pub fn OPTION(pic: &mut PIC10F200)  {
    todo!()
}

pub fn SLEEP(pic: &mut PIC10F200)  {
    todo!()
}

pub fn CLRWDT(pic: &mut PIC10F200)  {
    todo!()
}

pub fn TRIS(pic: &mut PIC10F200)  {
    todo!()
}

pub fn MOVLB(pic: &mut PIC10F200)  {
    todo!()
}

pub fn RETURN(pic: &mut PIC10F200)  {
    //pop the stack and move the value to the program counter
    todo!()
}

pub fn RETFIE(pic: &mut PIC10F200)  {
    todo!()
}

/* ALU Operation */

pub fn MOVWF(pic: &mut PIC10F200)  {
    // f <- W
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let w = pic.w_register;

    pic.data_memory.write(f, w);
}

pub fn CLR(pic: &mut PIC10F200)  {
    todo!()
}

pub fn SUBWF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn DECF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn IORWF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn ANDWF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn XORWF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn ADDWF(pic: &mut PIC10F200)  {
    // dest ← f+W 
    let w = pic.w_register;
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let f_value = pic.data_memory.read(f);
    let result = f_value + w;

    pic.data_memory.write(f, result);
}

pub fn MOVF(pic: &mut PIC10F200)  {
    // The contents of register ‘f’ are
    // moved to destination ‘d’. If ‘d’ is ‘0’,
    // destination is the W register. If ‘d’
    // is ‘1’, the destination is file
    // register ‘f’. ‘d’ = 1 is useful as a
    // test of a file register, since status
    // flag Z is affected.
    
    // Move the contents of the f register to the
    // dest register
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let d = instruction.extract_d();
    let f_value = pic.data_memory.read(f);

    if d.as_u16() == 0 {
        pic.w_register = f_value;
    } else {
        pic.data_memory.write(f, f_value);
    }
    
}

pub fn COMF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn INCF(pic: &mut PIC10F200)  {
   // let register = pic.current_instruction.extract_f();
   // pic.data_memory.read(r)
}

pub fn DECFSZ(pic: &mut PIC10F200)  {
    todo!()
}

pub fn RRF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn RLF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn SWAPF(pic: &mut PIC10F200)  {
    todo!()
}

pub fn INCFSZ(pic: &mut PIC10F200)  {
    todo!()
}

/* Bit Operation */

pub fn BCF(pic: &mut PIC10F200)  {
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let b = instruction.extract_b();
    let f_value = pic.data_memory.read(f);
    let result: u8 = f_value & !(1 << b.as_u16());

    pic.data_memory.write(f, result);
}

pub fn BSF(pic: &mut PIC10F200)  {
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let b = instruction.extract_b();
    let f_value = pic.data_memory.read(f);
    let result: u8 = f_value | (1 << b.as_u16());

    pic.data_memory.write(f, result);
}

pub fn BTFSC(pic: &mut PIC10F200)  {
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let b = instruction.extract_b();
    let f_value = pic.data_memory.read(f);
    let result: u8 = f_value & (1 << b.as_u16());

    if result == 0 {
        // Skip the next instruction
        pic.program_counter = pic.program_counter + u9::new(1);
    }
}

pub fn BTFSS(pic: &mut PIC10F200)  {
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let b = instruction.extract_b();
    let f_value = pic.data_memory.read(f);
    let result: u8 = f_value & (1 << b.as_u16());

    if result == 1 {
        // Skip the next instruction
        pic.program_counter = pic.program_counter + u9::new(1);
    }
}

/* Control Transfers */

pub fn GOTO(pic: &mut PIC10F200)  {
    // Set the program counter PC to 
    // the 9-bit address specified by the instruction
    // at k using instruction.extract_k_goto()
    let instruction = pic.current_instruction;
    let k = instruction.extract_k_goto();
    pic.program_counter = k;
}

pub fn CALL(pic: &mut PIC10F200)  {
    todo!()
}

pub fn RETLW(pic: &mut PIC10F200)  {
    // W <- k then return()
    MOVLW(pic);
    RETURN(pic);
}

/* Operations with W */

pub fn MOVLW(pic: &mut PIC10F200)  {
    // W <- k
    let instruction = pic.current_instruction;
    let f = instruction.extract_f();
    let k: u8 = instruction.extract_k();

    pic.data_memory.write(f, k);
}

pub fn IORLW(pic: &mut PIC10F200)  {
    todo!()
}

pub fn ANDLW(pic: &mut PIC10F200)  {
    todo!()
}

pub fn XORLW(pic: &mut PIC10F200)  {
    todo!()
}