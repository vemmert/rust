use crate::abi::call::{FnType, ArgType};

fn classify_ret_ty<Ty>(ret: &mut ArgType<'_, Ty>) {
    ret.extend_integer_width_to(32);
}

fn classify_arg_ty<Ty>(arg: &mut ArgType<'_, Ty>) {
    arg.extend_integer_width_to(32);
}

pub fn compute_abi_info<Ty>(fty: &mut FnType<'_, Ty>) {
    if !fty.ret.is_ignore() {
        classify_ret_ty(&mut fty.ret);
    }

    for arg in &mut fty.args {
        if arg.is_ignore() { continue; }
        classify_arg_ty(arg);
    }
}
