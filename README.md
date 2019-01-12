# sundials-sys

A barebones `-sys` crate around the [SUNDIALS](https://computation.llnl.gov/projects/sundials) suite of ODE solvers. The system must have CMake (`cmake` dependency) and clang (`bindgen` dependency) already installed for compilation to succeed.

## License

The license and copyright information for the SUNDIALS suite can be viewed [here](https://computation.llnl.gov/projects/sundials/license). At the time of writing, it is a BSD 3-Clause license. The code specific to this crate is also made available under the BSD 3-Clause license.

## Versions
* 0.1.1 -- removal of (S) libraries from default features, addition of pthreads support if requested
* 0.1.0 -- initial `-sys` wrapper with minor tests

