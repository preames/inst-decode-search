// Do a brute force search for individual instructions which contain the
// embedded instruction (byte sequences) of interest.
use itertools::Itertools;

fn main() {
    let vec = &*inst_decode_search::PROBLEMATIC_BYTE_SEQUENCES;

    // As you'd expect with a brute force search, performance scales very badly.
    // The largest I've bothered with is extra_bytes=3 with wkpru.  Note that
    // you need to manually explore all values up to your max, as this tool
    // only considers exact single instruction matches.
    let extra_bytes = 2;
    println!(
        "Searching {} bytes around problematic sequences:",
        extra_bytes
    );
    for v in vec {
        println!("  {:?}", hex::encode(v));
    }
    let inst_len = vec[0].len();

    // Construct all byte sequences of length N containing the specified
    // (3 byte) subsequences.
    let mut combinations = 0;
    for bytes in (0..extra_bytes).map(|_| 0..=255).multi_cartesian_product() {
        // Don't both to search bytes after the embedded instruction as we
        // already know it decodes to 2+ instructions.
        for offset in 1..=bytes.len() {
            for inst in vec {
                let mut tmp: Vec<u8> = Vec::with_capacity(bytes.len() + inst_len);
                for (i, b) in bytes.iter().enumerate() {
                    if i == offset {
                        tmp.extend(inst);
                    }
                    tmp.push(*b);
                }
                if offset == bytes.len() {
                    tmp.extend(inst);
                }

                combinations += 1;

                // println!("{:?}", hex::encode(&tmp));
                // Note: prints only when decodes to exactly one instruction.
                inst_decode_search::try_decode_one(&tmp, &inst)
            }
        }
    }
    println!("Searched {} combinations", combinations);
}
