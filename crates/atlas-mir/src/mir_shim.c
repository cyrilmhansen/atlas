#include <stdint.h>
#include <stdarg.h>

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
