/// Store of all data.
///
/// Note that the structures that implement Data are immutable after initialization.
#[derive(Default)]
pub struct DataTable {
    data: Vec<Arc<Data>>,

    scalars: HashMap<ScalarType, Vec<Arc<Data>>>,
    contexts: Vec<Arc<Data>>,
    structs: Vec<Arc<Data>>,
    functions: Vec<Arc<Data>>,
    lambdas: Vec<Arc<Data>>,
}

impl DataTable {
    fn new() -> DataTable {
        DataTable {
            ..Default::default()
        }
    }

    pub fn add_scalar(&mut self, s: Scalar) -> Arc<Scalar> {
        let s = Arc::new(s);
        self.data.push(s.clone());
        self.get_scalars_of_type(s.scalar_type()).push(s.clone());
        s.clone()
    }

    pub fn add_context(&mut self, c: Context) -> Arc<Context> {
        let c = Arc::new(c);
        self.data.push(c.clone());
        match c.name() {
            Some(name) => self.structs,
            None => self.contexts,
        }.push(c.clone())
        c.clone()
    }

    pub fn add_functions(&mut self, f: Function) -> Arc<Function> {
        let f = Arc::new(f);
        self.data.push(f.clone());
        match f.name() {
            Some(name) => self.functions,
            None => self.lambdas,
        }.push(f.clone())
        f.clone()
    }

    pub fn iter_scalars(&self) -> usize {
        self.scalars.iter()
    }

    fn get_scalars_of_type(&mut self, t: ScalarType) -> &mut Vec<Arc<Data>> {
        self.scalars.entry(t).or_insert(Vec::new())
    }
}
