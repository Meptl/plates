struct Program {
    pub vars: Vec<Variable>,
    pub varindex: HashMap<String, Vec<usize>>,
    //pub output: Arena<Expression>
}

impl Program {
    pub fn new() -> Program {
        Program {
            vars: Vec::new(),
            varindexer: HashMap::new()
        }
    }

    /// Add a variable into the program.
    pub fn add_variable(&mut self, v: Variable) {
        let idx = self.vars.len();
        self.push_varindex(v.vartype.as_str(), idx);
        self.varindexer[v.vartype.as_str()]
        self.vars.push(v);
    }

    fn push_varindex(&mut self, key: &str, value: usize) {
        let mut map = self.varindex.entry(key).or_default(Vec::new());
        map.push(value);
    }
}

