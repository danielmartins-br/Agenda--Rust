extern crate rusqlite;
use rusqlite::{Connection,params,Result};
use std::io;
use std::io::{stdin,stdout,Write};

#[derive(Debug)]
struct Contatos{
     grava_nome: String,
     grava_email: String,
     grava_telefone: String,
     grava_nascimento: String,
}

fn read(input: &mut String)
{
     stdout().flush().expect("Falha no Flush");
     stdin().read_line(input).expect("Erro na Leitura");
}

fn main() -> Result<()>
{
     let cria_banco = Connection::open("contatos.db").unwrap();
//Todo perguntar se quer criar um banco, ou apagar
     cria_banco.execute(" CREATE TABLE IF NOT EXISTS Contatos (
          id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
          nome VARCHAR(50) NOT NULL,
          email VARCHAR(40) NOT NULL,
          telefone VARCHAR(20) NOT NULL,
          dataNascimento VARCHAR(10)
               )",params![], ).unwrap();

     let mut opcao = String::new();
     let mut grava_nome = String::new();
     let mut grava_email = String::new();
     let mut grava_telefone = String::new();
     let mut grava_nascimento = String::new();

     println!("\nBem Vindo a Sua Agenda em Rust");
     println!("-------------------------------");
     println!("Selecione uma Opção: \n\n1-> Cadastrar Contato\n2-> Listar Contatos\n");
     read(&mut opcao);
//Todo mudar tudo para read,por causa da biblioteca
     //converte string para int
     let opcao: i32 = opcao.trim().parse().unwrap();
     if opcao == 1
     {

          println!("\nEntre com o Nome do Contato: ");
          io::stdin().read_line(&mut grava_nome).expect("Ocorreu um Erro com nome");

          println!("\nInsira o Email: ");
          io::stdin().read_line(&mut grava_email).expect("Ocorreu um Erro com email");

          println!("\nInsira o Número de Telefone: ");
          io::stdin().read_line(&mut grava_telefone).expect("Ocorreu um Erro com telefone");

          println!("\nInsira a Data de Nascimento (00/00/0000): ");
          io::stdin().read_line(&mut grava_nascimento).expect("Ocorreu um Erro com dataNascimento");

          println!("\n\n----Dados Inseridos com Sucesso----");
          print!("Nome -> {}",grava_nome);
          print!("Email -> {}",grava_email);
          print!("Telefone -> {}",grava_telefone);
          print!("Nascimento -> {}",grava_nascimento);

          cria_banco.execute("INSERT INTO Contatos (nome,email,telefone,dataNascimento) VALUES (?1,?2,?3,?4)",
                  &[&grava_nome, &grava_email,&grava_telefone,&grava_nascimento]).unwrap();
     }
     
     if opcao == 2
     {
          println!("\n-----------Lista de Contatos------------\n");

          let mut stmt  = cria_banco.prepare("SELECT nome,email,telefone,dataNascimento FROM Contatos")?;
          let iterator = stmt.query_map(params![], |row|{ 
                    Ok(Contatos {
                         grava_nome: row.get(0)?,
                         grava_email: row.get(1)?,
                         grava_telefone: row.get(2)?,
                         grava_nascimento: row.get(3)?,
                         })
                    })?;
          for pessoa in iterator {
               println!("Contato: {:?}",pessoa);
          }
     }
     Ok(())
}
