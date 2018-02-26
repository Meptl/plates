struct Program {
    pub vars: Arena<Variable>,
    pub varindexer: HashMap<String, NodeId>,
    //pub output: Arena<Expression>
}

impl Program {
    pub fn add_variable(&mut self, v: Variable) {
        let id = self.vars.new_node(v);
        let vartype = v.as_str();
        self.varindexer[vartype] = id;
    }
}

