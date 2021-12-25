/// conjunto de estruturas e funções para o cálculo
/// de conceitos geométricos em segundo plano, 
/// como a coordenada e o vetor. 
pub mod data
{

  use core::fmt;
  use std::f32::consts::PI as pi;
  use std::fmt::Debug;

  /// ângulo de 90 em radianos
  pub static Â90 :f32 = 0.5 * pi;

  /// ângulo de 180 em radianos
  pub static Â180:f32 = 1.0 * pi;
  
  /// ângulo de 270 em radianos
  pub static Â270:f32 = 1.5 * pi;
  
  /// ângulo de 360 em radianos
  pub static Â360:f32 = 2.0 * pi;

  
  // estruturas
    /// Ponto no plano cartesiano 
    /// descrito por valores inteiros
    #[derive(Debug, Clone, Copy)]
    pub struct Ponto {
      /// posição x
      pub x: i32,
      /// posição y
      pub y: i32
    }

    /// Vetor no plano cartesiano 
    /// descrito por valores inteiros
    #[derive(Debug, Clone, Copy)]
    pub struct Vetor {
      /// comprimento x
      pub x: i32,
      /// comprimento y
      pub y: i32
    }

    /// Vetor com intensidade explícita
    /// e seu ângulo de inclinação
    #[derive(Clone, Copy)]
    pub struct Modulo {
      /// intensidade 
      pub i: f64,
      /// ângulo do vetor
      pub â: f32,
    }

  
  // Funções
    /// Fórmula de pitágoras extensa, aonde não tratamos
    /// triângulos retângulos.
    pub fn cosine_rule( b : i32, c : i32, â : f32) -> f64 {
      let aux1 = (b.pow(2) + c.pow(2)) as f64;
      
      // pitágoras
      if â == Â90 || â == Â270 {
        return  aux1.sqrt();
      }

      let aux2 = (2 * b * c) as f64 * â.cos() as f64;
      return  (aux1 - aux2).sqrt();
    }

    /// Fórmula de pitágoras
    pub fn pitagoras( b : i32, c : i32) -> f64 {
      let aux1 = (b.pow(2) + c.pow(2)) as f64;
      return aux1.sqrt();
    }

    /// Produto escalar, extração de uma grandeza 
    /// resultante por dois vetores
    pub fn scalar_prod( v1: Vetor, v2: Vetor) -> i32{
      return {
        v1.x * v2.x +
        v1.y * v2.y
      };
    }

    /// Extração do ângulo em radianos entre dois vetores
    pub fn inner_angle( v1: Vetor, v2: Vetor) -> f32 {
      // produto escalar
      let aux1 = scalar_prod(v1, v2) as f64;

      // módulos
      let aux2 = v1.module();
      let aux3 = v2.module();
      
      let cos:f64 = aux1 / (aux2 * aux3);
      return cos.acos() as f32;
    }


  // métodos
    impl Ponto {

      /// Gera um novo ponto no espaço
      pub fn new( x: i32, y: i32) -> Self {
        Self {
          x,
          y
        }
      } 

      /// Seta novos valores à um ponto existente
      pub fn set( &mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
      }
      
      /// Soma valores fixos ao ponto
      pub fn add( &mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
      }

      /// Soma um vetor ao ponto
      pub fn sum( &mut self, v: Vetor) {
        self.x += v.x;
        self.y += v.y;
      }
    }

    impl Vetor {

      /// Gera um novo vetor no espaço
      pub fn new( x: i32, y: i32 ) -> Self {
        Self {
          x,
          y
        }
      }

      /// Gera um novo vetor entre dois pontos
      pub fn from( p1: Ponto, p2: Ponto) -> Self{
        Self {
          x: (p2.x - p1.x),
          y: (p2.y - p1.y)      
        }
      }

      /// Extrai o comprimento ou módulo do vetor
      pub fn module( &self ) -> f64 {
        pitagoras(self.x, self.y)
      }

      /// Extrai o ângulo do vetor em radianos
      pub fn angle( &self ) -> f32 {
        let modulo = pitagoras(self.x, self.y);
        let aux = self.x as f32 / modulo as f32;
        let ângulo = aux.acos();

        if self.y < 0 {
          return Â360 - ângulo;
        }

        return ângulo;
      }

      /// Seta novos valores ao vetor
      pub fn set( &mut self, x: i32, y: i32) {
          self.x = x;
          self.y = y;
      }
      
      /// Soma valores fixos ao vetor
      pub fn add( &mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
      }

      /// Soma um vetor à este vetor
      pub fn sum( &mut self, v : Vetor) {
        self.x += v.x;
        self.y += v.y;
      }  
    }

    impl Modulo {
      /// Geração de um módulo de um vetor 
      /// através de pontos definidos
      pub fn new( x: i32, y: i32) -> Self {
        // produto escalar simplificado por Álgebra
        let intensidade = pitagoras(x, y);
        let aux = x as f32 / intensidade as f32;
        let ângulo = aux.acos();

        // descobrimento da orientação
        if y < 0 {
          return Self {
            i: intensidade,
            â: Â360 - ângulo
          }
        }

        return Self {
          i: intensidade,
          â: ângulo
        }
      }

      /// Extração do módulo e da orientação de um 
      /// vetor já existente
      pub fn extract( v1: Vetor) -> Self{
        return Modulo::new( v1.x, v1.y);
      }

      /// Extrai o módulo de dois vetores
      pub fn from( mut v1: Vetor, v2: Vetor) -> Self{
        v1.sum(v2);
        return Modulo::new(v1.x, v1.y);
      }
    }

    impl Debug for Modulo{
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " Modulo: {{ i: {:.2}, â: {:>3.0} }}", self.i, self.â.to_degrees())
      }
    }
}

/// Objetos exemplares que usufruem das estruturas declaradas
/// no módulo de data, estes objetos corresponbdem á figuras 
/// geométricas como quadrados e são descritos por componentes
/// presentes no módulo de data.
/// 
/// Os objetos presentes no módulo crescem em um padrão de
/// baixo para cima + esquerda para direita, como no plano
/// cartesiano comum, as coordenadas sempre são posicionadas
/// na esquerda-inferior (0,0) ou na direita-superior (1,1)
/// quando o objeto em questão é um quadrilátero, ou posicionadas
/// no centro do objeto quando este é um círculo.
pub mod objects 
{
  use super::data::{
      Ponto as Ponto,
      Vetor as Vetor,
    };


  // Estruturas 
  /// Bloco determinado por dois pontos no espaço
  /// A estrutura é normalizada para que o primeiro ponto
  /// se posicione no esquerdo inferior e o segundo na
  /// direita superior.
  #[derive(Debug, Clone, Copy)]
  pub struct Bloco {
   /// Primeira coordenada xy: Esquerda inferior
    pub p1: Ponto,

    /// Segunda coordenada xy: Direita superior
    pub p2: Ponto
  }

  /// Quadrilátero determinado por um ponto no espaço e duas dimensões
  /// A estrutura é normalizada de forma que sua coordenada seja 
  /// representada pelo canto esquerdo inferior.
  #[derive(Debug, Clone, Copy)]
  pub struct Quadrilatero {
    /// Coordenada xy: Esquerda inferior
    pub p: Ponto,

    /// Tamanho x
    pub tx: u16,

    /// Tamanho y
    pub ty: u16
  }

  /// Círculo determinado por um ponto no espaço e seu raio
  /// A estrutura é normalizada de forma que sua coordenada seja
  /// representada pelo seu centro.
  #[derive(Debug, Clone, Copy)]
  pub struct Circulo {
    /// Coordenada xy : Centro
    pub p: Ponto,

    /// Raio
    pub r: i32
  }


  // Métodos
  impl Bloco {
    /// Gera um novo Bloco através de quatro coordenadas no espaço
    pub fn new( p1: (i32, i32), p2: (i32, i32)) -> Self {

      // p1 deve estar na esquerda-inferior
      // p2 deve estar na direita-superior

      // arruma o X
      let (x1, x2);
      if p1.0 > p2.0 {
        x1 = p2.0; // x à esquerda
        x2 = p1.0; // x à direita
      } else {
        x1 = p1.0;
        x2 = p2.0;
      }

      // arruma o Y
      let (y1, y2);
      if p1.1 > p2.1 {
        y1 = p2.1; // y abaixo
        y2 = p1.1; // y acima
      } else {
        y1 = p1.1;
        y2 = p2.1;
      }

      Self {
        p1: Ponto::new(x1, y1),
        p2: Ponto::new(x2, y2),
      }
    }
    
    /// Gera um novo bloco através de dois pontos no espaço
    pub fn from( p1: Ponto, p2: Ponto) -> Self {

      // p1 deve estar na esquerda-inferior
      // p2 deve estar na direita-superior

      // arruma o X
      let (x1, x2);
      if p1.x > p2.x {
        x1 = p2.x; // x à esquerda
        x2 = p1.x; // x à direita
      } else {
        x1 = p1.x;
        x2 = p2.x;
      }

      // arruma o Y
      let (y1, y2);
      if p1.y > p2.y {
        y1 = p2.y; // y abaixo
        y2 = p2.y; // y acima
      } else {
        y1 = p2.y;
        y2 = p2.y;
      }

      Self {
        p1: Ponto::new(x1, y1),
        p2: Ponto::new(x2, y2),
      }
    }

    /// Move o bloco somando um vetor
    pub fn mov( &mut self, v: Vetor ) {
      self.p1.sum(v);
      self.p2.sum(v);
    }

    /// Retorna um bloco movimentado sem alterar o original
    pub fn sum(mut self, v: Vetor) -> Self{
      self.p1.sum(v);
      self.p2.sum(v);
      return self;
    }

    // Extrai um Quadrilátero do bloco
    pub fn into_quad( self ) -> Quadrilatero {
      let tamx = self.p2.x - self.p1.x;
      let tamy = self.p2.y - self.p1.y;
    
      Quadrilatero {
        p : self.p1,
        tx: tamx as u16,
        ty: tamy as u16,
      }
    }
  }
  
  impl Quadrilatero {
    /// Gera um novo quadrilátero através de uma coordenada e de um ponto
    pub fn new( p: (i32, i32), tx: u16, ty: u16 ) -> Self {
      Self {
        p: Ponto::new(p.0, p.1),
        tx,
        ty
      }
    }
      
    /// Gera um novo quadrilátero através de um Ponto e suas dimensões
    pub fn from( p: Ponto, tx: u16, ty: u16) -> Self {
      Self {
        p,
        tx,
        ty
      }
    }

    /// Retorna um quadrilátero movimentado sem alterar o original
    pub fn sum(mut self, v: Vetor) -> Self{
      self.p.sum(v);
      return self;
    }

    /// Move um quadrilátero 
    pub fn mov(&mut self, v: Vetor) {
      self.p.sum(v);
    }

    /// Gera um novo quadrilátero através de um Bloco 
    pub fn into_block( self ) -> Bloco {
      // ponto inicial
      let p1 = self.p;
      
      // pontos da segunda coordenada
      let px = self.p.x + self.tx as i32;
      let py = self.p.y + self.ty as i32;
      Bloco {
        p1,
        p2: Ponto::new(px, py),
      }
    } 
  }

  impl Circulo {
    /// Gera um novo Círculo através de uma coordenada e um raio
    pub fn new( p: (i32, i32), r: i32) -> Self {
      Self {
        p: Ponto::new(p.0, p.1),
        r
      }
    }

    /// Gera um novo Círculo através de um ponto e um raio
    pub fn from( p: Ponto, r: i32) -> Self {
      Self {
        p,
        r
      }
    }
  }
}
