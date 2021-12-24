    # This file contains some of the unintended instructions examples
    # found with the fuzzer for pkru manipulating instructions (e.g. wrpkru,
    # and xrstor)
    .intel_syntax noprefix

    # instruction boundaries
    # =======================

    # encodes as: 000f01EF - any two byte instruction ending with 0f also
    # triggers - there's a lot of those
    add [rdi], cl
    add edi, ebp
    int3

    # encodes as c0c00f01ef - an example three byte instruction
    rol al, 0xf
    add edi, ebp
    int3

    # all the cases above are wrkpru, show that we handle xrstor as well
    # encodes as c0c00fea29ff (0fea29 is xrstor)
    rol al, 0xf
    scasb
    sub edi, edi

    # instruction embeddings
    # ========================

    # embedded in immediate
    # encodes as 0dc60fae2b
    or eax, 0x2bae0fc6

    # a jump variant of previous
    # encodes as 0f85c50fae6d
    jnz 0x6dae0fc5

    # embedded in absolute address
    # encodes as a0ddff0fae6bae380f
    mov al, [0xf38ae6bae0fffdd]

    # embedded in address displacement
    # 6601ae000fae68
    add [rsi + 0x68ae0f00], bp

    # crossing between modrm and immediate
    # encodes as f70faeff002d
    test [rdi], 0x2d00ffae

    # prefix cases
    # =============

    # This involves a 3 byte vex prefix which changes the interpretation of the
    # following bytes.  i.e. 0f appears to be the opcode field here, but this
    # isn't a wrkpru
    vpalignr	$239, (%rcx), %xmm0, %xmm8

    # analogous case but containing an embedded xrstor 
    vtestpd	0x828(%rsi), %ymm13

    # Note that despite fuzzing, we haven't yet found examples involving either
    # 2 byte VEX (0xc5) or EVEX (0x62).  This may indicate these don't exist,
    # or the fuzzer may simply not have found them.

    # Note also that we haven't yet found an example which involves the bytes
    # of the prefix itself containing any of the problematic instruction.
