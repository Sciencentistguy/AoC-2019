#pragma once

#include "common.h"

struct day_12 {
    struct instruction_t {
        char opcode;
        ulong operand;

        instruction_t(char opcode, ulong operand);
    };
    const std::vector<instruction_t> input;

    day_12();

    void part_one() const;

    void part_two() const;
};
