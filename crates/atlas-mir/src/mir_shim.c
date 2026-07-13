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
