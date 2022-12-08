use alloc::vec::Vec;
use plonky2::field::extension::quintic::QuinticExtension;
use plonky2::field::ops::Square;

use plonky2::field::extension::{Extendable, FieldExtension, Frobenius};
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct QuinticExtensionTarget([Target; 5]);

impl QuinticExtensionTarget {
    pub fn new(limbs: [Target; 5]) -> Self {
        Self(limbs)
    }

    pub fn to_target_array(&self)  -> [Target; 5] {
        self.0
    }
}

pub trait CircuitBuilderGfp5<F: RichField + Extendable<5>> {
    fn add_virtual_gfp5_target(&mut self) -> QuinticExtensionTarget;
    fn connect_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget);
    fn register_gfp5_public_input(&mut self, a: QuinticExtensionTarget);

    fn zero_gfp5(&mut self) -> QuinticExtensionTarget;
    fn one_gfp5(&mut self) -> QuinticExtensionTarget;
    fn constant_gfp5(&mut self, c: QuinticExtension<F>) -> QuinticExtensionTarget;
    fn select_gfp5(&mut self, cond: BoolTarget, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;

    fn neg_gfp5(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn add_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn sub_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn mul_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn div_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn inverse_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn any_sqrt_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn canonical_sqrt_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn sgn0_gfp5(&mut self, x: QuinticExtensionTarget) -> BoolTarget;

    fn square_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn add_many_gfp5(&mut self, terms: Vec<QuinticExtensionTarget>) -> QuinticExtensionTarget;
    fn dot_product_gfp5(&mut self, a: Vec<QuinticExtensionTarget>, b: Vec<QuinticExtensionTarget>) -> QuinticExtensionTarget;
}

pub trait PartialWitnessGfp5<F: RichField + Extendable<5>>: Witness<F> {
    fn get_gfp5_target(&self, target: QuinticExtensionTarget) -> QuinticExtension<F>;
    fn get_gfp5_targets(&self, targets: &[QuinticExtensionTarget]) -> Vec<QuinticExtension<F>>;
    fn set_gfp5_target(&mut self, target: QuinticExtensionTarget, value: QuinticExtension<F>);
    fn set_gfp5_targets(&mut self, targets: &[QuinticExtensionTarget], values: &[QuinticExtension<F>]);
}

impl<W: Witness<F>, F: RichField + Extendable<5>> PartialWitnessGfp5<F> for W {
    fn get_gfp5_target(&self, target: QuinticExtensionTarget) -> QuinticExtension<F> {
        let QuinticExtensionTarget([t0, t1, t2, t3, t4]) = target;
        QuinticExtension([
            self.get_target(t0),
            self.get_target(t1),
            self.get_target(t2),
            self.get_target(t3),
            self.get_target(t4),
        ])
    }

    fn get_gfp5_targets(&self, targets: &[QuinticExtensionTarget]) -> Vec<QuinticExtension<F>> {
        targets.iter().map(|&t| self.get_gfp5_target(t)).collect()
    }

    fn set_gfp5_target(&mut self, target: QuinticExtensionTarget, value: QuinticExtension<F>) {
        let QuinticExtensionTarget([t0, t1, t2, t3, t4]) = target;
        let [v0, v1, v2, v3, v4] = value.0;

        self.set_target(t0, v0);
        self.set_target(t1, v1);
        self.set_target(t2, v2);
        self.set_target(t3, v3);
        self.set_target(t4, v4);
    }

    fn set_gfp5_targets(&mut self, targets: &[QuinticExtensionTarget], values: &[QuinticExtension<F>]) {
        for (&t, &v) in targets.iter().zip(values.iter()) {
            self.set_gfp5_target(t, v);
        }
    }
}

impl<F: RichField + Extendable<D> + Extendable<5>, const D: usize> CircuitBuilderGfp5<F> for CircuitBuilder<F, D> {
    fn add_virtual_gfp5_target(&mut self) -> QuinticExtensionTarget {
        QuinticExtensionTarget::new([
            self.add_virtual_target(),
            self.add_virtual_target(),
            self.add_virtual_target(),
            self.add_virtual_target(),
            self.add_virtual_target(),
        ])
    }

    fn connect_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) {
        for (lhs, rhs) in a.to_target_array().into_iter().zip(b.to_target_array().into_iter()) {
            self.connect(lhs, rhs);
        }
    }

    fn register_gfp5_public_input(&mut self, a: QuinticExtensionTarget) {
        for t in a.to_target_array().into_iter() {
            self.register_public_input(t);
        }
    }

    fn zero_gfp5(&mut self) -> QuinticExtensionTarget {
        QuinticExtensionTarget::new([self.zero(); 5])
    }

    fn one_gfp5(&mut self) -> QuinticExtensionTarget {
        QuinticExtensionTarget::new([self.one(), self.zero(), self.zero(), self.zero(), self.zero()])
    }

    fn constant_gfp5(&mut self, c: QuinticExtension<F>) -> QuinticExtensionTarget {
        let QuinticExtension([c0, c1, c2, c3, c4]) = c;
        QuinticExtensionTarget::new([
            self.constant(c0),
            self.constant(c1),
            self.constant(c2),
            self.constant(c3),
            self.constant(c4),
        ])
    }

    fn select_gfp5(&mut self, cond: BoolTarget, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
        let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;
        QuinticExtensionTarget::new([
            self.select(cond, a0, b0),
            self.select(cond, a1, b1),
            self.select(cond, a2, b2),
            self.select(cond, a3, b3),
            self.select(cond, a4, b4),
        ])
    }

    fn neg_gfp5(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
        QuinticExtensionTarget::new([
            self.neg(a0),
            self.neg(a1),
            self.neg(a2),
            self.neg(a3),
            self.neg(a4),
        ])
    }

    fn add_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
        let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;
        QuinticExtensionTarget::new([
            self.add(a0, b0),
            self.add(a1, b1),
            self.add(a2, b2),
            self.add(a3, b3),
            self.add(a4, b4),
        ]) 
    }

    fn sub_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
        let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;
        QuinticExtensionTarget::new([
            self.sub(a0, b0),
            self.sub(a1, b1),
            self.sub(a2, b2),
            self.sub(a3, b3),
            self.sub(a4, b4),
        ]) 
    }


    fn mul_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
        let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;

        // c0 ← a0b0 + 3(a1b4 + a2b3 + a3b2 + a4b1)
        // c1 ← a0b1 + a1b0 + 3(a2b4 + a3b3 + a4b2)
        // c2 ← a0b2 + a1b1 + a2b0 + 3(a3b4 + a4b3)
        // c3 ← a0b3 + a1b2 + a2b1 + a3b0 + 3a4b4
        // c4 ← a0b4 + a1b3 + a2b2 + a3b1 + a4b0 

        let mut c0 = self.mul(a4, b1);
        c0 = self.mul_add(a3, b2, c0);
        c0 = self.mul_add(a2, b3, c0);
        c0 = self.mul_add(a1, b4, c0);
        c0 = self.mul_const(F::from_canonical_u64(3), c0);
        c0 = self.mul_add(a0, b0, c0);

        let mut c1 = self.mul(a4, b2);
        c1 = self.mul_add(a3, b3, c1);
        c1 = self.mul_add(a2, b4, c1);
        c1 = self.mul_const(F::from_canonical_u64(3), c1);
        c1 = self.mul_add(a1, b0, c1);
        c1 = self.mul_add(a0, b1, c1);

        let mut c2 = self.mul(a4, b3);
        c2 = self.mul_add(a3, b4, c2);
        c2 = self.mul_const(F::from_canonical_u64(3), c2);
        c2 = self.mul_add(a2, b0, c2);
        c2 = self.mul_add(a1, b1, c2);
        c2 = self.mul_add(a0, b2, c2);
        
        let mut c3 = self.mul(a4, b4);
        c3 = self.mul_const(F::from_canonical_u64(3), c3);
        c3 = self.mul_add(a3, b0, c3);
        c3 = self.mul_add(a2, b1, c3);
        c3 = self.mul_add(a1, b2, c3);
        c3 = self.mul_add(a0, b3, c3);

        let mut c4 = self.mul(a4, b0);
        c4 = self.mul_add(a3, b1, c4);
        c4 = self.mul_add(a2, b2, c4);
        c4 = self.mul_add(a1, b3, c4);
        c4 = self.mul_add(a0, b4, c4);

        QuinticExtensionTarget::new([c0, c1, c2, c3, c4])
    }

    fn div_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let quotient = self.add_virtual_gfp5_target();
        self.add_simple_generator(QuinticQuotientGenerator::new(a, b, quotient));

        let quotient_times_denominator = self.mul_gfp5(quotient, b);
        self.connect_gfp5(quotient_times_denominator, a);

        quotient
    }

    fn inverse_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let one = self.one_gfp5();
        let inverse = self.add_virtual_gfp5_target();
        self.add_simple_generator(QuinticQuotientGenerator::new(one, x, inverse));

        let should_be_one = self.mul_gfp5(inverse, x);
        self.connect_gfp5(should_be_one, one);

        inverse
    }

    fn any_sqrt_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let root_x = self.add_virtual_gfp5_target();
        self.add_simple_generator(QuinticSqrtGenerator::new(x, root_x));
        
        let should_be_x = self.square_gfp5(root_x);
        self.connect_gfp5(should_be_x, x);

        root_x
    }

    /// returns true or false indicating a notion of "sign" for gfp5.
    /// This is used to canonicalize the square root
    /// This is an implementation of the function sgn0 from the IRTF's hash-to-curve document
    /// https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-hash-to-curve-07#name-the-sgn0-function
    fn sgn0_gfp5(&mut self, x: QuinticExtensionTarget) -> BoolTarget {
        let one = self.one();
        let zero = self.zero();

        let mut sign = self.constant_bool(false);
        let mut is_zero = self.constant_bool(true);
        for limb in x.to_target_array() {
            let bit_decomp = self.split_le_base::<2>(limb, 64);

            // sign_i = x_i mod 2
            // is_zero_i = x_i == 0
            // SAFETY: targets from bit_decomp guaranteed to contain values of 0 or 1
            let sign_i = BoolTarget::new_unsafe(self.sub(one, bit_decomp[0]));
            let is_zero_i = self.is_equal(limb, zero);
            
            // sign = sign || (is_zero && sign_i)
            // is_zero = is_zero && is_zero_i

            // x or y = x + y - xy
            let is_zero_and_sign_i = self.and(is_zero_i, sign_i);
            let sign_and_is_zero_and_sign_i = self.and(sign, is_zero_and_sign_i);
            let tmp = self.mul_const_add(-F::ONE, sign_and_is_zero_and_sign_i.target, sign.target);
            sign = BoolTarget::new_unsafe(self.add(tmp, is_zero_and_sign_i.target));
            is_zero = self.and(is_zero, is_zero_i);
        }

        sign
    }

    // returns the sqrt(x) such that `sgn0(sqrt(x)) == true`
    fn canonical_sqrt_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let root_x = self.any_sqrt_gfp5(x);
        let neg_root_x = self.neg_gfp5(root_x);

        let sign = self.sgn0_gfp5(root_x);
        self.select_gfp5(sign, root_x, neg_root_x)
    }

    fn square_gfp5(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget {
        self.mul_gfp5(a, a)
    }

    fn add_many_gfp5(&mut self, terms: Vec<QuinticExtensionTarget>) -> QuinticExtensionTarget {
        let mut sum = self.zero_gfp5();
        for term in terms {
            sum = self.add_gfp5(sum, term);
        }
        sum 
    }

    fn dot_product_gfp5(&mut self, a: Vec<QuinticExtensionTarget>, b: Vec<QuinticExtensionTarget>) -> QuinticExtensionTarget {
        let mut terms = Vec::new();
        for (a, b) in a.into_iter().zip(b.into_iter()) {
            terms.push(self.mul_gfp5(a, b));
        }
        self.add_many_gfp5(terms) 
    }
}

#[derive(Debug)]
pub struct QuinticQuotientGenerator {
    numerator: QuinticExtensionTarget,
    denominator: QuinticExtensionTarget,
    quotient: QuinticExtensionTarget,
}

impl QuinticQuotientGenerator {
    pub fn new(numerator: QuinticExtensionTarget, denominator: QuinticExtensionTarget, quotient: QuinticExtensionTarget) -> Self {
        QuinticQuotientGenerator { numerator, denominator, quotient }
    }
}

impl<F: RichField + Extendable<5>> SimpleGenerator<F>
    for QuinticQuotientGenerator 
{
    fn dependencies(&self) -> Vec<Target> {
        let mut deps = self.numerator.to_target_array().to_vec();
        deps.extend(self.denominator.to_target_array());
        deps
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let numerator_limbs = self.numerator.to_target_array().map(|t| witness.get_target(t));
        let numerator = QuinticExtension::<F>::from_basefield_array(numerator_limbs);

        let denominator_limbs = self.denominator.to_target_array().map(|t| witness.get_target(t));
        let denominator = QuinticExtension::<F>::from_basefield_array(denominator_limbs);

        let quotient = numerator / denominator;
        for (lhs, rhs) in self.quotient.to_target_array().into_iter().zip(<QuinticExtension<F> as FieldExtension<5>>::to_basefield_array(&quotient).into_iter()) {
            out_buffer.set_target(lhs, rhs);
        }
    }
}

#[derive(Debug)]
pub struct QuinticSqrtGenerator {
    x: QuinticExtensionTarget,
    root_x: QuinticExtensionTarget,
}

impl QuinticSqrtGenerator {
    pub fn new(x: QuinticExtensionTarget, root_x: QuinticExtensionTarget) -> Self {
        QuinticSqrtGenerator { x, root_x }
    }
}

impl<F: RichField + Extendable<5>> SimpleGenerator<F>
    for QuinticSqrtGenerator
{
    fn dependencies(&self) -> Vec<Target> {
        self.x.to_target_array().to_vec()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let x_limbs = self.x.to_target_array().map(|t| witness.get_target(t));
        let x = QuinticExtension::<F>::from_basefield_array(x_limbs);

        let root_x = canonical_sqrt_gfp5(x).expect("not a square!");
        
        for (lhs, rhs) in self.root_x.to_target_array().into_iter().zip(<QuinticExtension<F> as FieldExtension<5>>::to_basefield_array(&root_x).into_iter()) {
            out_buffer.set_target(lhs, rhs);
        }
    }
}

/// returns true or false indicating a notion of "sign" for gfp5.
/// This is used to canonicalize the square root
/// This is an implementation of the function sgn0 from the IRTF's hash-to-curve document
/// https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-hash-to-curve-07#name-the-sgn0-function
fn gfp5_sgn0<F: RichField + Extendable<5>>(x: QuinticExtension<F>) -> bool {
    let mut sign = false;
    let mut zero = true;
    for &limb in x.0.iter() {
        let sign_i = limb.to_canonical_u64() & 1 == 0;
        let zero_i = limb == F::ZERO;
        sign = sign || (zero && sign_i);
        zero = zero && zero_i;
    }
    return sign;
}


// returns the "canoncal" square root of x, if it exists
// the "canonical" square root is the one such that `sgn0(sqrt(x)) == true`
fn canonical_sqrt_gfp5<F: RichField + Extendable<5>>(x: QuinticExtension<F>) -> Option<QuinticExtension<F>> {
    match sqrt_gfp5(x) {
        Some(root_x) => {
            if gfp5_sgn0(root_x) {
                Some(root_x)
            } else {
                Some(-root_x)
            }
        }
        None => None
    }
}

/// returns `Some(sqrt(x))` if `x` is a square in the field, and `None` otherwise
/// basically copied from here: https://github.com/pornin/ecgfp5/blob/ce059c6d1e1662db437aecbf3db6bb67fe63c716/python/ecGFp5.py#L879
fn sqrt_gfp5<F: RichField + Extendable<5>>(x: QuinticExtension<F>) -> Option<QuinticExtension<F>> {
    let v = x.exp_power_of_2(31);
    let d = x * v.exp_power_of_2(32) * v.try_inverse().unwrap_or(QuinticExtension::<F>::ZERO);
    let e = (d * d.repeated_frobenius(2)).frobenius();
    let f = e.square();

    let [x0, x1, x2, x3, x4] = x.0;
    let [f0, f1, f2, f3, f4] = f.0;
    let g = x0 * f0 + F::from_canonical_u64(3) * (x1 * f4 + x2 * f3 + x3 * f2 + x4 * f1);

    g.sqrt().map(|s| e.try_inverse().unwrap_or(QuinticExtension::<F>::ZERO) * s.into())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::field::types::{Field, Sample};
    use plonky2::field::extension::quintic::QuinticExtension;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use rand::thread_rng;

    use super::*;

    #[test]
    fn test_add() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let y_expected = GFp5::sample(&mut rng);
        let z_expected = x_expected + y_expected;

        let x = builder.constant_gfp5(x_expected);
        let y = builder.constant_gfp5(y_expected);
        let z = builder.add_gfp5(x, y);
        builder.register_gfp5_public_input(z);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_gfp5_target(z, z_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_mul() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();


        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let y_expected = GFp5::sample(&mut rng);
        let z_expected = x_expected * y_expected;

        let x = builder.constant_gfp5(x_expected);
        let y = builder.constant_gfp5(y_expected);
        let z = builder.mul_gfp5(x, y);
        builder.register_gfp5_public_input(z);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_gfp5_target(z, z_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_sub() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();


        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let y_expected = GFp5::sample(&mut rng);
        let z_expected = x_expected - y_expected;

        let x = builder.constant_gfp5(x_expected);
        let y = builder.constant_gfp5(y_expected);
        let z = builder.sub_gfp5(x, y);
        builder.register_gfp5_public_input(z);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_gfp5_target(z, z_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }
    

    #[test]
    fn test_div() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();


        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let y_expected = GFp5::sample(&mut rng);
        let z_expected = x_expected / y_expected;

        let x = builder.constant_gfp5(x_expected);
        let y = builder.constant_gfp5(y_expected);
        let z = builder.div_gfp5(x, y);
        builder.register_gfp5_public_input(z);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_gfp5_target(z, z_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_inverse_gfp5() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let x_inv_expected = x_expected.inverse();

        let x = builder.constant_gfp5(x_expected);
        let x_inv = builder.inverse_gfp5(x);
        builder.register_gfp5_public_input(x_inv);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_gfp5_target(x_inv, x_inv_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }


    #[test]
    fn test_any_sqrt_gfp5() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = GFp5::sample(&mut rng);
        let square_expected = x * x;

        let square = builder.constant_gfp5(square_expected);
        builder.any_sqrt_gfp5(square);

        let circuit = builder.build::<C>();

        let pw = PartialWitness::new();
        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }    #[test]

    fn test_canonical_sqrt_gfp5() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = GFp5::sample(&mut rng);
        let square_expected = x * x;

        let square = builder.constant_gfp5(square_expected);
        builder.canonical_sqrt_gfp5(square);

        let circuit = builder.build::<C>();

        let pw = PartialWitness::new();
        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_sqrt_gfp5_outside_circuit() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();

        for _ in 0..30 {
            let x = GFp5::sample(&mut rng);
            let square = x * x;
            let sqrt = sqrt_gfp5(square).unwrap();

            assert_eq!(sqrt * sqrt, square);
        }
    }

    #[test]
    fn test_canonical_sqrt_gfp5_outside_circuit() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type GFp5 = QuinticExtension::<F>;

        let mut rng = thread_rng();

        for _ in 0..32 {
            let x = GFp5::sample(&mut rng);
            let square = x * x;
            let sqrt = canonical_sqrt_gfp5(square).unwrap();

            assert_eq!(sqrt * sqrt, square);
            assert!(gfp5_sgn0(sqrt))
        }
    }
}
