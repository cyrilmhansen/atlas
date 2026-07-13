use std::fmt;

#[cfg(target_arch = "x86_64")]
use capstone::Capstone;
#[cfg(target_arch = "x86_64")]
use capstone::arch;
#[cfg(target_arch = "x86_64")]
use capstone::prelude::{BuildsCapstone, BuildsCapstoneSyntax};

/// One host instruction decoded from a process-local MIR code observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisassembledInstruction {
    pub offset: usize,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
}

/// Diagnostic decoding of a bounded host-function prefix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostCodeDisassembly {
    pub architecture: &'static str,
    pub instructions: Vec<DisassembledInstruction>,
    pub decoded_bytes: usize,
    pub termination: DisassemblyTermination,
    pub trailing_bytes: Vec<u8>,
}

/// Why decoding the observed span stopped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisassemblyTermination {
    Return,
    EndOfObservation,
    UndecodableByte,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisassemblyError {
    EmptyCode,
    UnsupportedHostArchitecture(&'static str),
    Engine(String),
    NoInstruction,
}

impl fmt::Display for DisassemblyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCode => formatter.write_str("cannot disassemble an empty code observation"),
            Self::UnsupportedHostArchitecture(architecture) => {
                write!(
                    formatter,
                    "host architecture {architecture} is not enabled for disassembly"
                )
            }
            Self::Engine(message) => write!(formatter, "Capstone disassembly failed: {message}"),
            Self::NoInstruction => formatter.write_str(
                "Capstone could not decode an instruction at the start of the observation",
            ),
        }
    }
}

impl std::error::Error for DisassemblyError {}

/// Decodes an observed MIR host-code prefix using offsets relative to its start.
///
/// MIR's observed span may include padding and embedded relocation data. Atlas
/// stops at the first x86 return and retains the suffix rather than silently
/// classifying it as code. This is a diagnostic boundary, not control-flow
/// reconstruction.
pub fn disassemble_host_code(code: &[u8]) -> Result<HostCodeDisassembly, DisassemblyError> {
    if code.is_empty() {
        return Err(DisassemblyError::EmptyCode);
    }
    disassemble_x86_64(code)
}

#[cfg(target_arch = "x86_64")]
fn disassemble_x86_64(code: &[u8]) -> Result<HostCodeDisassembly, DisassemblyError> {
    let capstone = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .build()
        .map_err(|error| DisassemblyError::Engine(error.to_string()))?;
    let mut decoded = capstone
        .disasm_iter(code, 0)
        .map_err(|error| DisassemblyError::Engine(error.to_string()))?;
    let mut instructions = Vec::new();
    let mut returned = false;
    while let Some(instruction) = decoded.next() {
        let mnemonic = instruction.mnemonic().unwrap_or_default().to_owned();
        let is_return = matches!(mnemonic.as_str(), "ret" | "retf");
        instructions.push(DisassembledInstruction {
            offset: instruction.address() as usize,
            bytes: instruction.bytes().to_vec(),
            mnemonic,
            operands: instruction.op_str().unwrap_or_default().to_owned(),
        });
        if is_return {
            returned = true;
            break;
        }
    }
    if instructions.is_empty() {
        return Err(DisassemblyError::NoInstruction);
    }
    let decoded_bytes = instructions
        .last()
        .map(|instruction| instruction.offset + instruction.bytes.len())
        .unwrap_or(0);
    let termination = if returned {
        DisassemblyTermination::Return
    } else if decoded_bytes == code.len() {
        DisassemblyTermination::EndOfObservation
    } else {
        DisassemblyTermination::UndecodableByte
    };

    Ok(HostCodeDisassembly {
        architecture: "x86_64",
        instructions,
        decoded_bytes,
        termination,
        trailing_bytes: code[decoded_bytes..].to_vec(),
    })
}

#[cfg(not(target_arch = "x86_64"))]
fn disassemble_x86_64(_code: &[u8]) -> Result<HostCodeDisassembly, DisassemblyError> {
    Err(DisassemblyError::UnsupportedHostArchitecture(
        std::env::consts::ARCH,
    ))
}

#[cfg(test)]
mod tests {
    use super::{DisassemblyError, DisassemblyTermination, disassemble_host_code};

    #[test]
    fn empty_observation_is_rejected() {
        assert_eq!(disassemble_host_code(&[]), Err(DisassemblyError::EmptyCode));
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn x86_64_decoding_uses_relative_offsets_and_stops_at_return() {
        let decoded =
            disassemble_host_code(&[0x48, 0x01, 0xf8, 0xc3, 0x0f]).expect("valid x86-64 prefix");
        assert_eq!(decoded.architecture, "x86_64");
        assert_eq!(decoded.decoded_bytes, 4);
        assert_eq!(decoded.termination, DisassemblyTermination::Return);
        assert_eq!(decoded.trailing_bytes, [0x0f]);
        assert_eq!(decoded.instructions[0].offset, 0);
        assert_eq!(decoded.instructions[0].mnemonic, "add");
        assert_eq!(decoded.instructions[1].offset, 3);
        assert_eq!(decoded.instructions[1].mnemonic, "ret");
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn x86_64_decoding_reports_an_undecodable_suffix() {
        let decoded =
            disassemble_host_code(&[0x90, 0x0f]).expect("valid instruction before invalid suffix");
        assert_eq!(decoded.decoded_bytes, 1);
        assert_eq!(decoded.termination, DisassemblyTermination::UndecodableByte);
        assert_eq!(decoded.trailing_bytes, [0x0f]);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn invalid_first_instruction_is_actionable() {
        assert_eq!(
            disassemble_host_code(&[0x0f]),
            Err(DisassemblyError::NoInstruction)
        );
    }
}
