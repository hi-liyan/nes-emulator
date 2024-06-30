/// # 寻址模式
///
/// ## 参考
/// - [https://skilldrick.github.io/easy6502/#addressing](https://skilldrick.github.io/easy6502/#addressing)
/// - [https://www.nesdev.org/obelisk-6502-guide/reference.html](https://www.nesdev.org/obelisk-6502-guide/reference.html)
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    Accumulator,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_X_PageCross,
    Absolute_Y,
    Absolute_Y_PageCross,
    Indirect_X,
    Indirect_Y,
    Indirect_Y_PageCross,
    NoneAddressing,
}