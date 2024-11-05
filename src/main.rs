mod bytes;
mod isa;

use anyhow::Result;

use bytes::{bytes, AppendByteOrder, AsBytes, Bytes};
use isa::{
    ISA_INT, ISA_JMP_REL8, ISA_MOV_IMM32_TO_R32, ISA_MOV_R32_TO_RM32, ISA_POP_R32, ISA_PUSH_R32,
    MOD_REG, REG_EAX, REG_EBX, REG_ECX, REG_EDX, REG_ESI,
};

const AF_INET: u8 = 0x02;
const SOCK_STREAM: u8 = 0x01;

const SYS_EXIT: u32 = 0x01;
const SYS_READ: u32 = 0x03;
const SYS_WRITE: u32 = 0x04;
const SYS_CLOSE: u32 = 0x06;
const SYS_SOCKET: u32 = 0x167;
const SYS_BIND: u32 = 0x169;
const SYS_LISTEN: u32 = 0x16b;
const SYS_ACCEPT4: u32 = 0x16c;

fn main() -> Result<()> {
    let mut header = Bytes::empty();
    let mut program = Bytes::empty();

    let text_start_addr = 0x08_04_80_00u32; // linux x86 convention for text segment address
    let program_start_offset = 0x54u32; // 84 bytes
    let program_vaddr = text_start_addr + program_start_offset;

    let log_message = "http server listening on 0.0.0.0:9001\n".into_bytes();

    let html_bytes = include_bytes!("./index.html");
    let http_headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {len}\r\nX-Powered-By: X86 Machine Code, Bitch!\r\n\r\n",
        len = html_bytes.len(),
    );

    // always respond with index.html
    let http_response = http_headers.as_str().into_bytes() + Bytes::new(html_bytes.into());

    // write(STDOUT, *log_message, len(log_message))
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_WRITE.into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EBX] + 0x01u32.into_bytes_le());
    let log_message_buffer_load = program.len();
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_ECX] + 0xaabbccddu32.into_bytes_le());
    program
        .add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EDX] + (log_message.len() as u32).into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    // serverfd = socket(AF_INET, SOCK_STREAM, 0)
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_SOCKET.into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EBX] + (AF_INET as u32).into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_ECX] + (SOCK_STREAM as u32).into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EDX] + 0x00u32.into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    // push EAX (save serverfd)
    program.add(bytes![ISA_PUSH_R32 + REG_EAX]);

    // bind(serverfd, *sock_addr, 16)
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_BIND.into_bytes_le());
    program.add(bytes![ISA_POP_R32 + REG_EBX]); // copy stack value (serverfd) to EBX
    program.add(bytes![ISA_PUSH_R32 + REG_EBX]);
    let sock_addr_load = program.len();
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_ECX] + 0xaabbccddu32.into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EDX] + 16u32.into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    // listen(serverfd, 5)
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_LISTEN.into_bytes_le());
    program.add(bytes![ISA_POP_R32 + REG_EBX]); // copy stack value (serverfd) to EBX
    program.add(bytes![ISA_PUSH_R32 + REG_EBX]);
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_ECX] + 0x05u32.into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    // accept_client loop start label
    let accept_client = program.len() as isize;

    // clientfd = accept4(serverfd, null, null, 0)
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_ACCEPT4.into_bytes_le());
    program.add(bytes![ISA_POP_R32 + REG_EBX]); // copy stack value (serverfd) to EBX
    program.add(bytes![ISA_PUSH_R32 + REG_EBX]);
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_ECX] + 0x0u32.into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EDX] + 0x0u32.into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_ESI] + 0x0u32.into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    // push EAX (save clientfd)
    program.add(bytes![ISA_PUSH_R32 + REG_EAX]);

    // reqlen = read(clientfd, *read_buffer, 4096)
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_READ.into_bytes_le());
    program.add(bytes![ISA_POP_R32 + REG_EBX]); // copy stack value (clientfd) to EBX
    program.add(bytes![ISA_PUSH_R32 + REG_EBX]);
    let read_buffer_load = program.len();
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_ECX] + 0xaabbccddu32.into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EDX] + 0x1000u32.into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    // write(STDOUT, *read_buffer, reqlen)
    program.add(bytes![
        ISA_MOV_R32_TO_RM32,
        MOD_REG | REG_EAX << 3 | REG_EDX
    ]); // TODO: check this
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_WRITE.into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EBX] + 0x01u32.into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    // write(clientfd, *write_buffer, len(write_buffer))
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_WRITE.into_bytes_le());
    program.add(bytes![ISA_POP_R32 + REG_EBX]); // copy stack value (clientfd) to EBX
    program.add(bytes![ISA_PUSH_R32 + REG_EBX]);
    let write_buffer_load = program.len();
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_ECX] + 0xaabbccddu32.into_bytes_le());
    program
        .add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EDX] + (http_response.len() as u32).into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    // close(clientfd)
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_CLOSE.into_bytes_le());
    program.add(bytes![ISA_POP_R32 + REG_EBX]); // copy stack value (clientfd) to EBX
    program.add(bytes![ISA_PUSH_R32 + REG_EBX]);
    program.add(bytes![ISA_INT, 0x80]);

    // pop EAX (clientfd)
    program.add(bytes![ISA_POP_R32 + REG_EAX]); // drop stack value (clientfd)

    // jump back to accept_client
    let here = program.len() as isize;
    assert!((accept_client - here).abs() < 128);
    let jump_size = (accept_client - here) as i8 - 2;
    program.add(bytes![ISA_JMP_REL8] + jump_size.into_bytes());

    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EAX] + SYS_EXIT.into_bytes_le());
    program.add(bytes![ISA_MOV_IMM32_TO_R32 + REG_EBX] + 69u32.into_bytes_le());
    program.add(bytes![ISA_INT, 0x80]);

    let log_message_buffer = program.len();
    program.add(log_message);
    program.set_at(
        log_message_buffer_load + 1,
        (program_vaddr + log_message_buffer as u32).into_bytes_le(),
    );

    let sock_addr = program.len();
    program.add((AF_INET as u16).into_bytes().reverse()); // sin_family
    program.add(9001u16.into_bytes()); // sin_port | MSB
    program.add(bytes![0x00, 0x00, 0x00, 0x00]); // sin_addr | MSB
    program.add(bytes![0x00] * 8); // sin_zero
    program.set_at(
        sock_addr_load + 1,
        (program_vaddr + sock_addr as u32).into_bytes_le(),
    );

    let http_response_buffer = program.len();
    program.add(http_response);
    program.set_at(
        write_buffer_load + 1,
        (program_vaddr + http_response_buffer as u32).into_bytes_le(),
    );

    let program_size = program.len();
    dbg!(program_size);

    let temp_buffer = program.len();
    program.add(bytes![0x00] * 0x1000); // 4kb buffer
    program.set_at(
        read_buffer_load + 1,
        (program_vaddr + temp_buffer as u32).into_bytes_le(),
    );

    header.add(bytes![0x7f, b'E', b'L', b'F']); // magic number
    header.add(0x01u8.into()); // 32-bit
    header.add(0x01u8.into()); // little endian
    header.add(0x01u8.into()); // version 1
    header.add(0x03u8.into()); // ABI | 0x03 = Linux
    header.add(0x00u8.into()); // ABI version
    header.add(bytes![0x00] * 7); // padding

    assert!(header.len() == 0x10); // 16 bytes

    header.set_append_byte_order(AppendByteOrder::LittleEndian); // will automatically reverse the bytes

    header.add(0x02u16.into()); // executable
    header.add(0x03u16.into()); // x86
    header.add(0x01u32.into()); // ELF version 1
    header.add(program_vaddr.into()); // entry point
    header.add(0x34u32.into()); // program header offset
    header.add(0x00u32.into()); // section header offset | no section headers
    header.add(bytes![0x00] * 4); // flags
    header.add(0x34u16.into()); // header size (52 bytes)

    header.add(0x20u16.into()); // program header size (32 bytes)
    header.add(0x01u16.into()); // number of program header entries

    header.add(0x00u16.into()); // section header size | no section headers
    header.add(0x00u16.into()); // number of section header entries | no section headers
    header.add(0x00u16.into()); // section header string table index | no section headers

    assert!(header.len() == 0x34); // 52 bytes

    header.add(0x01u32.into()); // PT_LOAD | loadable segment
    header.add(program_start_offset.into()); // segment offset in file
    header.add(program_vaddr.into()); // virtual address
    header.add(0x00u32.into()); // physical address | not used
    header.add((program.len() as u32).into()); // segment size in file
    header.add((program.len() as u32).into()); // segment size in memory
    header.add((1u32 | 2u32 | 4u32).into()); // segment flags | read, write, execute
    header.add(0x1000u32.into()); // segment alignment | 4096 bytes (4kb)

    assert!(header.len() == program_start_offset as usize);

    let output = Bytes::combine(vec![header, program], AppendByteOrder::BigEndian);

    std::fs::create_dir_all("./output")?;
    std::fs::write("./output/server.bin", Vec::<u8>::from(output))?;
    Ok(())
}
