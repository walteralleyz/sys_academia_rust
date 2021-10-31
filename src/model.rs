use std::fmt::{Display, Formatter, Result};

pub struct User {
    id: u16,
    name: String,
    age: u8,
    height: f32,
    weight: f32
}

impl User {
    pub fn new(id: u16, name: String, age: u8, height: f32, weight: f32) -> Self {
        User { id, name, age, height, weight }
    }

    pub fn get_id(&self) -> &u16 {
        &self.id
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, r#"
        -------->Busca<--------
        |id: {}
        |nome: {}
        |idade: {}
        |altura: {}
        |peso: {}
        -----------------------"#,
            self.id, self.name, self.age, self.height, self.weight)
    }
}
