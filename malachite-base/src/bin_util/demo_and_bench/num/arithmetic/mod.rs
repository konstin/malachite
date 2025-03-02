use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    add_mul::register(runner);
    arithmetic_checked_shl::register(runner);
    arithmetic_checked_shr::register(runner);
    binomial_coefficient::register(runner);
    ceiling::register(runner);
    checked_add_mul::register(runner);
    checked_square::register(runner);
    checked_sub_mul::register(runner);
    coprime_with::register(runner);
    div_exact::register(runner);
    div_mod::register(runner);
    div_round::register(runner);
    divisible_by::register(runner);
    divisible_by_power_of_2::register(runner);
    eq_mod::register(runner);
    eq_mod_power_of_2::register(runner);
    extended_gcd::register(runner);
    factorial::register(runner);
    floor::register(runner);
    gcd::register(runner);
    is_power_of_2::register(runner);
    kronecker_symbol::register(runner);
    lcm::register(runner);
    log_base::register(runner);
    log_base_2::register(runner);
    log_base_power_of_2::register(runner);
    mod_inverse::register(runner);
    mod_is_reduced::register(runner);
    mod_add::register(runner);
    mod_mul::register(runner);
    mod_neg::register(runner);
    mod_op::register(runner);
    mod_pow::register(runner);
    mod_power_of_2::register(runner);
    mod_power_of_2_add::register(runner);
    mod_power_of_2_inverse::register(runner);
    mod_power_of_2_is_reduced::register(runner);
    mod_power_of_2_mul::register(runner);
    mod_power_of_2_pow::register(runner);
    mod_power_of_2_shl::register(runner);
    mod_power_of_2_shr::register(runner);
    mod_power_of_2_square::register(runner);
    mod_power_of_2_neg::register(runner);
    mod_power_of_2_sub::register(runner);
    mod_shl::register(runner);
    mod_shr::register(runner);
    mod_square::register(runner);
    mod_sub::register(runner);
    neg::register(runner);
    next_power_of_2::register(runner);
    overflowing_abs::register(runner);
    overflowing_add::register(runner);
    overflowing_add_mul::register(runner);
    overflowing_div::register(runner);
    overflowing_mul::register(runner);
    overflowing_neg::register(runner);
    overflowing_pow::register(runner);
    overflowing_square::register(runner);
    overflowing_sub::register(runner);
    overflowing_sub_mul::register(runner);
    parity::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    primorial::register(runner);
    root::register(runner);
    rotate::register(runner);
    round_to_multiple::register(runner);
    round_to_multiple_of_power_of_2::register(runner);
    saturating_abs::register(runner);
    saturating_add::register(runner);
    saturating_add_mul::register(runner);
    saturating_mul::register(runner);
    saturating_neg::register(runner);
    saturating_pow::register(runner);
    saturating_square::register(runner);
    saturating_sub::register(runner);
    saturating_sub_mul::register(runner);
    shl_round::register(runner);
    shr_round::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub_mul::register(runner);
    wrapping_abs::register(runner);
    wrapping_add::register(runner);
    wrapping_add_mul::register(runner);
    wrapping_div::register(runner);
    wrapping_mul::register(runner);
    wrapping_neg::register(runner);
    wrapping_pow::register(runner);
    wrapping_square::register(runner);
    wrapping_sub::register(runner);
    wrapping_sub_mul::register(runner);
    x_mul_y_to_zz::register(runner);
    xx_add_yy_to_zz::register(runner);
    xx_div_mod_y_to_qr::register(runner);
    xx_sub_yy_to_zz::register(runner);
    xxx_add_yyy_to_zzz::register(runner);
    xxx_sub_yyy_to_zzz::register(runner);
    xxxx_add_yyyy_to_zzzz::register(runner);
}

mod abs;
mod add_mul;
mod arithmetic_checked_shl;
mod arithmetic_checked_shr;
mod binomial_coefficient;
mod ceiling;
mod checked_add_mul;
mod checked_square;
mod checked_sub_mul;
mod coprime_with;
mod div_exact;
mod div_mod;
mod div_round;
mod divisible_by;
mod divisible_by_power_of_2;
mod eq_mod;
mod eq_mod_power_of_2;
mod extended_gcd;
mod factorial;
mod floor;
mod gcd;
mod is_power_of_2;
mod kronecker_symbol;
mod lcm;
mod log_base;
mod log_base_2;
mod log_base_power_of_2;
mod mod_add;
mod mod_inverse;
mod mod_is_reduced;
mod mod_mul;
mod mod_neg;
mod mod_op;
mod mod_pow;
mod mod_power_of_2;
mod mod_power_of_2_add;
mod mod_power_of_2_inverse;
mod mod_power_of_2_is_reduced;
mod mod_power_of_2_mul;
mod mod_power_of_2_neg;
mod mod_power_of_2_pow;
mod mod_power_of_2_shl;
mod mod_power_of_2_shr;
mod mod_power_of_2_square;
mod mod_power_of_2_sub;
mod mod_shl;
mod mod_shr;
mod mod_square;
mod mod_sub;
mod neg;
mod next_power_of_2;
mod overflowing_abs;
mod overflowing_add;
mod overflowing_add_mul;
mod overflowing_div;
mod overflowing_mul;
mod overflowing_neg;
mod overflowing_pow;
mod overflowing_square;
mod overflowing_sub;
mod overflowing_sub_mul;
mod parity;
mod pow;
mod power_of_2;
mod primorial;
mod root;
mod rotate;
mod round_to_multiple;
mod round_to_multiple_of_power_of_2;
mod saturating_abs;
mod saturating_add;
mod saturating_add_mul;
mod saturating_mul;
mod saturating_neg;
mod saturating_pow;
mod saturating_square;
mod saturating_sub;
mod saturating_sub_mul;
mod shl_round;
mod shr_round;
mod sign;
mod sqrt;
mod square;
mod sub_mul;
mod wrapping_abs;
mod wrapping_add;
mod wrapping_add_mul;
mod wrapping_div;
mod wrapping_mul;
mod wrapping_neg;
mod wrapping_pow;
mod wrapping_square;
mod wrapping_sub;
mod wrapping_sub_mul;
mod x_mul_y_to_zz;
mod xx_add_yy_to_zz;
mod xx_div_mod_y_to_qr;
mod xx_sub_yy_to_zz;
mod xxx_add_yyy_to_zzz;
mod xxx_sub_yyy_to_zzz;
mod xxxx_add_yyyy_to_zzzz;
