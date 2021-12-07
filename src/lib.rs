use console::Style;
use yaxpeax_arch::*;

use num_traits::identities::Zero;

use std::fmt;

#[macro_use]
extern crate lazy_static;

// If you want to search for an alternate instruction encoding, change the
// following routine to enumerate all of the interesting byte sequences.
fn compute_inst_list() -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    // ENDBR64
    //vec.push(hex::decode("f30f1efa").unwrap());
    // wrpkru "0F01EF"
    // xrstor 0x0FAE[2|6|A][8-F]
    vec.push(hex::decode("0F01EF").unwrap());
    for c1 in &['2', '6', 'F'] {
        for c2 in &['8', '9', 'A', 'B', 'C', 'D', 'E', 'F'] {
            let mut str = String::from("0FAE");
            str.push(*c1);
            str.push(*c2);
            vec.push(hex::decode(&str).unwrap());
        }
    }
    return vec;
}

lazy_static! {
    pub static ref PROBLEMATIC_BYTE_SEQUENCES: Vec<Vec<u8>> = compute_inst_list();
}

fn has_subsequence(haystack: &[u8], needle: &[u8]) -> bool {
    find_subsequence(haystack, needle).is_some()
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

pub fn find_interesting_sequence(data: &[u8]) -> Option<&Vec<u8>> {
    let vec = &*PROBLEMATIC_BYTE_SEQUENCES;
    return vec.iter().find(|seq| has_subsequence(data, seq));
}

// This version decodes a byte stream and if the decoded stream represents
// exactly one instruction prints it to the console.  If the seqeuence is
// invalid or is not entirely consumed when parsing the single instruction
// swallows input without output.
pub fn try_decode_one(buf: &[u8], problem_seq: &[u8]) {
    try_decode_one_impl::<yaxpeax_x86::long_mode::Arch>(&buf, &problem_seq)
}

// This function is derived from decode_input below, and is thus derived from
// yaxpeax-dis which is BSD zero clause licensed.
fn try_decode_one_impl<A>(buf: &[u8], problem_seq: &[u8])
where
    A: Arch,
    A::Instruction: fmt::Display,
{
    let decoder = A::Decoder::default();
    match decoder.decode(buf.iter().cloned()) {
        Ok(inst) => {
            let parsed_len = A::Address::zero().wrapping_offset(inst.len()).to_linear();
            if parsed_len != buf.len() {
                return;
            }

            pretty_print_decode_one(buf, problem_seq, inst)
        }
        Err(_e) => {
            // ignore failed decode, not interesting
        }
    }
}

fn pretty_print_decode_one<I>(buf: &[u8], problem_seq: &[u8], inst: I)
where
    I: fmt::Display
{
    let red = Style::new().red();
    let idx = find_subsequence(buf, problem_seq)
                    .expect("problem_seq must be subsequence of buf");
    let pre = &buf[..idx];
    let post = &buf[idx + problem_seq.len()..];
    let color_hex_seq = format!("{}{}{}", 
        hex::encode(&pre),
        red.apply_to(hex::encode(&problem_seq)),
        hex::encode(&post),
    );
    println!("{:14}: {}", color_hex_seq, inst);
}

// This version decodes an exact number of instructions, greater than one.
// It only prints any of the instructions if the buffer exactly decodes
// without garbage bytes at the end.
pub fn try_decode_multiple(buf: &[u8]) {
    try_decode_multiple_impl::<yaxpeax_x86::long_mode::Arch>(&buf)
}

// This function is copied from the main.rs of yaxpeax-dis, it's left
// here for ease of debugging.  yaxpeax-dis is BSD zero clause licensed.
fn try_decode_multiple_impl<A: Arch>(buf: &[u8])
where
    A::Instruction: fmt::Display,
{
    let decoder = A::Decoder::default();
    let start = A::Address::zero();
    let mut addr = start;
    let mut output = Vec::new();
    loop {
        match decoder.decode(buf[addr.to_linear()..].iter().cloned()) {
            Ok(inst) => {
                let decoded = &buf[addr.to_linear()..]
                    [..A::Address::zero().wrapping_offset(inst.len()).to_linear()];
                if find_interesting_sequence(decoded).is_some() {
                    // Need the problematic bytes to be cross boundary to
                    // be interesting
                    return;
                }
                let t = format!("{:14}: {}", hex::encode(decoded), inst);
                // if t.contains("cli") {
                //     return;
                // }
                // if t.contains("nop edx") {
                //     return;
                // }
                output.push(t);
                addr += inst.len();
            }
            Err(_e) => {
                // failed parse
                return;
            }
        }
        if addr.to_linear() == buf.len() {
            break;
        }
    }
    if output.len() != 2 {
        return;
    }

    println!("# {:?}", hex::encode(&buf));
    for line in output {
        println!("{}", line);
    }
}

// This version decodes all instructions in region. It is mostly useful for
// debugging.
pub fn try_decode_loose(buf: &[u8]) {
    decode_input::<yaxpeax_x86::long_mode::Arch>(&buf, false)
}

// This function is copied from the main.rs of yaxpeax-dis, it's left
// here for ease of debugging. yaxpeax-dis is BSD zero clause licensed.
#[allow(dead_code)]
fn decode_input<A: Arch>(buf: &[u8], verbose: bool)
where
    A::Instruction: fmt::Display,
{
    let decoder = A::Decoder::default();
    let start = A::Address::zero();
    let mut addr = start;
    loop {
        match decoder.decode(buf[addr.to_linear()..].iter().cloned()) {
            Ok(inst) => {
                println!(
                    "{:#010x}: {:14}: {}",
                    addr.to_linear(),
                    hex::encode(
                        &buf[addr.to_linear()..]
                            [..A::Address::zero().wrapping_offset(inst.len()).to_linear()]
                    ),
                    inst
                );
                if verbose {
                    println!("  {:?}", inst);
                    if !inst.well_defined() {
                        println!("  not well-defined");
                    }
                }
                addr += inst.len();
            }
            Err(e) => {
                println!("{:#010x}: {}", addr.to_linear(), e);
                addr += A::Instruction::min_size();
            }
        }
        if addr.to_linear() >= buf.len() {
            break;
        }
    }
}
