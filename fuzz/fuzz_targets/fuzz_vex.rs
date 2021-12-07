// Targetted fuzzing of the single instruction wrapper case where the wrapper
// instruction uses the a VEX or EVEX prefix.
#![no_main]
use libfuzzer_sys::fuzz_target;

// 3 byte VEX
const C4: u8 = 0xC4;
// 2 byte VEX
const C5: u8 = 0xC5;
// EVEX (4 bytes)
const B62: u8 = 0x62;

// data format: raw bytes for the instruction stream
fuzz_target!(|data: &[u8]| {
    if data.len() > 15 {
        return;
    }

    let vec = &*inst_decode_search::PROBLEMATIC_BYTE_SEQUENCES;

    // Try to construct cases where either a) the problematic bytes cross
    // between prefix and opcode, or b) the prefix changes the interpretation
    // of the following opcode bytes.
    for (which, prefix) in [C4, C5, B62].iter().enumerate() {
        let payload_length = match which {
            0 => 2,
            1 => 1,
            2 => 3,
            _ => unreachable!(),
        };
        for v in vec {
            let mut tmp: Vec<u8> = Vec::with_capacity(data.len() + 4);
            for i in 0..=std::cmp::min(data.len(), payload_length) {
                tmp.clear();
                tmp.push(*prefix);
                tmp.extend(&data[0..i]);
                tmp.extend(v);
                tmp.extend(&data[i..]);
                inst_decode_search::try_decode_one(&tmp, &v);
            }
        }
    }
});
