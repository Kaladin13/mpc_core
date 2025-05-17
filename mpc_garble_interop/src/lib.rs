//! Interoperability between the mpc_core MPC engine and the Garble language.
//!
//! This crate provides helper functions for translating between the mpc_core MPC engine circuit
//! representation and the Garble language circuit representation and types.

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

pub use garble_lang::{ast::Type, literal::*, TypedFnDef, TypedProgram};

/// A mpc_core circuit together with its associated Garble types.
#[derive(Debug, Clone)]
pub struct TypedCircuit {
    /// Boolean circuit executable by the mpc_core engine.
    pub gates: mpc_core::Circuit,
    /// Typed Garble function corresponding to the mpc_core circuit.
    pub fn_def: TypedFnDef,
    /// Number of gates in the circuit as a formatted string.
    ///
    /// E.g. "79k gates (XOR: 44k, NOT: 13k, AND: 21k)"
    pub info_about_gates: String,
}

/// The role of a party in the MPC execution (evaluator or contributor).
#[derive(Debug, Clone, Copy)]
pub enum Role {
    /// The party that contributes its input to the MPC protocol.
    Contributor,
    /// The party that evaluates the circuit and the output.
    Evaluator,
}

type Result<T> = std::result::Result<T, String>;

/// Scans, parses and type-checks a Garble program.
pub fn check_program(program: &str) -> Result<TypedProgram> {
    garble_lang::check(program).map_err(|e| e.prettify(program))
}

/// Compiles the (type-checked) program, producing a circuit of gates.
///
/// Assumes that the input program has been correctly type-checked and **panics** if
/// incompatible types are found that should have been caught by the type-checker.
pub fn compile_program(prg: &TypedProgram, fn_name: &str) -> Result<TypedCircuit> {
    let (circuit, fn_def) = prg.compile(fn_name).map_err(|e| format!("{:?}", e))?;
    let info_about_gates = circuit.report_gates();
    if circuit.input_gates.len() != 2 {
        return Err("The main function is not a 2-Party function".to_string());
    }

    // Garble script semantics are as follows: input at index `i` implicitly belongs to party `i`
    // In our case, party `0` is `Party A` in mpc_core terms; likewise, party `1` is `Party B`
    let input_party_a = circuit.input_gates.first().copied().unwrap_or(0);
    let input_party_b = circuit.input_gates.get(1).copied().unwrap_or(0);

    let mut gates: Vec<mpc_core::Gate> =
        Vec::with_capacity(circuit.gates.len() + input_party_a + input_party_b);

    // here we simply resize to `clone` the respective input gates into the vec...
    gates.resize(input_party_a, mpc_core::Gate::InContrib);
    gates.resize(input_party_a + input_party_b, mpc_core::Gate::InEval);

    // as Garble and mpc_core are independent code bases right now, we must currently map
    // between the 2 type systems in this rather straight-forward way.
    for gate in circuit.gates {
        gates.push(match gate {
            garble_lang::circuit::Gate::Xor(lhs, rhs) => {
                mpc_core::Gate::Xor(lhs as mpc_core::GateIndex, rhs as mpc_core::GateIndex)
            }
            garble_lang::circuit::Gate::And(lhs, rhs) => {
                mpc_core::Gate::And(lhs as mpc_core::GateIndex, rhs as mpc_core::GateIndex)
            }
            garble_lang::circuit::Gate::Not(source) => {
                mpc_core::Gate::Not(source as mpc_core::GateIndex)
            }
        })
    }

    let output_gates = circuit
        .output_gates
        .iter()
        .map(|i| *i as mpc_core::GateIndex)
        .collect();
    let program = mpc_core::Circuit::new(gates, output_gates);

    Ok(TypedCircuit {
        gates: program,
        fn_def: fn_def.clone(),
        info_about_gates,
    })
}

/// Returns the Garble type of the input associated with the specified role.
///
/// In the case of the contributor, the result will be the type of the _first_ function parameter.
/// In the case of the evaluator, the result will be the type of the _second_ function parameter.
pub fn input_type(role: Role, fn_def: &TypedFnDef) -> &'_ Type {
    match role {
        Role::Contributor => &fn_def.params[0].ty,
        Role::Evaluator => &fn_def.params[1].ty,
    }
}

/// Parses an input string as a Garble literal.
pub fn parse_input(
    role: Role,
    prg: &TypedProgram,
    fn_def: &TypedFnDef,
    input: &str,
) -> Result<Literal> {
    let input_ty = input_type(role, fn_def);
    Literal::parse(prg, input_ty, input).map_err(|e| e.prettify(input))
}

/// Parses an input string as a Garble literal and encodes it as input bits for the mpc_core engine.
pub fn serialize_input(
    role: Role,
    prg: &TypedProgram,
    fn_def: &TypedFnDef,
    input: &str,
) -> Result<Vec<bool>> {
    let input_ty = input_type(role, fn_def);
    let input = Literal::parse(prg, input_ty, input).map_err(|e| e.prettify(input))?;
    let const_sizes = std::collections::HashMap::new();
    Ok(input.as_bits(prg, &const_sizes))
}

/// Decodes output bits from the mpc_core engine as a Garble literal.
pub fn deserialize_output(
    prg: &TypedProgram,
    fn_def: &TypedFnDef,
    output: &[bool],
) -> Result<Literal> {
    let output_ty = &fn_def.ty;
    let const_sizes = std::collections::HashMap::new();
    Literal::from_result_bits(prg, output_ty, output, &const_sizes).map_err(|e| e.prettify(""))
}
