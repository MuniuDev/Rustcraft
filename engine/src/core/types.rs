pub type PosScalar = i32; // Position type, for blocks
pub type SizeScalar = u32; // Extent type
pub type FpScalar = f64; // Floating point position type, for entities

// #### Validation ####

// ## Helpers
// Const max for i64
const fn i64max(a: i64, b: i64) -> i64 { [a, b][(a < b) as usize] }

// ## Validations
// SizeScalar must be able to fit Any PosScalar with eps <= 1
#[allow(unused)]
const POS_ABS_MAX: usize = i64max((PosScalar::MAX as i64).abs(), (PosScalar::MIN as i64).abs()) as usize;
const_assert!(POS_ABS_MAX <= SizeScalar::MAX as usize); 

// FpScalar must be able to fit any PosScalar with eps <= 0.1
#[allow(unused)]
const ALLOWED_EPS: FpScalar = 0.1;
const_assert_ne!(PosScalar::MAX as FpScalar, PosScalar::MAX as FpScalar + ALLOWED_EPS); 
const_assert_ne!(PosScalar::MIN as FpScalar, PosScalar::MIN as FpScalar - ALLOWED_EPS);