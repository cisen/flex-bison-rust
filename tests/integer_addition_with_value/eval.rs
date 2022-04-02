pub fn eval(value: &AST) -> isize {
    match value {
        AST::Int(int) => *int,
        AST::BinaryOperation(args) => match &args[1] {
            AST::OperatorAdd => eval(&args[0]) + eval(&args[2]),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
