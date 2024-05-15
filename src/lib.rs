use tandem_garble_interop::{check_program, compile_program, TypedCircuit};

pub fn compatibility_circuit() -> TypedCircuit {
    let garble_src = include_str!("compatibility.garble.rs");

    let garble_prog = check_program(garble_src).unwrap();
    let garble_circuit = compile_program(&garble_prog, "main").unwrap();

    garble_circuit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compiles() {
        let circuit = compatibility_circuit();

        assert_eq!(circuit.gates.contrib_inputs(), circuit.gates.eval_inputs());
    }
}
