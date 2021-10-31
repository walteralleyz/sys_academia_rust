use std::io::stdin;
use crate::model::User;
use crate::utils::{input_string, input_u8, input_f32};

pub struct Repository {
    pub database: Vec<User>
}

impl Repository {
    pub const fn new() -> Self {
        Repository { database: Vec::new() }
    }

    pub fn validate_index(&self) -> usize {
        let mut id = String::new();

        println!("Insira o id para buscar o usuário: ");

        match stdin().read_line(&mut id) {
            Ok(_) => {
                for (i, user) in self.database.iter().enumerate() {
                    let result: u16 = id.trim().parse().unwrap();
                    if user.get_id() == &result {
                        return i;
                    }
                }

                usize::MAX
            },

            Err(_e) => panic!("Id do usuário não encontrado")
        }
    }

    pub fn read(&self, index: usize) -> &User {
        self.database.get(index).unwrap()
    }

    pub fn create(&mut self) -> i16 {
        let len = self.database.len() as u16;

        let user = User::new(
            len,
            input_string("Nome:".to_string()),
            input_u8("Idade:".to_string()),
            input_f32("Altura:".to_string()),
            input_f32("Peso:".to_string())
        );

        self.database.push(user);

        if self.database.len() as u16 > len {
            let i = self.database.len() as i16;
            return i-1;
        }

        -1
    }

    pub fn update(&mut self) -> bool {
        let id = input_u8("Id:".to_string()) as u16;
        let user_temp: User;

        for (i, user) in self.database.iter().enumerate() {
            if user.get_id() == &id {
                user_temp = User::new(
                    id,
                    input_string("Nome:".to_string()),
                    input_u8("Idade:".to_string()),
                    input_f32("Altura:".to_string()),
                    input_f32("Peso:".to_string())
                );

                self.database.insert(i, user_temp);
                return true;
            }
        }

        false
    }

    pub fn delete(&mut self) -> bool {
        let id = input_u8("Id:".to_string()) as u16;

        for (i, user) in self.database.iter().enumerate() {
            if user.get_id() == &id {
                self.database.remove(i);
                return true;
            }
        }

        false
    }
}