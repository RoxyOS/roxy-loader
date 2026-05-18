use anyhow::{Context, Result, bail};
use elfloader::ElfBinary;

pub fn parse_kernel_elf(kernel_file: &[u8]) -> Result<ElfBinary<'_>> {
    ElfBinary::new(kernel_file)
        .ok()
        .context("Failed to parse kernel elf")
}

pub fn validate_kernel_elf(kernel_elf: &ElfBinary<'_>) -> Result<()> {
    if kernel_elf.is_pie() {
        bail!("PIE kernels are not supported.");
    }

    Ok(())
}

pub fn kernel_entry_point(kernel_elf: &ElfBinary<'_>) -> u64 {
    kernel_elf.entry_point()
}

#[cfg(test)]
mod tests {
    use alloc::{string::ToString, vec, vec::Vec};

    use super::{kernel_entry_point, parse_kernel_elf, validate_kernel_elf};

    const ELF64_HEADER_SIZE: usize = 64;
    const PROGRAM_HEADER_SIZE: usize = 56;
    const PT_DYNAMIC: u32 = 2;
    const DT_NULL: i64 = 0;
    const DT_FLAGS_1: i64 = 0x6ffffffb;
    const DF_1_PIE: u64 = 0x0800_0000;

    #[test]
    fn parse_kernel_elf_rejects_non_elf_bytes() {
        let error = parse_kernel_elf(b"definitely-not-an-elf").unwrap_err();
        assert_eq!(error.to_string(), "Failed to parse kernel elf");
    }

    #[test]
    fn validate_kernel_elf_rejects_pie_binaries() {
        let bytes = pie_elf_bytes(0x401000);
        let elf = parse_kernel_elf(&bytes).unwrap();
        let error = validate_kernel_elf(&elf).unwrap_err();
        assert_eq!(error.to_string(), "PIE kernels are not supported.");
    }

    #[test]
    fn kernel_entry_point_returns_elf_entry_address() {
        let bytes = executable_elf_bytes(0x401000);
        let elf = parse_kernel_elf(&bytes).unwrap();

        validate_kernel_elf(&elf).unwrap();
        assert_eq!(kernel_entry_point(&elf), 0x401000);
    }

    fn executable_elf_bytes(entry_point: u64) -> Vec<u8> {
        let mut bytes = vec![0_u8; ELF64_HEADER_SIZE];
        write_elf_header(&mut bytes, entry_point, 0, 0);
        bytes
    }

    fn pie_elf_bytes(entry_point: u64) -> Vec<u8> {
        let phoff = ELF64_HEADER_SIZE as u64;
        let dyn_offset = (ELF64_HEADER_SIZE + PROGRAM_HEADER_SIZE) as u64;
        let dyn_size = 32_u64;

        let mut bytes = vec![0_u8; dyn_offset as usize + dyn_size as usize];
        write_elf_header(&mut bytes, entry_point, phoff, 1);
        write_program_header(
            &mut bytes[ELF64_HEADER_SIZE..ELF64_HEADER_SIZE + PROGRAM_HEADER_SIZE],
            dyn_offset,
            dyn_size,
        );
        write_dynamic_entry(
            &mut bytes[dyn_offset as usize..dyn_offset as usize + 16],
            DT_FLAGS_1,
            DF_1_PIE,
        );
        write_dynamic_entry(
            &mut bytes[dyn_offset as usize + 16..dyn_offset as usize + 32],
            DT_NULL,
            0,
        );
        bytes
    }

    fn write_elf_header(bytes: &mut [u8], entry_point: u64, phoff: u64, phnum: u16) {
        bytes[..4].copy_from_slice(b"\x7FELF");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = 1;
        bytes[7] = 0;
        bytes[8..16].fill(0);

        bytes[16..18].copy_from_slice(&2_u16.to_le_bytes());
        bytes[18..20].copy_from_slice(&0x3e_u16.to_le_bytes());
        bytes[20..24].copy_from_slice(&1_u32.to_le_bytes());
        bytes[24..32].copy_from_slice(&entry_point.to_le_bytes());
        bytes[32..40].copy_from_slice(&phoff.to_le_bytes());
        bytes[40..48].copy_from_slice(&0_u64.to_le_bytes());
        bytes[48..52].copy_from_slice(&0_u32.to_le_bytes());
        bytes[52..54].copy_from_slice(&(ELF64_HEADER_SIZE as u16).to_le_bytes());
        bytes[54..56].copy_from_slice(&(PROGRAM_HEADER_SIZE as u16).to_le_bytes());
        bytes[56..58].copy_from_slice(&phnum.to_le_bytes());
        bytes[58..60].copy_from_slice(&0_u16.to_le_bytes());
        bytes[60..62].copy_from_slice(&0_u16.to_le_bytes());
        bytes[62..64].copy_from_slice(&0_u16.to_le_bytes());
    }

    fn write_program_header(bytes: &mut [u8], offset: u64, size: u64) {
        bytes[0..4].copy_from_slice(&PT_DYNAMIC.to_le_bytes());
        bytes[4..8].copy_from_slice(&0_u32.to_le_bytes());
        bytes[8..16].copy_from_slice(&offset.to_le_bytes());
        bytes[16..24].copy_from_slice(&0_u64.to_le_bytes());
        bytes[24..32].copy_from_slice(&0_u64.to_le_bytes());
        bytes[32..40].copy_from_slice(&size.to_le_bytes());
        bytes[40..48].copy_from_slice(&size.to_le_bytes());
        bytes[48..56].copy_from_slice(&8_u64.to_le_bytes());
    }

    fn write_dynamic_entry(bytes: &mut [u8], tag: i64, value: u64) {
        bytes[0..8].copy_from_slice(&tag.to_le_bytes());
        bytes[8..16].copy_from_slice(&value.to_le_bytes());
    }
}
