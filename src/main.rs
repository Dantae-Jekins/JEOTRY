pub mod vector_data;
use vector_data::{
  Ponto as Ponto,
  Vetor as Vetor,
  Modulo as Modulo
};

pub mod object_data;
use object_data::{
  Bloco as Bloco,
  Quadrilatero as Quadrilatero,
};

use crate::object_data::Circulo;

/// Projeto de treino
/// a função main tem como propósito apenas o teste
/// das ferramentas disponibilizadas pelos módulos
/// presentes
fn main() 
{
  use std::io::stdin;
  println!(" Função à printar?");
  print!(">> Modulo\n");
  print!(">> Soma\n");
  print!(">> Objetos\n");
  print!(">> Colisoes\n");

  let mut input = String::new();
  stdin().read_line(&mut input)
    .expect("Não foi possível ler a string");
  match input.trim() {
      "Modulo"  => {modulo()},
      "Soma"    => {soma()},
      "Objetos" => {objetos()},
      "Colisoes"=> {colisoes()}
      x    => println!("Não existe uma função {}", x),
  }
}

/// Função teste para provar métodos e funções referentes á módulo
/// e orientação de vetores
#[allow(non_snake_case)]
fn modulo() {
  let A = Ponto::new(5, 5);
  let B = Ponto::new(-5, 5);
  let C = Ponto::new(-5, -5);
  let D = Ponto::new(5, -5);
  let O = Ponto::new(0, 0);

  // isto forma um quadrado, mas no espaço vetorial
  // são apenas vetores apontando para as 4 direções/sentidos 
  // diferentes. (cima, baixo, <-, ->);
  let AB = Vetor::from(A, B);
  let BC = Vetor::from(B, C);
  let CD = Vetor::from(C, D);
  let DA = Vetor::from(D, A);

  let mAB = Modulo::extract(AB);
  let mBC = Modulo::extract(BC);
  let mCD = Modulo::extract(CD);
  let mDA = Modulo::extract(DA);

  println!("\nVetores: ");
  println!("Vetor {}\n >> {:?}\n", "AB", AB);
  println!("Vetor {}\n >> {:?}\n", "BC", BC);
  println!("Vetor {}\n >> {:?}\n", "CD", CD);
  println!("Vetor {}\n >> {:?}\n", "DE", DA);

  println!("\nVetores (module + angle em rads): ");
  println!("Vetor {}\n >> {:.2}, {:.2}\n", "AB", AB.module(), AB.angle());
  println!("Vetor {}\n >> {:.2}, {:.2}\n", "BC", BC.module(), BC.angle());
  println!("Vetor {}\n >> {:.2}, {:.2}\n", "CD", CD.module(), CD.angle());
  println!("Vetor {}\n >> {:.2}, {:.2}\n", "DE", DA.module(), DA.angle());

  println!("\nMódulos: ");
  println!("Vetor {}\n >> {:?}\n", "AB", mAB);
  println!("Vetor {}\n >> {:?}\n", "BC", mBC);
  println!("Vetor {}\n >> {:?}\n", "CD", mCD);
  println!("Vetor {}\n >> {:?}\n", "DE", mDA);

  let OA = Vetor::from(O, A);
  let OB = Vetor::from(O, B);
  let OC = Vetor::from(O, C);
  let OD = Vetor::from(O, D);
  
  let mOA = Modulo::extract(OA);  
  let mOB = Modulo::extract(OB); 
  let mOC = Modulo::extract(OC);
  let mOD = Modulo::extract(OD);

  println!("\n\nVetores de O (ponto 0, 0): ");
  println!("Vetor {}\n >> {:?}\n", "OA", OA);
  println!("Vetor {}\n >> {:?}\n", "OB", OB);
  println!("Vetor {}\n >> {:?}\n", "OC", OC);
  println!("Vetor {}\n >> {:?}\n", "OD", OD);
  
  println!("\nMódulos: ");
  println!("Vetor {}\n >> {:?}\n", "OA", mOA);
  println!("Vetor {}\n >> {:?}\n", "OB", mOB);
  println!("Vetor {}\n >> {:?}\n", "OC", mOC);
  println!("Vetor {}\n >> {:?}\n", "OD", mOD);
 
}

/// Função teste para provar métodos e funções relacionadas
/// à soma de vetores e extração
fn soma() {

  let v1 = Vetor::new(5, 5);
  let v2 = Vetor::new(-5, -5);
  let v3 = Vetor::new(0, 10);
  let v4 = Vetor::new(-10, 0);

  println!("\nVetores: ");
  println!("Vetor {}\n >> {:?}\n", "1", v1);
  println!("Vetor {}\n >> {:?}\n", "2", v2);
  println!("Vetor {}\n >> {:?}\n", "3", v3);
  println!("Vetor {}\n >> {:?}\n", "4", v4);

  println!("\nVetores somados: ");
  println!("Vetor {}\n >> {:?}\n", "1 + 2", v1.clone().sum(v2));
  println!("Vetor {}\n >> {:?}\n", "2 + 3", v2.clone().sum(v3));
  println!("Vetor {}\n >> {:?}\n", "3 + 4", v3.clone().sum(v4));
  println!("Vetor {}\n >> {:?}\n", "4 + 1", v4.clone().sum(v1));

  println!("\nMódulo do conjunto: ");
  println!("Vetor {}\n >> {:?}\n", "1 + 2", Modulo::from(v1, v2));
  println!("Vetor {}\n >> {:?}\n", "2 + 3", Modulo::from(v2, v3));
  println!("Vetor {}\n >> {:?}\n", "3 + 4", Modulo::from(v3, v4));
  println!("Vetor {}\n >> {:?}\n", "4 + 1", Modulo::from(v4, v1));
}

/// Função teste para provar métodos e funções relacionadas aos objetos
#[allow(non_snake_case)]
fn objetos() {
  // vetores para teste
  let v1 = Vetor::new(2, 4);

  // pontos para teste
  let p1 = Ponto::new(2, 4);
  let p2 = Ponto::new(2, -4);


  let mut B1 = Bloco::new((1, 1), (1, 1));
  let mut B2 = Bloco::from(p1, p2);
  
  let mut B3 = Bloco::new((1, 3), (3, 1));
  // deve resular em : 
  // p1: 1, 1    p2: 3, 3 

  let mut B4 = Bloco::new((2, 1), (5, 2));
  // deve resultar em:
  // p1: 2, 1    p2: 5, 2
  

  let mut Q1 = 
  Quadrilatero::new( 
    (1, 2), 
    4, 
    1 
  );

  let mut Q2 =
  Quadrilatero::from(
    p1,
    1,
    1
  );

  // correto
  println!("\nBlocos: ");
  println!("Bloco {}\n >> {:?}\n", "B1", B1);
  println!("Bloco {}\n >> {:?}\n", "B2", B2);
  println!("Bloco {}\n >> {:?}\n", "B3", B3);
  println!("Bloco {}\n >> {:?}\n", "B4", B4);
  
  println!("\nQuadriláteros: ");
  println!("Quad. {}\n >> {:?}\n", "Q1", Q1);
  println!("Quad. {}\n >> {:?}\n", "Q2", Q2);

  B1.mov(v1);
  B2.mov(v1);
  B3.mov(v1);
  B4.mov(v1);

  Q1.mov(v1);
  Q2.mov(v1);


  println!("\n\n Somados ao vetor {:?}", v1);
  println!("\nBlocos: ");
  println!("Bloco {}\n >> {:?}\n", "B1", B1);
  println!("Bloco {}\n >> {:?}\n", "B2", B2);
  println!("Bloco {}\n >> {:?}\n", "B3", B3);
  println!("Bloco {}\n >> {:?}\n", "B4", B4);
  
  println!("\nQuadriláteros: ");
  println!("Quad. {}\n >> {:?}\n", "Q1", Q1);
  println!("Quad. {}\n >> {:?}\n", "Q2", Q2);
  
  B1.mov(v1.neg());
  B2.mov(v1.neg());
  B3.mov(v1.neg());
  B4.mov(v1.neg());

  Q1.mov(v1.neg());
  Q2.mov(v1.neg());

  println!("\nObjetos com merge (não somados ao vetor): ");
  println!("Quad. {}\n >> {:?}\n", "Q1+Q2", Quadrilatero::merge(Q1, Q2));
  println!("Bloco {}\n >> {:?}\n", "B1+B2", Bloco::merge(B1, B2));
  
}

fn colisoes() {
  let b1 = Bloco::new((1, 1), (3, 3));
  let b2 = Bloco::new((2, 2), (5, 5));
  let b3 = Bloco::new((4, 4), (6, 6));
  // uma corrente na diagonal de blocos
  // bloco 1 colide com bloco 2
  // bloco 2 colide com bloco 3
  // bloco 3 não colide com bloco 1

  println!("\n COLISÕES BLOCOS: ");
  println!(">> {} : {}", "12", b1.collide_block(b2));
  println!(">> {} : {}", "23", b2.collide_block(b3));
  println!(">> {} : {}", "31", b3.collide_block(b1));
  println!("      3____ ");
  println!("   2__|    |");
  println!("1__|  |____|");
  println!("|  |____|   ");
  println!("|____|      ");


  // mesma coisa com quadrláteros
  let q1 = Quadrilatero::new((1, 1), 2, 2);
  let q2 = Quadrilatero::new((2, 2), 2, 2);
  let q3 = Quadrilatero::new((4, 4), 2, 2);
  
  println!("\n COLISÕES QUADRILÁTEROS: ");
  println!(">> {} : {}", "12", q1.collide_quad(q2));
  println!(">> {} : {}", "23", q2.collide_quad(q3));
  println!(">> {} : {}", "31", q3.collide_quad(q1));
  println!("      3____ ");
  println!("   2__|    |");
  println!("1__|  |____|");
  println!("|  |____|   ");
  println!("|____|      ");

  println!("\n COLISÕES ENTRE QUADRILÁTEROS NÃO EQUILÁTEROS: ");
  let q1 = Quadrilatero:: new((1, 1), 2, 1);
  let q2 = Quadrilatero:: new((2, 2), 2, 1);
  let q3 = Quadrilatero:: new((3, 1), 3, 1);
  println!(">> {} : {}", "12", q1.collide_quad(q2));
  println!(">> {} : {}", "23", q2.collide_quad(q3));
  println!(">> {} : {}", "31", q3.collide_quad(q1));
  println!("                     ");
  println!("     ________        ");
  println!("  ___|______|___     ");
  println!(" |______|______|     ");
  let q1 = Quadrilatero:: new((1, 1), 2, 1);
  let q2 = Quadrilatero:: new((2, 3), 2, 1);
  let q3 = Quadrilatero:: new((3, 1), 3, 1);
  println!("\n>> {} : {}", "12", q1.collide_quad(q2));
  println!(">> {} : {}", "23", q2.collide_quad(q3));
  println!(">> {} : {}", "31", q3.collide_quad(q1));
  println!("     ________        ");
  println!("     |______|        ");
  println!("  ______________     ");
  println!(" |______|______|     ");

  println!("\n COLISÕES ENTRE QUADRILÁTEROS E BLOCOS");
  println!(">> {} : {}", "12", b1.collide_quad(q2));
  println!(">> {} : {}", "23", b2.collide_quad(q3));
  println!(">> {} : {}", "31", b3.collide_quad(q1));
  println!("     Mucha informacíon               ");


  let c1 = Circulo::new((1, 1), 2.0);
  let c2 = Circulo::from(Ponto::new(3, 3), 1.0);
  let c3 = Circulo::new((3, 5), 1.0);
  println!("\n COLISÕES CÍRCULOS:");
  println!(">> {} : {}", "12", c1.collide_circle(c2));
  println!(">> {} : {}", "23", c2.collide_circle(c3));
  println!(">> {} : {}", "31", c3.collide_circle(c1));

}