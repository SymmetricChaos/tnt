/// TNT consists of any valid statement
#[derive(Debug)]
pub enum TNT {
    Formula(Formula),
    Number(Number),
    Variable(Variable),
    Expression(Expression),
}

impl TNT {
    pub fn new(input: &str) -> TNT {
        if is_num(input) {
            return TNT::Number(Number::new(input))
        } else if is_var(input) {
            return TNT::Variable(Variable::new(input))
        } else if is_expression(input) {
            return TNT::Expression(Expression::new(input))
        } else if is_formula(input) {
            return TNT::Formula(Formula::new(input))
        } else {
            panic!()
        }
    }

    pub fn latex(&self) -> String {
        to_latex(self.to_string())
    }

    pub fn english(&self) -> String {
        to_english(self.to_string())
    }
}

impl fmt::Display for TNT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TNT::Number(term) => write!(f, "{}", term),
            TNT::Variable(term) => write!(f, "{}", term),
            TNT::Expression(term) => write!(f, "{}", term),
            TNT::Formula(term) => write!(f, "{}", term),
        }
    }
}