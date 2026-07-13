#include <inttypes.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#include "mir-gen.h"

struct code_observation {
  const uint8_t *code;
  size_t code_length;
  size_t calls;
};

static void observe_code(MIR_context_t context, MIR_item_t function,
                         const uint8_t *code, size_t code_length,
                         void *user_data) {
  struct code_observation *observation = user_data;
  (void)context;
  (void)function;
  if (code == NULL || code_length == 0 || observation->calls != 0) abort();
  observation->code = code;
  observation->code_length = code_length;
  observation->calls++;
}

int main(int argc, char **argv) {
  typedef uint64_t (*add_function_t)(uint64_t, uint64_t);
  MIR_context_t context;
  MIR_module_t module;
  MIR_item_t function;
  MIR_reg_t left, right, result;
  MIR_type_t result_types[1] = {MIR_T_I64};
  struct code_observation observation = {0};
  FILE *code_file;
  uint64_t value;

  if (argc != 2) {
    fprintf(stderr, "usage: rv64_mir_generator CODE_PATH\n");
    return 2;
  }

  context = MIR_init();
  module = MIR_new_module(context, "atlas_rv64_generator_probe");
  function = MIR_new_func(context, "add_u64", 1, result_types, 2, MIR_T_I64,
                          "left", MIR_T_I64, "right");
  left = MIR_reg(context, "left", function->u.func);
  right = MIR_reg(context, "right", function->u.func);
  result = MIR_new_func_reg(context, function->u.func, MIR_T_I64, "result");
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_ADD, MIR_new_reg_op(context, result),
                               MIR_new_reg_op(context, left),
                               MIR_new_reg_op(context, right)));
  MIR_append_insn(context, function,
                  MIR_new_ret_insn(context, 1, MIR_new_reg_op(context, result)));
  MIR_finish_func(context);
  MIR_finish_module(context);
  MIR_load_module(context, module);
  MIR_gen_init(context);
  MIR_gen_set_optimize_level(context, 2);
  MIR_gen_set_code_observer(context, observe_code, &observation);
  MIR_link(context, MIR_set_gen_interface, NULL);

  if (observation.calls != 1) abort();
  value = ((add_function_t)function->addr)(40, 2);
  if (value != 42) abort();

  code_file = fopen(argv[1], "wb");
  if (code_file == NULL) {
    perror("open generated-code output");
    return 3;
  }
  if (fwrite(observation.code, 1, observation.code_length, code_file) !=
          observation.code_length ||
      fclose(code_file) != 0) {
    perror("write generated-code output");
    return 4;
  }

  printf("mir-rv64:add:%" PRIu64 ":bytes:%zu\n", value,
         observation.code_length);
  MIR_gen_finish(context);
  MIR_finish(context);
  return 0;
}
