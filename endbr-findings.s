    # This file contains some of the unintended instructions examples
    # found with the fuzzer for endbr64
    .intel_syntax noprefix

    # instruction boundaries
    # =======================

    # "0d0af30ff30f1efa"
    or eax, -0xcf00cf6
    nop edx

    # "002425ecf30f1efa"
    add [0x1e0ff3ec], ah
    cli

    # there should be an sti variant here, but not yet found

    # instruction embeddings
    # ========================

    # The prefixed nop variants
    # f30f1efa
    nop edx
    # 66f30f1efa
    nop dx

    # immediate embeddings
    # 05f30f1efa
    add eax, -0x5e1f00d

    # displacement embedding
    # 03a421f30f1efa
    add esp, [rcx - 0x5e1f00d]

    # absolute address (this is probably most likely in practice)
    # a3a0f30f1efa0f1efa
    mov [0xfa1e0ffa1e0ff3a0], eax

    # modrm embedding
    # 63a4f30f1efa27:
    movsxd rsp, [rbx + rsi * 8 + 0x27fa1e0f]

    # field crossings
    # 69f30f1efa3a
    imul esi, ebx, 0x3afa1e0f

    # 8080f3f30f1efa
    add [rax + 0x1e0ff3f3], -0x6

    #8173f30f1efa0f
    xor [rbx - 0xd], 0xffa1e0f

    # 66695cf30f1efa
    imul bx, [rbx + rsi * 8 + 0xf], -0x5e2
    
    # 66f7878df30f1efa5e
    test [rdi + 0x1e0ff38d], 0x5efa

