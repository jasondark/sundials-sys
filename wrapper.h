#include <nvector/nvector_serial.h>
#include <sunlinsol/sunlinsol_band.h>
#include <sunlinsol/sunlinsol_spbcgs.h>
#include <sunlinsol/sunlinsol_spgmr.h>
#include <sunlinsol/sunlinsol_dense.h>
#include <sunlinsol/sunlinsol_pcg.h>
#include <sunlinsol/sunlinsol_spfgmr.h>
#include <sunlinsol/sunlinsol_sptfqmr.h>
#include <sunmatrix/sunmatrix_band.h>
#include <sunmatrix/sunmatrix_dense.h>
#include <sunmatrix/sunmatrix_sparse.h>
#include <sunnonlinsol/sunnonlinsol_fixedpoint.h>
#include <sunnonlinsol/sunnonlinsol_newton.h>

#if USE_OPENMP
#include <nvector/nvector_openmp.h>
#endif

#if USE_CVODE
#include <cvode/cvode.h>
#endif

#if USE_CVODES
#include <cvodes/cvodes.h>
#endif

#if USE_IDA
#include <ida/ida.h>
#endif

#if USE_IDAS
#include <idas/idas.h>
#endif

#if USE_KINSOL
#include <kinsol/kinsol.h>
#endif

