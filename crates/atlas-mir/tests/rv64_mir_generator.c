#include <inttypes.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "mir-gen.h"

struct code_observation {
  const uint8_t *code;
  size_t code_length;
  size_t calls;
};

static uint8_t *guest_bytes;
static uint32_t guest_byte_length;
static int guest_memory_error;
static size_t guest_load_calls;
static size_t guest_store_calls;

static void require(int condition, const char *message) {
  if (!condition) {
    fprintf(stderr, "MIR RV64 guest probe failed: %s\n", message);
    exit(5);
  }
}

static void observe_code(MIR_context_t context, MIR_item_t function,
                         const uint8_t *code, size_t code_length,
                         void *user_data) {
  struct code_observation *observation = user_data;
  (void)context;
  (void)function;
  require(code != NULL && code_length != 0 && observation->calls == 0,
          "invalid or repeated generated-code observation");
  observation->code = code;
  observation->code_length = code_length;
  observation->calls++;
}

static void write_observed_code(const char *path,
                                const struct code_observation *observation) {
  FILE *code_file = fopen(path, "wb");
  if (code_file == NULL) {
    perror("open generated-code output");
    exit(3);
  }
  if (fwrite(observation->code, 1, observation->code_length, code_file) !=
          observation->code_length ||
      fclose(code_file) != 0) {
    fprintf(stderr, "could not write generated code to %s\n", path);
    exit(4);
  }
}

static int guest_i64_offset_valid(int64_t offset) {
  if (offset < 0 || (offset & 7) != 0 || (uint64_t)offset > guest_byte_length ||
      guest_byte_length - (uint32_t)offset < 8) {
    guest_memory_error = 1;
    return 0;
  }
  return 1;
}

static int64_t guest_load_i64(int64_t offset) {
  uint64_t bits = 0;
  size_t index;

  guest_load_calls++;
  if (!guest_i64_offset_valid(offset)) return 0;
  for (index = 0; index < 8; index++)
    bits |= (uint64_t)guest_bytes[(uint32_t)offset + index] << (index * 8);
  return (int64_t)bits;
}

static void guest_store_i64(int64_t offset, int64_t value) {
  uint64_t bits = (uint64_t)value;
  size_t index;

  guest_store_calls++;
  if (!guest_i64_offset_valid(offset)) return;
  for (index = 0; index < 8; index++)
    guest_bytes[(uint32_t)offset + index] = (uint8_t)(bits >> (index * 8));
}

static void run_add_probe(const char *code_path) {
  typedef uint64_t (*add_function_t)(uint64_t, uint64_t);
  MIR_context_t context = MIR_init();
  MIR_module_t module = MIR_new_module(context, "atlas_rv64_add_probe");
  MIR_type_t result_types[1] = {MIR_T_I64};
  MIR_item_t function = MIR_new_func(context, "add_u64", 1, result_types, 2,
                                     MIR_T_I64, "left", MIR_T_I64, "right");
  MIR_reg_t left = MIR_reg(context, "left", function->u.func);
  MIR_reg_t right = MIR_reg(context, "right", function->u.func);
  MIR_reg_t result =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "result");
  struct code_observation observation = {0};
  uint64_t value;

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

  require(observation.calls == 1,
          "scalar function was not observed exactly once");
  value = ((add_function_t)function->addr)(40, 2);
  require(value == 42, "scalar addition did not return 42");
  write_observed_code(code_path, &observation);
  printf("mir-rv64:add:%" PRIu64 ":bytes:%zu\n", value,
         observation.code_length);
  MIR_gen_finish(context);
  MIR_finish(context);
}

static int run_is_sorted(void *function_address, uint8_t *bytes,
                         uint32_t byte_length, uint32_t element_count,
                         uint32_t *first_inversion) {
  typedef int64_t (*is_sorted_function_t)(int64_t);
  int64_t result;

  if ((uint64_t)element_count * 8 != byte_length) return 1;
  guest_bytes = bytes;
  guest_byte_length = byte_length;
  guest_memory_error = 0;
  result = ((is_sorted_function_t)function_address)(element_count);
  if (guest_memory_error || result < 0 ||
      (result != 0 && (uint64_t)result >= element_count))
    return 1;
  *first_inversion = result == 0 ? UINT32_MAX : (uint32_t)result;
  return 0;
}

static void expect_is_sorted(void *function_address, uint8_t *values,
                             uint32_t count, uint32_t expected_inversion) {
  uint32_t first_inversion = 0;
  require(run_is_sorted(function_address, values, count * 8, count,
                        &first_inversion) == 0 &&
              first_inversion == expected_inversion,
          "is_sorted result differs from its correction fixture");
}

static void run_is_sorted_probe(const char *code_path) {
  MIR_context_t context = MIR_init();
  MIR_module_t module = MIR_new_module(context, "atlas_rv64_is_sorted_probe");
  MIR_type_t result_types[1] = {MIR_T_I64};
  MIR_item_t function = MIR_new_func(context, "is_sorted_i64", 1, result_types,
                                     1, MIR_T_I64, "element_count");
  MIR_item_t load_import = MIR_new_import(context, "guest_load_i64");
  MIR_item_t load_proto = MIR_new_proto(context, "guest_load_i64_proto", 1,
                                        result_types, 1, MIR_T_I64, "offset");
  MIR_reg_t count = MIR_reg(context, "element_count", function->u.func);
  MIR_reg_t index =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "index");
  MIR_reg_t left_offset =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "left_offset");
  MIR_reg_t right_offset =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "right_offset");
  MIR_reg_t left_value =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "left_value");
  MIR_reg_t right_value =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "right_value");
  MIR_label_t loop = MIR_new_label(context);
  MIR_label_t sorted = MIR_new_label(context);
  MIR_label_t inversion = MIR_new_label(context);
  struct code_observation observation = {0};
  uint8_t singleton[] = {42, 0, 0, 0, 0, 0, 0, 0};
  uint8_t sorted_values[] = {
      0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
      0,    0,    0,    0,    0,    0,    0,    0,
      0,    0,    0,    0,    0,    0,    0,    0,
      4,    0,    0,    0,    0,    0,    0,    0,
  };
  uint8_t inverted_values[] = {
      1, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0,
      4, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0,
  };
  uint32_t invalid_inversion = 123;
  size_t loads_before_invalid;

  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MOV, MIR_new_reg_op(context, index),
                               MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function, loop);
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, sorted),
                               MIR_new_reg_op(context, index),
                               MIR_new_reg_op(context, count)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_SUB,
                               MIR_new_reg_op(context, left_offset),
                               MIR_new_reg_op(context, index),
                               MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL,
                               MIR_new_reg_op(context, left_offset),
                               MIR_new_reg_op(context, left_offset),
                               MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4,
                                    MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, left_value),
                                    MIR_new_reg_op(context, left_offset)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL,
                               MIR_new_reg_op(context, right_offset),
                               MIR_new_reg_op(context, index),
                               MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4,
                                    MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, right_value),
                                    MIR_new_reg_op(context, right_offset)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGT,
                               MIR_new_label_op(context, inversion),
                               MIR_new_reg_op(context, left_value),
                               MIR_new_reg_op(context, right_value)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_ADD, MIR_new_reg_op(context, index),
                               MIR_new_reg_op(context, index),
                               MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_JMP, MIR_new_label_op(context, loop)));
  MIR_append_insn(context, function, inversion);
  MIR_append_insn(context, function,
                  MIR_new_ret_insn(context, 1, MIR_new_reg_op(context, index)));
  MIR_append_insn(context, function, sorted);
  MIR_append_insn(context, function,
                  MIR_new_ret_insn(context, 1, MIR_new_int_op(context, 0)));
  MIR_finish_func(context);
  MIR_finish_module(context);
  MIR_load_external(context, "guest_load_i64", guest_load_i64);
  MIR_load_module(context, module);
  MIR_gen_init(context);
  MIR_gen_set_optimize_level(context, 2);
  MIR_gen_set_code_observer(context, observe_code, &observation);
  MIR_link(context, MIR_set_gen_interface, NULL);

  require(observation.calls == 1,
          "is_sorted function was not observed exactly once");
  guest_load_calls = 0;
  expect_is_sorted(function->addr, NULL, 0, UINT32_MAX);
  expect_is_sorted(function->addr, singleton, 1, UINT32_MAX);
  expect_is_sorted(function->addr, sorted_values, 4, UINT32_MAX);
  expect_is_sorted(function->addr, inverted_values, 4, 2);
  require(guest_load_calls == 10,
          "valid fixtures performed an unexpected load count");
  loads_before_invalid = guest_load_calls;
  require(run_is_sorted(function->addr, singleton, 7, 1, &invalid_inversion) != 0 &&
              invalid_inversion == 123 &&
              guest_load_calls == loads_before_invalid,
          "invalid span was not rejected before generated execution");

  write_observed_code(code_path, &observation);
  printf("mir-rv64:is_sorted:cases:4:loads:%zu:bytes:%zu\n",
         guest_load_calls, observation.code_length);
  MIR_gen_finish(context);
  MIR_finish(context);
}

static int run_reverse(void *function_address, uint8_t *bytes,
                       uint32_t byte_length, uint32_t element_count) {
  typedef void (*reverse_function_t)(int64_t);

  if ((uint64_t)element_count * 8 != byte_length) return 1;
  guest_bytes = bytes;
  guest_byte_length = byte_length;
  guest_memory_error = 0;
  ((reverse_function_t)function_address)(element_count);
  return guest_memory_error;
}

static void run_reverse_probe(const char *code_path) {
  MIR_context_t context = MIR_init();
  MIR_module_t module = MIR_new_module(context, "atlas_rv64_reverse_probe");
  MIR_item_t function = MIR_new_func(context, "reverse_i64", 0, NULL, 1,
                                     MIR_T_I64, "element_count");
  MIR_item_t load_import = MIR_new_import(context, "guest_load_i64");
  MIR_item_t load_proto = MIR_new_proto(
      context, "guest_load_i64_proto", 1, (MIR_type_t[]){MIR_T_I64}, 1,
      MIR_T_I64, "offset");
  MIR_item_t store_import = MIR_new_import(context, "guest_store_i64");
  MIR_item_t store_proto =
      MIR_new_proto(context, "guest_store_i64_proto", 0, NULL, 2, MIR_T_I64,
                    "offset", MIR_T_I64, "value");
  MIR_reg_t count = MIR_reg(context, "element_count", function->u.func);
  MIR_reg_t left =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "left");
  MIR_reg_t right =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "right");
  MIR_reg_t left_offset =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "left_offset");
  MIR_reg_t right_offset =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "right_offset");
  MIR_reg_t left_value =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "left_value");
  MIR_reg_t right_value =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "right_value");
  MIR_label_t loop = MIR_new_label(context);
  MIR_label_t finish = MIR_new_label(context);
  struct code_observation observation = {0};
  uint8_t singleton[] = {42, 0, 0, 0, 0, 0, 0, 0};
  uint8_t even_values[] = {
      1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
      3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
  };
  const uint8_t even_original[] = {
      1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
      3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
  };
  const uint8_t even_reversed[] = {
      4, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
      2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
  };
  uint8_t odd_values[] = {
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
      0,    0,    0,    0,    0,    0,    0,    0,
      1,    0,    0,    0,    0,    0,    0,    0,
      2,    0,    0,    0,    0,    0,    0,    0,
      3,    0,    0,    0,    0,    0,    0,    0,
  };
  const uint8_t odd_reversed[] = {
      3,    0,    0,    0,    0,    0,    0,    0,
      2,    0,    0,    0,    0,    0,    0,    0,
      1,    0,    0,    0,    0,    0,    0,    0,
      0,    0,    0,    0,    0,    0,    0,    0,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
  };
  uint8_t invalid_before[sizeof(singleton)];
  size_t loads_before_invalid;
  size_t stores_before_invalid;

  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MOV, MIR_new_reg_op(context, left),
                               MIR_new_int_op(context, 0)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_SUB, MIR_new_reg_op(context, right),
                               MIR_new_reg_op(context, count),
                               MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function, loop);
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, finish),
                               MIR_new_reg_op(context, left),
                               MIR_new_reg_op(context, right)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL,
                               MIR_new_reg_op(context, left_offset),
                               MIR_new_reg_op(context, left),
                               MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL,
                               MIR_new_reg_op(context, right_offset),
                               MIR_new_reg_op(context, right),
                               MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4,
                                    MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, left_value),
                                    MIR_new_reg_op(context, left_offset)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4,
                                    MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, right_value),
                                    MIR_new_reg_op(context, right_offset)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4,
                                    MIR_new_ref_op(context, store_proto),
                                    MIR_new_ref_op(context, store_import),
                                    MIR_new_reg_op(context, left_offset),
                                    MIR_new_reg_op(context, right_value)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4,
                                    MIR_new_ref_op(context, store_proto),
                                    MIR_new_ref_op(context, store_import),
                                    MIR_new_reg_op(context, right_offset),
                                    MIR_new_reg_op(context, left_value)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_ADD, MIR_new_reg_op(context, left),
                               MIR_new_reg_op(context, left),
                               MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_SUB, MIR_new_reg_op(context, right),
                               MIR_new_reg_op(context, right),
                               MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_JMP, MIR_new_label_op(context, loop)));
  MIR_append_insn(context, function, finish);
  MIR_append_insn(context, function, MIR_new_ret_insn(context, 0));
  MIR_finish_func(context);
  MIR_finish_module(context);
  MIR_load_external(context, "guest_load_i64", guest_load_i64);
  MIR_load_external(context, "guest_store_i64", guest_store_i64);
  MIR_load_module(context, module);
  MIR_gen_init(context);
  MIR_gen_set_optimize_level(context, 2);
  MIR_gen_set_code_observer(context, observe_code, &observation);
  MIR_link(context, MIR_set_gen_interface, NULL);

  require(observation.calls == 1,
          "reverse function was not observed exactly once");
  guest_load_calls = 0;
  guest_store_calls = 0;
  require(run_reverse(function->addr, NULL, 0, 0) == 0,
          "empty reverse failed");
  require(run_reverse(function->addr, singleton, sizeof(singleton), 1) == 0,
          "singleton reverse failed");
  require(run_reverse(function->addr, even_values, sizeof(even_values), 4) == 0 &&
              memcmp(even_values, even_reversed, sizeof(even_values)) == 0,
          "even reverse differs from its correction fixture");
  require(run_reverse(function->addr, even_values, sizeof(even_values), 4) == 0 &&
              memcmp(even_values, even_original, sizeof(even_values)) == 0,
          "second even reverse did not restore the input");
  require(run_reverse(function->addr, odd_values, sizeof(odd_values), 5) == 0 &&
              memcmp(odd_values, odd_reversed, sizeof(odd_values)) == 0,
          "odd reverse differs from its correction fixture");
  require(guest_load_calls == 12 && guest_store_calls == 12,
          "valid reverse fixtures performed unexpected import counts");
  memcpy(invalid_before, singleton, sizeof(singleton));
  loads_before_invalid = guest_load_calls;
  stores_before_invalid = guest_store_calls;
  require(run_reverse(function->addr, singleton, 7, 1) != 0 &&
              memcmp(singleton, invalid_before, sizeof(singleton)) == 0 &&
              guest_load_calls == loads_before_invalid &&
              guest_store_calls == stores_before_invalid,
          "invalid reverse span was not rejected before mutation");

  write_observed_code(code_path, &observation);
  printf("mir-rv64:reverse:cases:4:loads:%zu:stores:%zu:bytes:%zu\n",
         guest_load_calls, guest_store_calls, observation.code_length);
  MIR_gen_finish(context);
  MIR_finish(context);
}

int main(int argc, char **argv) {
  if (argc != 4) {
    fprintf(stderr,
            "usage: rv64_mir_generator ADD_CODE_PATH SORTED_CODE_PATH "
            "REVERSE_CODE_PATH\n");
    return 2;
  }
  run_add_probe(argv[1]);
  run_is_sorted_probe(argv[2]);
  run_reverse_probe(argv[3]);
  return 0;
}
