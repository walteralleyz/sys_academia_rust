use crate::model::User;
use crate::utils::input_u8;

use std::thread::sleep;
use std::time::Duration;

pub struct View;

impl View {
    pub fn menu() -> u8 {
        View::clear_screen();
        input_u8(r#"
        --------->MENU<--------
        |1 - Buscar           |
        |2 - Cadastrar        |
        |3 - Atualizar        |
        |4 - Remover          |
        |0 - Sair             |
        -----------------------
        "#.to_string())
    }

    pub fn show_read_message(user: &User) {
        View::message_routine(format!("{}", user).as_str());
    }

    pub fn show_created_message(id: i16) {
        View::message_routine(format!("Usuário criado com Id: {}", id).as_str());
    }

    pub fn show_removed_message() {
        View::message_routine("Usuário removido com sucesso");
    }

    pub fn show_updated_message() {
        View::message_routine("Usuário atualizado com sucesso)")
    }

    pub fn message_routine(message: &str) {
        View::clear_screen();
        println!("{}", message);
        sleep(Duration::from_secs(3));
    }

    pub fn clear_screen() {
        print!("{}[2J", 27 as char);
    }
}