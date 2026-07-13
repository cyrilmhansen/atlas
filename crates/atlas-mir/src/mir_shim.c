#include <stdint.h>
#include <stdarg.h>
#include <string.h>

#include "mir.h"

uint64_t atlas_mir_interpret_add_u64(uint64_t left, uint64_t right) {
  MIR_context_t context = MIR_init();
  MIR_module_t module = MIR_new_module(context, "atlas_mir_probe");
  MIR_type_t result_types[1] = {MIR_T_I64};
  MIR_item_t function = MIR_new_func(context, "add_u64", 1, result_types, 2,
                                     MIR_T_I64, "left", MIR_T_I64, "right");
  MIR_reg_t left_register = MIR_reg(context, "left", function->u.func);
  MIR_reg_t right_register = MIR_reg(context, "right", function->u.func);
  MIR_reg_t result_register =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "result");
  MIR_val_t arguments[2] = {{.u = left}, {.u = right}};
  MIR_val_t result[1];

  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_ADD,
                               MIR_new_reg_op(context, result_register),
                               MIR_new_reg_op(context, left_register),
                               MIR_new_reg_op(context, right_register)));
  MIR_append_insn(context, function,
                  MIR_new_ret_insn(context, 1,
                                   MIR_new_reg_op(context, result_register)));
  MIR_finish_func(context);
  MIR_finish_module(context);
  MIR_load_module(context, module);
  MIR_link(context, MIR_set_interp_interface, NULL);
  MIR_interp_arr(context, function, result, 2, arguments);
  MIR_finish(context);
  return result[0].u;
}

enum { ATLAS_MIR_COMPARE_TRACE_CAPACITY = 2 };

typedef struct {
  int64_t candidate;
  int64_t current;
} atlas_mir_compare_event_t;

typedef struct {
  int64_t minimum;
  uint32_t count;
  atlas_mir_compare_event_t events[ATLAS_MIR_COMPARE_TRACE_CAPACITY];
} atlas_mir_minimum_trace_t;

static atlas_mir_minimum_trace_t atlas_mir_active_trace;

static void atlas_mir_record_compare(int64_t candidate, int64_t current) {
  uint32_t index = atlas_mir_active_trace.count;

  if (index >= ATLAS_MIR_COMPARE_TRACE_CAPACITY) return;
  atlas_mir_active_trace.events[index].candidate = candidate;
  atlas_mir_active_trace.events[index].current = current;
  atlas_mir_active_trace.count = index + 1;
}

void atlas_mir_interpret_minimum3_i64(int64_t left, int64_t middle, int64_t right,
                                      atlas_mir_minimum_trace_t *trace) {
  MIR_context_t context = MIR_init();
  MIR_module_t module = MIR_new_module(context, "atlas_mir_minimum_trace");
  MIR_type_t result_types[1] = {MIR_T_I64};
  MIR_item_t function =
      MIR_new_func(context, "minimum3_i64", 1, result_types, 3, MIR_T_I64, "left", MIR_T_I64,
                   "middle", MIR_T_I64, "right");
  MIR_item_t trace_import = MIR_new_import(context, "atlas_mir_record_compare");
  MIR_item_t trace_proto =
      MIR_new_proto(context, "atlas_mir_record_compare_proto", 0, NULL, 2, MIR_T_I64,
                    "candidate", MIR_T_I64, "current");
  MIR_reg_t left_register = MIR_reg(context, "left", function->u.func);
  MIR_reg_t middle_register = MIR_reg(context, "middle", function->u.func);
  MIR_reg_t right_register = MIR_reg(context, "right", function->u.func);
  MIR_reg_t minimum_register =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "minimum");
  MIR_label_t after_middle = MIR_new_label(context);
  MIR_label_t after_right = MIR_new_label(context);
  MIR_val_t arguments[3] = {{.i = left}, {.i = middle}, {.i = right}};
  MIR_val_t result[1];

  memset(&atlas_mir_active_trace, 0, sizeof(atlas_mir_active_trace));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MOV,
                               MIR_new_reg_op(context, minimum_register),
                               MIR_new_reg_op(context, left_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_reg_op(context, middle_register),
                                    MIR_new_reg_op(context, minimum_register)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, after_middle),
                               MIR_new_reg_op(context, middle_register),
                               MIR_new_reg_op(context, minimum_register)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MOV,
                               MIR_new_reg_op(context, minimum_register),
                               MIR_new_reg_op(context, middle_register)));
  MIR_append_insn(context, function, after_middle);
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_reg_op(context, right_register),
                                    MIR_new_reg_op(context, minimum_register)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, after_right),
                               MIR_new_reg_op(context, right_register),
                               MIR_new_reg_op(context, minimum_register)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MOV,
                               MIR_new_reg_op(context, minimum_register),
                               MIR_new_reg_op(context, right_register)));
  MIR_append_insn(context, function, after_right);
  MIR_append_insn(context, function,
                  MIR_new_ret_insn(context, 1,
                                   MIR_new_reg_op(context, minimum_register)));
  MIR_finish_func(context);
  MIR_finish_module(context);
  MIR_load_external(context, "atlas_mir_record_compare", atlas_mir_record_compare);
  MIR_load_module(context, module);
  MIR_link(context, MIR_set_interp_interface, NULL);
  MIR_interp_arr(context, function, result, 3, arguments);
  atlas_mir_active_trace.minimum = result[0].i;
  *trace = atlas_mir_active_trace;
  MIR_finish(context);
}

enum {
  ATLAS_MIR_PARTITION_TRACE_CAPACITY = 128,
  ATLAS_MIR_PARTITION_LEFT_READ = 1,
  ATLAS_MIR_PARTITION_LEFT_PREDICATE = 2,
  ATLAS_MIR_PARTITION_RIGHT_READ = 3,
  ATLAS_MIR_PARTITION_RIGHT_PREDICATE = 4,
  ATLAS_MIR_PARTITION_SWAP = 5,
  ATLAS_MIR_PARTITION_BOUNDARY = 6,
};

typedef struct {
  uint32_t boundary;
  uint32_t count;
  uint32_t truncated;
  uint32_t events[ATLAS_MIR_PARTITION_TRACE_CAPACITY];
} atlas_mir_partition_trace_t;

static uint8_t *atlas_mir_guest_bytes;
static uint32_t atlas_mir_guest_byte_length;
static int atlas_mir_guest_memory_error;
static atlas_mir_partition_trace_t atlas_mir_partition_trace;

static int atlas_mir_guest_i64_offset(int64_t offset, uint32_t *index) {
  uint64_t unsigned_offset;

  if (offset < 0 || offset % 8 != 0) return 0;
  unsigned_offset = (uint64_t)offset;
  if (unsigned_offset > atlas_mir_guest_byte_length
      || atlas_mir_guest_byte_length - unsigned_offset < 8) {
    return 0;
  }
  *index = (uint32_t)unsigned_offset;
  return 1;
}

static int64_t atlas_mir_guest_load_i64(int64_t offset) {
  uint32_t index;
  uint64_t value = 0;
  uint32_t byte;

  if (!atlas_mir_guest_i64_offset(offset, &index)) {
    atlas_mir_guest_memory_error = 1;
    return 0;
  }
  for (byte = 0; byte < 8; byte++) {
    value |= (uint64_t)atlas_mir_guest_bytes[index + byte] << (byte * 8);
  }
  return (int64_t)value;
}

static void atlas_mir_guest_store_i64(int64_t offset, int64_t value) {
  uint32_t index;
  uint64_t bits = (uint64_t)value;
  uint32_t byte;

  if (!atlas_mir_guest_i64_offset(offset, &index)) {
    atlas_mir_guest_memory_error = 1;
    return;
  }
  for (byte = 0; byte < 8; byte++) {
    atlas_mir_guest_bytes[index + byte] = (uint8_t)(bits >> (byte * 8));
  }
}

static void atlas_mir_record_partition_operation(int64_t event) {
  uint32_t index = atlas_mir_partition_trace.count;

  if (index >= ATLAS_MIR_PARTITION_TRACE_CAPACITY) {
    atlas_mir_partition_trace.truncated = 1;
    return;
  }
  atlas_mir_partition_trace.events[index] = (uint32_t)event;
  atlas_mir_partition_trace.count = index + 1;
}

int atlas_mir_interpret_partition_even_i64(uint8_t *guest_bytes, uint32_t byte_length,
                                           uint32_t element_count,
                                           atlas_mir_partition_trace_t *trace) {
  MIR_context_t context;
  MIR_module_t module;
  MIR_type_t result_types[1] = {MIR_T_I64};
  MIR_item_t function;
  MIR_item_t load_import;
  MIR_item_t load_proto;
  MIR_item_t store_import;
  MIR_item_t store_proto;
  MIR_item_t trace_import;
  MIR_item_t trace_proto;
  MIR_reg_t count_register;
  MIR_reg_t left_register;
  MIR_reg_t right_register;
  MIR_reg_t index_register;
  MIR_reg_t offset_register;
  MIR_reg_t other_offset_register;
  MIR_reg_t value_register;
  MIR_reg_t other_value_register;
  MIR_reg_t mask_register;
  MIR_label_t outer_loop;
  MIR_label_t left_loop;
  MIR_label_t after_left;
  MIR_label_t right_loop;
  MIR_label_t after_right;
  MIR_label_t finish;
  MIR_val_t arguments[1] = {{.i = element_count}};
  MIR_val_t result[1];

  if ((uint64_t)element_count * 8 != byte_length) return 1;
  context = MIR_init();
  module = MIR_new_module(context, "atlas_mir_partition_offset");
  function = MIR_new_func(context, "partition_even_i64", 1, result_types, 1, MIR_T_I64,
                          "element_count");
  load_import = MIR_new_import(context, "atlas_mir_guest_load_i64");
  load_proto = MIR_new_proto(context, "atlas_mir_guest_load_i64_proto", 1, result_types, 1,
                             MIR_T_I64, "offset");
  store_import = MIR_new_import(context, "atlas_mir_guest_store_i64");
  store_proto = MIR_new_proto(context, "atlas_mir_guest_store_i64_proto", 0, NULL, 2,
                              MIR_T_I64, "offset", MIR_T_I64, "value");
  trace_import = MIR_new_import(context, "atlas_mir_record_partition_operation");
  trace_proto = MIR_new_proto(context, "atlas_mir_record_partition_operation_proto", 0, NULL,
                              1, MIR_T_I64, "event");
  count_register = MIR_reg(context, "element_count", function->u.func);
  left_register = MIR_new_func_reg(context, function->u.func, MIR_T_I64, "left");
  right_register = MIR_new_func_reg(context, function->u.func, MIR_T_I64, "right");
  index_register = MIR_new_func_reg(context, function->u.func, MIR_T_I64, "index");
  offset_register = MIR_new_func_reg(context, function->u.func, MIR_T_I64, "offset");
  other_offset_register =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "other_offset");
  value_register = MIR_new_func_reg(context, function->u.func, MIR_T_I64, "value");
  other_value_register =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "other_value");
  mask_register = MIR_new_func_reg(context, function->u.func, MIR_T_I64, "mask");
  outer_loop = MIR_new_label(context);
  left_loop = MIR_new_label(context);
  after_left = MIR_new_label(context);
  right_loop = MIR_new_label(context);
  after_right = MIR_new_label(context);
  finish = MIR_new_label(context);

  memset(&atlas_mir_partition_trace, 0, sizeof(atlas_mir_partition_trace));
  atlas_mir_guest_bytes = guest_bytes;
  atlas_mir_guest_byte_length = byte_length;
  atlas_mir_guest_memory_error = 0;
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MOV, MIR_new_reg_op(context, left_register),
                               MIR_new_int_op(context, 0)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MOV, MIR_new_reg_op(context, right_register),
                               MIR_new_reg_op(context, count_register)));
  MIR_append_insn(context, function, outer_loop);
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, finish),
                               MIR_new_reg_op(context, left_register),
                               MIR_new_reg_op(context, right_register)));
  MIR_append_insn(context, function, left_loop);
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, after_left),
                               MIR_new_reg_op(context, left_register),
                               MIR_new_reg_op(context, right_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context, ATLAS_MIR_PARTITION_LEFT_READ)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL, MIR_new_reg_op(context, offset_register),
                               MIR_new_reg_op(context, left_register), MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, value_register),
                                    MIR_new_reg_op(context, offset_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context, ATLAS_MIR_PARTITION_LEFT_PREDICATE)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_AND, MIR_new_reg_op(context, mask_register),
                               MIR_new_reg_op(context, value_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BNE, MIR_new_label_op(context, after_left),
                               MIR_new_reg_op(context, mask_register), MIR_new_int_op(context, 0)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_ADD, MIR_new_reg_op(context, left_register),
                               MIR_new_reg_op(context, left_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_JMP, MIR_new_label_op(context, left_loop)));
  MIR_append_insn(context, function, after_left);
  MIR_append_insn(context, function, right_loop);
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, after_right),
                               MIR_new_reg_op(context, left_register),
                               MIR_new_reg_op(context, right_register)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_SUB, MIR_new_reg_op(context, index_register),
                               MIR_new_reg_op(context, right_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context, ATLAS_MIR_PARTITION_RIGHT_READ)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL, MIR_new_reg_op(context, offset_register),
                               MIR_new_reg_op(context, index_register), MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, value_register),
                                    MIR_new_reg_op(context, offset_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context, ATLAS_MIR_PARTITION_RIGHT_PREDICATE)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_AND, MIR_new_reg_op(context, mask_register),
                               MIR_new_reg_op(context, value_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BEQ, MIR_new_label_op(context, after_right),
                               MIR_new_reg_op(context, mask_register), MIR_new_int_op(context, 0)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_SUB, MIR_new_reg_op(context, right_register),
                               MIR_new_reg_op(context, right_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_JMP, MIR_new_label_op(context, right_loop)));
  MIR_append_insn(context, function, after_right);
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, outer_loop),
                               MIR_new_reg_op(context, left_register),
                               MIR_new_reg_op(context, right_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context, ATLAS_MIR_PARTITION_SWAP)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL, MIR_new_reg_op(context, offset_register),
                               MIR_new_reg_op(context, left_register), MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, value_register),
                                    MIR_new_reg_op(context, offset_register)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_SUB, MIR_new_reg_op(context, index_register),
                               MIR_new_reg_op(context, right_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL,
                               MIR_new_reg_op(context, other_offset_register),
                               MIR_new_reg_op(context, index_register), MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, other_value_register),
                                    MIR_new_reg_op(context, other_offset_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, store_proto),
                                    MIR_new_ref_op(context, store_import),
                                    MIR_new_reg_op(context, offset_register),
                                    MIR_new_reg_op(context, other_value_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, store_proto),
                                    MIR_new_ref_op(context, store_import),
                                    MIR_new_reg_op(context, other_offset_register),
                                    MIR_new_reg_op(context, value_register)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_ADD, MIR_new_reg_op(context, left_register),
                               MIR_new_reg_op(context, left_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_SUB, MIR_new_reg_op(context, right_register),
                               MIR_new_reg_op(context, right_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_JMP, MIR_new_label_op(context, outer_loop)));
  MIR_append_insn(context, function, finish);
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context, ATLAS_MIR_PARTITION_BOUNDARY)));
  MIR_append_insn(context, function,
                  MIR_new_ret_insn(context, 1, MIR_new_reg_op(context, left_register)));
  MIR_finish_func(context);
  MIR_finish_module(context);
  MIR_load_external(context, "atlas_mir_guest_load_i64", atlas_mir_guest_load_i64);
  MIR_load_external(context, "atlas_mir_guest_store_i64", atlas_mir_guest_store_i64);
  MIR_load_external(context, "atlas_mir_record_partition_operation",
                    atlas_mir_record_partition_operation);
  MIR_load_module(context, module);
  MIR_link(context, MIR_set_interp_interface, NULL);
  MIR_interp_arr(context, function, result, 1, arguments);
  atlas_mir_partition_trace.boundary = (uint32_t)result[0].i;
  *trace = atlas_mir_partition_trace;
  MIR_finish(context);
  return atlas_mir_guest_memory_error;
}

enum {
  ATLAS_MIR_IS_SORTED_TRACE_CAPACITY = 128,
  ATLAS_MIR_IS_SORTED_LEFT_READ = 1,
  ATLAS_MIR_IS_SORTED_RIGHT_READ = 2,
  ATLAS_MIR_IS_SORTED_ADJACENT_COMPARE = 3,
};

typedef struct {
  uint32_t sorted;
  uint32_t first_inversion;
  uint32_t count;
  uint32_t truncated;
  uint32_t events[ATLAS_MIR_IS_SORTED_TRACE_CAPACITY];
} atlas_mir_is_sorted_trace_t;

static atlas_mir_is_sorted_trace_t atlas_mir_is_sorted_trace;

static void atlas_mir_record_is_sorted_operation(int64_t event) {
  uint32_t index = atlas_mir_is_sorted_trace.count;

  if (index >= ATLAS_MIR_IS_SORTED_TRACE_CAPACITY) {
    atlas_mir_is_sorted_trace.truncated = 1;
    return;
  }
  atlas_mir_is_sorted_trace.events[index] = (uint32_t)event;
  atlas_mir_is_sorted_trace.count = index + 1;
}

int atlas_mir_interpret_is_sorted_i64(uint8_t *guest_bytes, uint32_t byte_length,
                                      uint32_t element_count,
                                      atlas_mir_is_sorted_trace_t *trace) {
  MIR_context_t context;
  MIR_module_t module;
  MIR_type_t result_types[1] = {MIR_T_I64};
  MIR_item_t function;
  MIR_item_t load_import;
  MIR_item_t load_proto;
  MIR_item_t trace_import;
  MIR_item_t trace_proto;
  MIR_reg_t count_register;
  MIR_reg_t index_register;
  MIR_reg_t left_offset_register;
  MIR_reg_t right_offset_register;
  MIR_reg_t left_value_register;
  MIR_reg_t right_value_register;
  MIR_label_t loop;
  MIR_label_t sorted;
  MIR_label_t inversion;
  MIR_val_t arguments[1] = {{.i = element_count}};
  MIR_val_t result[1];

  if ((uint64_t)element_count * 8 != byte_length) return 1;
  context = MIR_init();
  module = MIR_new_module(context, "atlas_mir_is_sorted_offset");
  function = MIR_new_func(context, "is_sorted_i64", 1, result_types, 1, MIR_T_I64,
                          "element_count");
  load_import = MIR_new_import(context, "atlas_mir_guest_load_i64");
  load_proto = MIR_new_proto(context, "atlas_mir_guest_load_i64_proto", 1, result_types, 1,
                             MIR_T_I64, "offset");
  trace_import = MIR_new_import(context, "atlas_mir_record_is_sorted_operation");
  trace_proto = MIR_new_proto(context, "atlas_mir_record_is_sorted_operation_proto", 0, NULL,
                              1, MIR_T_I64, "event");
  count_register = MIR_reg(context, "element_count", function->u.func);
  index_register = MIR_new_func_reg(context, function->u.func, MIR_T_I64, "index");
  left_offset_register =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "left_offset");
  right_offset_register =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "right_offset");
  left_value_register =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "left_value");
  right_value_register =
      MIR_new_func_reg(context, function->u.func, MIR_T_I64, "right_value");
  loop = MIR_new_label(context);
  sorted = MIR_new_label(context);
  inversion = MIR_new_label(context);

  memset(&atlas_mir_is_sorted_trace, 0, sizeof(atlas_mir_is_sorted_trace));
  atlas_mir_guest_bytes = guest_bytes;
  atlas_mir_guest_byte_length = byte_length;
  atlas_mir_guest_memory_error = 0;
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MOV, MIR_new_reg_op(context, index_register),
                               MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function, loop);
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGE, MIR_new_label_op(context, sorted),
                               MIR_new_reg_op(context, index_register),
                               MIR_new_reg_op(context, count_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context, ATLAS_MIR_IS_SORTED_LEFT_READ)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_SUB,
                               MIR_new_reg_op(context, left_offset_register),
                               MIR_new_reg_op(context, index_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL,
                               MIR_new_reg_op(context, left_offset_register),
                               MIR_new_reg_op(context, left_offset_register),
                               MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, left_value_register),
                                    MIR_new_reg_op(context, left_offset_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context, ATLAS_MIR_IS_SORTED_RIGHT_READ)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_MUL,
                               MIR_new_reg_op(context, right_offset_register),
                               MIR_new_reg_op(context, index_register), MIR_new_int_op(context, 8)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 4, MIR_new_ref_op(context, load_proto),
                                    MIR_new_ref_op(context, load_import),
                                    MIR_new_reg_op(context, right_value_register),
                                    MIR_new_reg_op(context, right_offset_register)));
  MIR_append_insn(context, function,
                  MIR_new_call_insn(context, 3, MIR_new_ref_op(context, trace_proto),
                                    MIR_new_ref_op(context, trace_import),
                                    MIR_new_int_op(context,
                                                   ATLAS_MIR_IS_SORTED_ADJACENT_COMPARE)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_BGT, MIR_new_label_op(context, inversion),
                               MIR_new_reg_op(context, left_value_register),
                               MIR_new_reg_op(context, right_value_register)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_ADD, MIR_new_reg_op(context, index_register),
                               MIR_new_reg_op(context, index_register), MIR_new_int_op(context, 1)));
  MIR_append_insn(context, function,
                  MIR_new_insn(context, MIR_JMP, MIR_new_label_op(context, loop)));
  MIR_append_insn(context, function, inversion);
  MIR_append_insn(context, function,
                  MIR_new_ret_insn(context, 1,
                                   MIR_new_reg_op(context, index_register)));
  MIR_append_insn(context, function, sorted);
  MIR_append_insn(context, function,
                  MIR_new_ret_insn(context, 1, MIR_new_int_op(context, 0)));
  MIR_finish_func(context);
  MIR_finish_module(context);
  MIR_load_external(context, "atlas_mir_guest_load_i64", atlas_mir_guest_load_i64);
  MIR_load_external(context, "atlas_mir_record_is_sorted_operation",
                    atlas_mir_record_is_sorted_operation);
  MIR_load_module(context, module);
  MIR_link(context, MIR_set_interp_interface, NULL);
  MIR_interp_arr(context, function, result, 1, arguments);
  atlas_mir_is_sorted_trace.sorted = result[0].i == 0;
  atlas_mir_is_sorted_trace.first_inversion =
      result[0].i == 0 ? UINT32_MAX : (uint32_t)result[0].i;
  *trace = atlas_mir_is_sorted_trace;
  MIR_finish(context);
  return atlas_mir_guest_memory_error;
}
