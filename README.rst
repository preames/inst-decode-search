This repo contains a set of simple tools for exploring the intricacies of x86-64 instruction decoding.  This was developed when trying to understand the possible embeddings of unintended wrpkru instructions.

The interesting insight is that using a coverage based fuzzer to explore an instruction decoding library appears to get decent coverage of the encoding space without much computational effort.

The resulting fuzzer output can be used to rapidly find existence proofs for embeddings which are suspected to be possible.  A fuzzers inability to find a counter example can also increase confidence (though not prove) that a proposed analytical model is sound.

The decoder used is `iximeow's <https://github.com/iximeow>`_ `yaxpeax <https://github.com/iximeow/yaxpeax-x86>`_.  To my knowledge, there's nothing that prevents other decoders from being used for the same purpose.  I simply needed something written entirely in Rust (so the `cargo fuzz <https://github.com/rust-fuzz/cargo-fuzz>`_ handled all the coverage guided fuzzing logic for me), and iximeow happens to be a friend.

Building
--------

.. code::

   cd <checkout-location>
   rustup override set nightly
   cargo build

The tools
---------

``cargo fuzz run fuzz_single_inst``

This fuzzer attempts to find single wrapper instructions which contain the embedded instruction bytes of interest.

``cargo fuzz run fuzz_boundaries``

This fuzzer attempts to find embeddings of the instruction bytes of interest which cross between the encoding of two or more other instructions.  

``cargo fuzz run fuzz_vex``

This fuzzer is a variant of ``fuzz_single_inst`` which is strongly biased towards exploring sequences which begin with a VEX or EVEX prefix.  This was motivated by a suspicion that there existed a ``wrpkru`` embedding which reused prefix bytes; that suspicion was quickly confirmed.

``cargo run``

The main binary built from the top level package isn't particular interesting.  It is simply an brute force search of particular subsets of the search space.  As you'd imagine, this scales incredibly badly and is mostly useful for sanity checking that the fuzzers are working properly.

I'll note that you could write a much more sophisticated brute forcer which tried to take advantage of space pruning resulting from successful prefix parses.  It was while thinking about writing such a thing (and how to convince myself it was actually correct), that I stumbled across the idea of using fuzzing.

Configuration
-------------

To change the searched for unintended instruction, update ``compute_inst_list`` in ``src/lib.rs`` to enumerate all possible encodings of the instruction or instructions of interest.

Disclaimers
-----------
This is research code - written solely to help me understand a new problem space.  I was completely new to Rust at the time, so the resulting code will likely make you shudder.

This depends on a particular older version of yaxpeax.  This simply happened to be the version current when I wrote the code originally.

Finally, this is a code drop.  There's likely to never be further activity on this repo.  The project it was written for is wrapped up, and it already served its purpose in helping to understand a new problem space anyway.

Sponsorship
-----------
This work was done under contract with `Immunant <https://immunant.com/>`_, and was funded by the grant whose acknowledgement follows.

This material is based upon work supported by the Defense Advanced Research Projects Agency (DARPA) Small Business Technology Transfer (STTR) Program Office under Contract No. W31P4Q-20-C-0052. Any opinions, findings and conclusions or recommendations expressed in this material are those of the author(s) and do not necessarily reflect the views of the DARPA STTR Program Office.
