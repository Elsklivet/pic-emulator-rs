use crate::nbitnumber::{u12, u9, NumberOperations, NBitNumber};
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
    pic.program_counter = pic.program_memory.pop();
}

pub fn RETFIE(pic: &mut PIC10F200)  {
    todo!()
}

/* ALU Operation - if d is '0' store result in W, if d is '1' store result in f*/

fn store_wf(pic: &mut PIC10F200, result: u8){
    let d = pic.current_instruction.extract_d();
    if d.as_u16() == 1{
        pic.w_register = result;
    } else {
        pic.data_memory.write(pic.current_instruction.extract_f(), result);
    }
}

fn update_Z(pic: &mut PIC10F200, result: u8){
    //TODO: implement Z flag
    // if result == 0 {
    //     pic.data_memory;
    // } else {
    //     pic.data_memory.clear_z_flag();
    // }
}

fn get_f_value(pic: &mut PIC10F200) -> u8 {
    let f = pic.current_instruction.extract_f();
    return pic.data_memory.read(f)
}

pub fn MOVWF(pic: &mut PIC10F200)  {
    // f <- W, only ALU OP that does not have d bit
    let f = pic.current_instruction.extract_f();
    let w = pic.w_register;

    pic.data_memory.write(f, w);
}

pub fn CLR(pic: &mut PIC10F200)  {
    //sometimes called CLRF when d is '1' or CLRW when d is '0'
    // W/f <- 0
    update_Z(pic, 0);
    store_wf(pic, 0);
}

pub fn SUBWF(pic: &mut PIC10F200)  {
    // dest <- f - W
    let f_value = get_f_value(pic);
    let result = pic.w_register - f_value;

    update_Z(pic, result);
    store_wf(pic, result);
}

pub fn DECF(pic: &mut PIC10F200)  {
    // dest <- f - 1
    let f_value = get_f_value(pic);
    let result = f_value - 1;

    update_Z(pic, result);
    store_wf(pic, result);
}

pub fn IORWF(pic: &mut PIC10F200)  {
    let f_value = get_f_value(pic);
    let result = f_value | pic.w_register;

    update_Z(pic, result);
    store_wf(pic, result);
}

pub fn ANDWF(pic: &mut PIC10F200)  {
    // dest <- f AND W
    let f_value = get_f_value(pic);
    let result = f_value & pic.w_register;

    update_Z(pic, result);
    store_wf(pic, result);
}

pub fn XORWF(pic: &mut PIC10F200)  {
    // dest <- f XOR W
    let f_value = get_f_value(pic);
    let result = f_value ^ pic.w_register;

    update_Z(pic, result);
    store_wf(pic, result);
}

pub fn ADDWF(pic: &mut PIC10F200)  {
    // dest <- f+W 
    let f_value = get_f_value(pic);
    let result = f_value + pic.w_register;

    update_Z(pic, result);
    store_wf(pic, result);
}

pub fn MOVF(pic: &mut PIC10F200)  {
    // dest <- f
    let result = get_f_value(pic);

    update_Z(pic, result);
    store_wf(pic, result);    
}

pub fn COMF(pic: &mut PIC10F200)  {
    // dest <- bitwise NOT f
    let f_value = get_f_value(pic);
    let result = !f_value;

    update_Z(pic, result);
    store_wf(pic, result);
}

pub fn INCF(pic: &mut PIC10F200)  {
    // dest <- f + 1
    let f_value = get_f_value(pic);
    let result = f_value + 1;

    update_Z(pic, result);
    store_wf(pic, result);
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

/* Control Transfers - TWO CYCLE INSTRUCTIONS */

pub fn GOTO(pic: &mut PIC10F200)  {
    // Set the program counter PC to 
    // the 9-bit address specified by the instruction
    // at k using instruction.extract_k_goto()
    let instruction = pic.current_instruction;
    let k = instruction.extract_k_goto();
    pic.program_counter = k; //TODO: make sure that program counter does not increment at the end of the cycle
}

pub fn CALL(pic: &mut PIC10F200)  {
    //push PC + 1 onto stack and GOTO k
    pic.program_memory.push(pic.program_counter + u9::new(1));
    //mask out bit 8
    pic.current_instruction.instruction_raw = pic.current_instruction.instruction_raw & u12::new(0xEFF);
    GOTO(pic);
}

pub fn RETLW(pic: &mut PIC10F200)  {
    // W <- k then RETURN()
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