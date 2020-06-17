//MUDAR NOME DO PROJETO - AGENDA-RUST
extern crate rusqlite;
use rusqlite::{Connection,params,Result};
use std::io::{stdin,stdout,Write};
use colored::*;
use std::process::Command;

#[derive(Debug)]
struct Contatos{
     id: i32,
     nome: String,
     email: String,
     telefone: String,
     nascimento: String,
}

fn read(input: &mut String)
{
     stdout().flush().expect("Falha no Flush");
     stdin().read_line(input).expect("Erro na Leitura");
}

fn clear_screen() 
{
     //Grava o comando clear na variavel output
     let output = Command::new("clear").output().unwrap_or_else(|e| {
          panic!("falha ao executar processo: {}", e)
         });
     //printa a variavel, limpando a tela
     println!("{}", String::from_utf8_lossy(&output.stdout));
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

     let mut id = String::new();
     let mut opcao = String::new();
     let mut grava_nome = String::new();
     let mut grava_email = String::new();
     let mut grava_telefone = String::new();
     let mut grava_nascimento = String::new();
     
     clear_screen();
     println!("{}","Bem Vindo a Sua Agenda".green().bold());
     println!("{}","-----------------------".bold());
     println!("Selecione uma Opção: \n\n1-> Cadastrar \n2-> Listar \n3-> Excluir \n4-> Atualizar Dados\n5-> Consultar");
     println!("0-> Ajuda/\n");
     print!("Opção: ");
     read(&mut opcao);
     //converte string para int
     let opcao: i32 = opcao.trim().parse().unwrap();
     if opcao == 1
     {
          println!("{}","\n-----------Cadastro de Contato------------\n".bold());

          println!("\nEntre com o Nome do Contato: ");
          read(&mut grava_nome);
          println!("\nInsira o Email: ");
          read(&mut grava_email);
          println!("\nInsira o Número de Telefone: ");
          read(&mut grava_telefone);
          println!("\nInsira a Data de Nascimento (Exemplo: 00/00/0000): ");
          read(&mut grava_nascimento);

          println!("{}","\n\n----Dados Inseridos com Sucesso----".green().bold());
          print!("Nome -> {}",grava_nome);
          print!("Email -> {}",grava_email);
          print!("Telefone -> {}",grava_telefone);
          print!("Nascimento -> {}",grava_nascimento);

          cria_banco.execute("INSERT INTO Contatos (nome,email,telefone,dataNascimento) VALUES (?1,?2,?3,?4)",
                  &[&grava_nome, &grava_email,&grava_telefone,&grava_nascimento]).unwrap();
     }
     
     if opcao == 2
     {
          println!("{}","\n-----------Lista de Contatos------------\n".bold());

          let mut stmt  = cria_banco.prepare("SELECT nome,email,telefone,dataNascimento,id FROM Contatos")?;
          let iterador = stmt.query_map(params![], |row|{ 
                    Ok(Contatos {
                         nome: row.get(0)?,
                         email: row.get(1)?,
                         telefone: row.get(2)?,
                         nascimento: row.get(3)?,
                         id: row.get(4)?,
                         })
                    })?;
          for pessoa in iterador {
               println!("{:?}",pessoa);
          }
     }

     if opcao == 3
     { 
          println!("{}","\n-----------Deletar Contato-------------\n".bold());
          println!("\nInsira o ID do Contato que deseja Deletar: ");
          read(&mut id);
          let id: i32 = id.trim().parse().unwrap();
//Todo listar informacoes do contato removido 
          cria_banco.execute("DELETE FROM Contatos where id = ?",&[&id]).unwrap();
          println!("{}","\nContato deletado com Sucesso!".red().bold());
     }

     if opcao == 4
     {
          println!("{}","\n-----------Atualizar Dados do Contato-------------\n".bold());
          println!("\nInsira o ID do Contato que deseja Atualizar: ");
          read(&mut id);
//Todo verificar se o contato existe no BD
          let mut campo = String::new();
          println!("Qual Campo Deseja Alterar:\n");
          println!("1~> Nome\n2~> Email\n3~> Telefone\n4~> Data de Nascimento\n0~> Todos os Campos");
          print!("Campo: ");
          read(&mut campo);
          let campo: i32 = campo.trim().parse().unwrap();
//Todo adicionar dicas
          if campo == 1
          {
               println!("\nInsira o Nome do Contato: ");
               read(&mut grava_nome);
               cria_banco.execute("UPDATE Contatos SET nome=? WHERE id = ?",
                  &[&grava_nome,&id]).unwrap();
               println!("\nAlteração Concluida!");
          }

          if campo == 2
          {
               println!("\nInsira o Email: ");
               read(&mut grava_email);
               cria_banco.execute("UPDATE Contatos SET email=? WHERE id = ?",
                  &[&grava_email,&id]).unwrap();
               println!("\nAlteração Concluida!");
          }
          
          if campo == 3
          {
               println!("\nInsira o Número de Telefone: ");
               read(&mut grava_telefone);
               cria_banco.execute("UPDATE Contatos SET telefone=? WHERE id = ?",
                  &[&grava_telefone,&id]).unwrap();
               println!("\nAlteração Concluida!");
          }

          if campo == 4
          {
               println!("\nInsira a Data de Nascimento (Exemplo: 00/00/0000): ");
               read(&mut grava_nascimento);
               cria_banco.execute("UPDATE Contatos SET dataNascimento=? WHERE id = ?",
                  &[&grava_nascimento,&id]).unwrap();
               println!("\nAlteração Concluida!");
          }

          if campo == 0
          {
               println!("\nEntre com o Nome do Contato: ");
               read(&mut grava_nome);
               println!("\nInsira o Email: ");
               read(&mut grava_email);
               println!("\nInsira o Número de Telefone: ");
               read(&mut grava_telefone);
               println!("\nInsira a Data de Nascimento (Exemplo: 00/00/0000): ");
               read(&mut grava_nascimento);

               println!("\n\n----Dados Atualizados com Sucesso----");
               print!("Nome -> {}",grava_nome);
               print!("Email -> {}",grava_email);
               print!("Telefone -> {}",grava_telefone);
               print!("Nascimento -> {}",grava_nascimento);

               cria_banco.execute("UPDATE Contatos SET nome=?,email=?,telefone=?,dataNascimento=? WHERE id = ?",
                  &[&grava_nome, &grava_email,&grava_telefone,&grava_nascimento,&id]).unwrap();
          }

     }

     if opcao == 5
     { 
          println!("\n-----------Pesquisar dados do Contato-------------\n");
         
     }

     Ok(())
}
