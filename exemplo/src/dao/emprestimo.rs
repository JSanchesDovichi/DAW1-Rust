use mongodb::bson::Bson::DateTime;
use mongodb::{bson, Collection};
use crate::classes::emprestimo::Emprestimo;

pub struct ColecaoEmprestimos {
    pub colecao: Collection<Emprestimo>,
}

impl ColecaoEmprestimos {
   /*
    pub async fn criar_emprestimo(&self, nome_chave: String, nome_servidor: String) {
        //Buscar a chave
        //Atualizar a chave

        //Buscar o servidor

        Emprestimo{
            id: None,
            data_hora_emprestimo: bson::DateTime::now(),
            data_hora_devolucao: None,
            chave: (),
            servidor_retirada: (),
            servidor_devolucao: None,
            ativo: true,
        }

        //Adicionar empréstimo
    }

    */
}