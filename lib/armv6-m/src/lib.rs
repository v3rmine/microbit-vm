/* === Memory === */
/// Memory is addressed with 32 bits addresses
pub const MEMORY_MAX_ADDRESSABLE_ADDRESS: u32 = u32::MAX;

/* === Registers === */

/// Application Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Apsr(u32);

/// Interrupt Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Ipsr(u32);

/// Execution Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Epsr(u32);

/// List of the registers:
/// - General purpose registers R0-R12.
/// - Two Stack Pointer registers, `SP_main` and `SP_process`. These are banked versions of SP, also described as R13.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-ARM-core-registers>
/// - The Link Register, `LR` also described as R14.
/// - The Program Counter, `PC`, sometimes described as R15.
/// - Status registers for flags, execution state bits, and the current exception number.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
/// - A mask register, `PRIMASK`, used to manage the prioritization scheme for exceptions and interrupts.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-mask-register--PRIMASK>
/// - A control register, `CONTROL` that identifies the current stack.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-CONTROL-register>
///
/// Source: <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers>
#[allow(nonstandard_style)]
pub enum Register {
    R0(u32),
    R1(u32),
    R2(u32),
    R3(u32),
    R4(u32),
    R5(u32),
    R6(u32),
    R7(u32),
    R8(u32),
    R9(u32),
    R10(u32),
    R11(u32),
    R12(u32),
    /// Stack Pointer register, banked version of SP (with SpProcess, described as R13), also described as MSP
    SP_main(u32),
    /// Stack Pointer register, banked version of SP (with SpMain, described as R13), also described as PSP
    SP_process(u32),
    /// The Link Register, also described as R14
    LR(u32),
    /// The Program Counter, also described as R15
    PC(u32),
    /// A mask register, used to manage the prioritization scheme for exceptions and interrupts
    PRIMASK(u32),
    /// A control register, that identifies the current stack
    CONTROL(u32),
    /// Application Program Status Register
    APSR(Apsr),
    /// Interrupt Program Status Register
    IPSR(Ipsr),
    /// Execution Program Status Register
    EPSR(Epsr),
}

/* === Instruction set === */
/// Source: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/The-ARMv6-M-Instruction-Set>
#[allow(nonstandard_style)]
pub enum OpCode {
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADC--register->
    ADC,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADD--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADD--register->
    /// SP plus immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADD--SP-plus-immediate->
    /// SP plus register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADD--SP-plus-register->
    ADD,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADR>
    ADR,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/AND--register->
    AND,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ASR--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ASR--register->
    ASR,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/B>
    B,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BIC--register->
    BIC,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BKPT>
    BKPT,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BL>
    BL,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BLX--register->
    BLX,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BX>
    BX,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMN--register->
    CMN,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMP--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMP--register->
    CMP,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CPS>
    CPS,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CPY>
    CPY,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/DMB>
    DMB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/DSB>
    DSB,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/EOR--register->
    EOR,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ISB>
    ISB,
}
