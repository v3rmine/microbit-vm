use nom::{IResult, bits};
use nom::error::Error as NomError;

use crate::abi::MemoryMutation;
use crate::{parse_bits, Armv6M};

// Source: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/PUSH>
#[derive(Debug)]
pub struct Push {
    pub m: u8,
    pub register_list: u8,
}

pub fn parse_push(i: &[u8]) -> IResult<&'_ [u8], Push> {
    bits::<_, _, NomError<(&[u8], usize)>, _, _>(
        parse_bits!(
            (0b1011010, 7u8),
            (1u8, 8u8),
            (m, register_list),
            Push { m, register_list }
        )
    )(i)
}

impl MemoryMutation<Armv6M> for Push {
    fn apply(&self, on: &mut Armv6M) {
        let address = on.sp;
        todo!()
        // for i in [0u8..7].iter() {
        //     if self.register_list >> i & 1 == 1 {
        //         on.registers[]
        //     }
        // }
    }

    fn rollback(&self, _on: &mut Armv6M) {
        todo!()
    }
}

