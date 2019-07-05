mod internals;

// TODO: write tests (especially for check_validity)
// TODO: implement reformatter (simplify in Parsing format and reexpand to string)
// TODO: implement multi-step truth table, split expression into significant parts
// TODO: write command line interface

fn main() {
    internals::display("((p => !q) & r) <=> (!r => q)");
}