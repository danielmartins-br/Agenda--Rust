extern crate rusqlite;
use rusqlite::{Connection,NO_PARAMS};
use std::io;

fn main()
{
     let cria_banco = Connection::open("contatos.db").unwrap();

     cria_banco.execute(" CREATE TABLE IF NOT EXISTS Contatos (
          id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
          nome VARCHAR(50) NOT NULL,
          email VARCHAR(40) NOT NULL,
          telefone VARCHAR(20) NOT NULL,
          dataNascimento VARCHAR(10)
               )", NO_PARAMS).unwrap();
     
//   let nome: String = "Gibson".to_string();
//   let email: String = "smith@mormail.com".to_string();

     let mut grava_nome = String::new();
     let mut grava_email = String::new();
     let mut grava_telefone = String::new();
     let mut grava_nascimento = String::new();

     println!("\nEntre com o Nome do Contato: ");
     io::stdin().read_line(&mut grava_nome).expect("Ocorreu um Erro");

     println!("\nInsira o Email: ");
     io::stdin().read_line(&mut grava_email).expect("Ocorreu um Erro");

     println!("\nInsira o Número de Telefone: ");
     io::stdin().read_line(&mut grava_telefone).expect("Ocorreu um Erro");

     println!("\nInsira a Data de Nascimento (00/00/0000): ");
     io::stdin().read_line(&mut grava_nascimento).expect("Ocorreu um Erro");

     println!("\n----Dados Inseridos----");
     print!("Nome -> {}",grava_nome);
     print!("Email -> {}",grava_email);
     print!("Telefone -> {}",grava_telefone);
     print!("Nascimento -> {}",grava_nascimento);

     cria_banco.execute("INSERT INTO Contatos (nome,email,telefone,dataNascimento) VALUES (?1,?2,?3,?4)",
             &[&grava_nome, &grava_email,&grava_telefone,&grava_nascimento]).unwrap();
}
