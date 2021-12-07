// Fuzzer which looks for instruction embeddings involving instruction boundary
// crossings (i.e. bytes from two or more instructions)
#![no_main]
use libfuzzer_sys::fuzz_target;

// data format: raw bytes for the instruction stream
fuzz_target!(|data: &[u8]| {
    if data.len() > 32 || data.len() < 4 {
        return;
    }

    // The only interesting sequences are those which contain one of the
    // problematic byte sequences.  Exit early so that the fuzzer is biased
    // away from exploring such cases.  (i.e. you get no new coverage with
    // an example which isn't interesting.)
    if !inst_decode_search::find_interesting_sequence(data).is_some() {
        return;
    }

    inst_decode_search::try_decode_multiple(data);
});
