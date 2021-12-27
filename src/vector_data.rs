/// Conjunto de estruturas e funções para o cálculo
/// de conceitos geométricos em segundo plano, 
/// como a coordenada e o vetor. 
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
    let aux1 = (b*b + c*c) as f64;
    
    // pitágoras
    if â == Â90 || â == Â270 {
      return  aux1.sqrt();
    }

    let aux2 = (2 * b * c) as f64 * â.cos() as f64;
    return  (aux1 - aux2).sqrt();
  }


  /// Fórmula de pitágoras
  pub fn pitagoras( b : i32, c : i32) -> f64 {
    let aux1 = (b*b + c*c) as f64;
    return aux1.sqrt();
  }


  /// Fórmula de pitágoras com floats
  pub fn fpitagoras( b: f64, c: f64) -> f64 {
    return (b*b + c*c).sqrt();
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


    /// Soma um vetor ao ponto sem alterar o original
    pub fn sum( self, v: Vetor) -> Self {
      Self {
        x: self.x + v.x,
        y: self.y + v.y
      }
    }


    /// Move o ponto através de um vetor
    pub fn mov( &mut self, v: Vetor) {
      self.x += v.x;
      self.y += v.y;
    }

    
    /// Retorna a distância entre dois pontos
    pub fn diff( p1: Ponto, p2: Ponto) -> f64 {
      let v = Vetor::from(p1, p2);
      return v.module();
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


    /// Retorna o vetor negado
    pub fn neg( self ) -> Self {
      Self {
        x: -self.x,
        y: -self.y
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