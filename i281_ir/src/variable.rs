use i281_ast::Literal;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Variable {
    pub data: Vec<i8>,
}

impl Variable {
    pub fn into_data(self) -> Vec<i8> {
        self.data
    }
}

impl From<&i281_ast::Variable> for Variable {
    fn from(v: &i281_ast::Variable) -> Self {
        let from_basic = |v: &Literal| match v {
            Literal::Byte(b) => b.0,
            Literal::NotSet(_) => 0,
            _ => unreachable!(),
        };

        let data = match &v.value {
            Literal::Byte(v) => vec![v.0],
            Literal::NotSet(_) => vec![0],
            Literal::Array(v) => v.0.iter().map(from_basic).collect(),
        };
        Self { data }
    }
}

impl From<i281_ast::Variable> for Variable {
    fn from(v: i281_ast::Variable) -> Self {
        Self::from(&v)
    }
}
