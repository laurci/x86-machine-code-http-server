#![allow(dead_code)]

pub const ISA_MOV_IMM32_TO_R32: u8 = 0xb8;
pub const ISA_MOV_R32_TO_RM32: u8 = 0x89;
pub const ISA_PUSH_R32: u8 = 0x50;
pub const ISA_POP_R32: u8 = 0x58;
pub const ISA_JMP_REL8: u8 = 0xeb;
pub const ISA_INT: u8 = 0xcd;

pub const MOD_REG: u8 = 0b11_000_000;

pub const REG_EAX: u8 = 0x00;
pub const REG_ECX: u8 = 0x01;
pub const REG_EDX: u8 = 0x02;
pub const REG_EBX: u8 = 0x03;
pub const REG_ESP: u8 = 0x04;
pub const REG_EBP: u8 = 0x05;
pub const REG_ESI: u8 = 0x06;
pub const REG_EDI: u8 = 0x07;
