use crate::repository::Repository;
use crate::view::View;

pub struct Controller;

impl Controller {
    pub fn new() -> Self {
        Controller {}
    }

    pub fn exec(&self) {
        let mut repository = Repository::new();

        loop {
            let option = View::menu();

            match option {
                0 => break,
                1 => self.get(&mut repository),
                2 => self.post(&mut repository),
                3 => self.put(&mut repository),
                4 => self.delete(&mut repository),
                _ => println!("Invalid Option")
            };
        }
    }

    pub fn get(&self, rep: &mut Repository) {
        let index = rep.validate_index();

        if index == usize::MAX {
            View::message_routine("Usuário não encontrado");
            return;
        }

        let user = rep.read(index);

        View::show_read_message(user);
    }

    pub fn post(&self, rep: &mut Repository) {
        let created = rep.create();

        if created == -1 {
            View::message_routine("Não foi possível criar o usuário");
            return;
        }

        View::show_created_message(created);
    }

    pub fn put(&self, rep: &mut Repository) {
        let updated = rep.update();

        if !updated {
            View::message_routine("Erro ao atualizar usuário");
            return;
        }

        View::show_updated_message();
    }

    pub fn delete(&self, rep: &mut Repository) {
        let deleted = rep.delete();

        if !deleted {
            View::message_routine("Usuário não removido");
            return;
        }

        View::show_removed_message();
    }
}