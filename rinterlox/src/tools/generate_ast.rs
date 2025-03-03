pub struct GenerateAst {
}

impl GenerateAst {
    pub fn new() -> Self {
        Self {}
    }

    pub fn gen(args: Vec<&str>) {
        if args.len() != 1 {
            println!("[Usage]: generate_ast <output dir>");
            std::process::exit(64);
        }
        let output = args[0];
        self.defineAST(output, "Expr", vec![
            "Binary   : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal  : Object value",
            "Unary    : Token operator, Expr right"
        ]);
    }

    fn defineAST(&self, output: &str, base: &str, types: Vec<&str>) {
        
    }
}