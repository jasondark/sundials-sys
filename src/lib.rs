#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;
    use core::ffi::c_void;
    use std::slice;

    #[test]
    // This just tests if the most basic of all programs works. More tests to come soon.
    fn simple_ode() {
        unsafe extern "C" fn rhs(
            _t: realtype,
            y: N_Vector,
            dy: N_Vector,
            _user_data: *mut c_void,
        ) -> i32 {
            *N_VGetArrayPointer(dy) = -*N_VGetArrayPointer(y);
            return 0;
        }

        unsafe {
            let y = N_VNew_Serial(1);
            *N_VGetArrayPointer(y) = 1.0;

            let mut cvode_mem = CVodeCreate(CV_ADAMS);

            CVodeInit(cvode_mem, Some(rhs), 0.0, y);
            CVodeSStolerances(cvode_mem, 1e-6, 1e-8);

            let matrix = SUNDenseMatrix(1, 1);
            let solver = SUNDenseLinearSolver(y, matrix);

            CVodeSetLinearSolver(cvode_mem, solver, matrix);

            let mut t = 0f64;
            CVode(cvode_mem, 1.0, y, &mut t, CV_NORMAL);
            // y[0] is now exp(-1)

            let result = (*N_VGetArrayPointer(y) * 1e6) as i32;
            assert_eq!(result, 367879);

            N_VDestroy(y);
            CVodeFree(&mut cvode_mem);
            SUNLinSolFree(solver);
            SUNMatDestroy(matrix);
        }
    }
}
