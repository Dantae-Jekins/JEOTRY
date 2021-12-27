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
pub use super::vector_data::{
    Ponto as Ponto,
    Vetor as Vetor,
};


// Estruturas 
/// Bloco determinado por dois pontos no espaço.
/// 
/// A estrutura é normalizada para que o primeiro ponto
/// se posicione na esquerda inferior e o segundo na
/// direita superior.
/// 
/// Este tipo de objeto não é o recomendável para operações
/// como colisões ou movimentações, o quadrilátero armazena
/// informações mais óptimas para estes tipos de trabalho.
#[derive(Debug, Clone, Copy)]
pub struct Bloco {
  /// Primeira coordenada xy: Esquerda inferior
  pub p1: Ponto,

  /// Segunda coordenada xy: Direita superior
  pub p2: Ponto
}


/// Quadrilátero determinado por um ponto no espaço e duas dimensões.
/// 
/// A estrutura é normalizada de forma que sua coordenada seja 
/// se posicione no canto esquerdo inferior.
#[derive(Debug, Clone, Copy)]
pub struct Quadrilatero {
  /// Coordenada xy: Esquerda inferior
  pub p: Ponto,

  /// Tamanho x
  pub tx: i32,

  /// Tamanho y
  pub ty: i32
}


/// Círculo determinado por um ponto no espaço e seu raio.
/// 
/// A estrutura é normalizada de forma que sua coordenada seja
/// posicionada em seu centro.
#[derive(Debug, Clone, Copy)]
pub struct Circulo {
  /// Coordenada xy : Centro
  pub p: Ponto,

  /// Raio
  pub r: f64
}


// Métodos relacionados ao Bloco
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
      y2 = p1.y; // y acima
    } else {
      y1 = p1.y;
      y2 = p2.y;
    }

    Self {
      p1: Ponto::new(x1, y1),
      p2: Ponto::new(x2, y2),
    }
  }


  /// Move o bloco somando um vetor
  pub fn mov( &mut self, v: Vetor ) {
    self.p1.mov(v);
    self.p2.mov(v);
  }


  /// Retorna um bloco movimentado sem alterar o original
  pub fn sum(mut self, v: Vetor) -> Self{
    self.p1.mov(v);
    self.p2.mov(v);
    return self;
  }
  
  
  /// Extrai as coordenadas centrais do bloco
  pub fn center( self ) -> (f64, f64) {
    let tx = (self.p2.x - self.p1.x) as f64 / 2.0;
    let ty = (self.p2.y - self.p1.y) as f64 / 2.0;
    let px = self.p1.x as f64 + tx;
    let py = self.p1.y as f64 + ty;
    (px, py)
  }


  /// Retorna a área formada por dois blocos no espaço
  pub fn merge(b1: Bloco, b2: Bloco) -> Self {
    // identificar o p1 mais à esquerda e
    // extrair seu x
    let x1 = b1.p1.x.min(b2.p1.x);


    // identificar o p1 mais abaixo e
    // extrair seu y
    let y1 = b1.p1.y.min(b2.p1.y);

    // identificar o p2 mais à direita e
    // extrair seu x
    let x2 = b1.p2.x.max(b2.p2.x);

    // identificar o p2 mais acima e
    // extrair seu y
    let y2 = b1.p2.y.max(b2.p2.y);

    // formar o ponto esquerda-inferior
    let p1 = Ponto::new(x1, y1);
    // formar o ponto direita-superior
    let p2 = Ponto::new(x2, y2);

    Self {
      p1,
      p2
    }      
  }


  /// Extrai um Quadrilátero do bloco
  pub fn into_quad( self ) -> Quadrilatero {
    let tamx = self.p2.x - self.p1.x;
    let tamy = self.p2.y - self.p1.y;
  
    Quadrilatero {
      p : self.p1,
      tx: tamx,
      ty: tamy,
    }
  }


  /// Analisa a colisão com outro bloco
  pub fn collide_block( self, b: Bloco) -> bool {
    let diff_x = self.p1.x - b.p1.x;
    let diff_y = self.p1.y - b.p1.y;
    let tam1x = self.p2.x - self.p1.x;
    let tam1y = self.p2.y - self.p1.y;
    let tam2x = b.p2.x - b.p1.x;
    let tam2y = b.p2.y - b.p1.y;

    if 
      diff_x >= -tam1x &&
      diff_x <= tam2x  &&
      diff_y >= -tam1y &&
      diff_y <= tam2y 
      {
        return true;
      }
    
    return false;
  }


  /// Analisa a colisão com um quadrilátero
  pub fn collide_quad( self, q: Quadrilatero ) -> bool {
    let diff_x = self.p1.x - q.p.x;
    let diff_y = self.p1.y - q.p.y;
    let tam1x = self.p2.x - self.p1.x;
    let tam1y = self.p2.y - self.p1.y;

    if 
      diff_x >= -tam1x &&
      diff_x <= q.tx  &&
      diff_y >= -tam1y &&
      diff_y <= q.ty 
      {
        return true;
      }
    
    return false;

  } 
}


/// Métodos relacionados ao Quadrilatero
impl Quadrilatero {


  /// Gera um novo quadrilátero através de uma coordenada e de um ponto
  pub fn new( p: (i32, i32), tx: u16, ty: u16 ) -> Self {
    Self {
      p: Ponto::new(p.0, p.1),
      tx: tx as i32,
      ty: ty as i32
    }
  }
    

  /// Gera um novo quadrilátero através de um Ponto e suas dimensões
  pub fn from( p: Ponto, tx: u16, ty: u16) -> Self {
    Self {
      p,
      tx: tx as i32,
      ty: ty as i32
    }
  }


  /// Move um quadrilátero 
  pub fn mov(&mut self, v: Vetor) {
    self.p.mov(v);
  }


  /// Retorna um quadrilátero movimentado sem alterar o original
  pub fn sum(mut self, v: Vetor) -> Self{
    self.p.mov(v);
    return self;
  }


  /// Retorna uma coordenada correspondente ao centro do quadrilátero
  pub fn center( self ) -> (f64, f64) {
    let tx = self.tx as f64 / 2.0;
    let ty = self.ty as f64 / 2.0;
    let px = self.p.x as f64 + tx;
    let py = self.p.y as f64 + ty;
    (px, py)
  }


  /// Retorna um quadrilátero que engloba dois quadriláteros
  pub fn merge( q1: Quadrilatero, q2: Quadrilatero) -> Self {
    
    // Identificar x do p à esquerda
    let x1 = q1.p.x.min(q2.p.x);

    // Identificar y do p à esquerda
    let y1 = q1.p.y.min(q2.p.y);

    // Identificar o máximo à direita
    let f1 = q1.p.x + q1.tx;
    let f2 = q2.p.x + q2.tx;
    let x2 = f1.max(f2);
    
    // identificar o máximo acima
    let f1 = q1.p.y + q1.ty;
    let f2 = q2.p.y + q2.ty;
    let y2 = f1.max(f2);
    
    // Retorna
    let p = Ponto::new(x1, y1);
    let pf = Ponto::new(x2, y2);
    Self {
      p,
      tx: pf.x - p.x,
      ty: pf.y - p.y,
    }
  }
  

  /// Gera um novo quadrilátero através de um Bloco 
  pub fn into_block( self ) -> Bloco {
    // ponto inicial
    let p1 = self.p;
    
    // pontos da segunda coordenada
    let px = self.p.x + self.tx;
    let py = self.p.y + self.ty;
    Bloco {
      p1,
      p2: Ponto::new(px, py),
    }
  } 
  

  /// Analisa a colisão com um Bloco
  pub fn collide_block( self, b: Bloco) -> bool {
    let diff_x = self.p.x - b.p1.x;
    let diff_y = self.p.y - b.p1.y;
    let tam2x = b.p2.x - b.p1.x;
    let tam2y = b.p2.y - b.p1.y;

    if 
      diff_x >= -self.tx &&
      diff_x <= tam2x  &&
      diff_y >= -self.ty &&
      diff_y <= tam2y 
      {
        return true;
      }
    
    return false;
  }


  /// Analisa a colisão com um quadrilátero
  pub fn collide_quad( self, q: Quadrilatero ) -> bool {
    let diff_x = self.p.x - q.p.x;
    let diff_y = self.p.y - q.p.y;

    if 
      diff_x >= -self.tx &&
      diff_x <= q.tx  &&
      diff_y >= -self.ty &&
      diff_y <= q.ty 
      {
        return true;
      }
    
    return false;

  } 

}

/// Métodos relacionados ao círculo
impl Circulo {


  /// Gera um novo Círculo através de uma coordenada e um raio
  pub fn new( p: (i32, i32), r: f64) -> Self {
    Self {
      p: Ponto::new(p.0, p.1),
      r
    }
  }


  /// Gera um novo Círculo através de um ponto e um raio
  pub fn from( p: Ponto, r: f64) -> Self {
    Self {
      p,
      r
    }
  }


  /// Move o círculo
  pub fn mov( &mut self, v: Vetor) {
    self.p.mov(v);
  }


  /// Retorna um círculo movido sem alterar o original
  pub fn sum( self, v: Vetor) -> Self {
    Self {
      p: self.p.sum(v),
      r: self.r,
    }
  }


  /// Analisa a colisão com um círculo
  pub fn collide_circle( self, c: Circulo) -> bool {
    // a colisão ocorre se a distância entre os pontos for menor
    // que o raio do primeiro círculo com o do segundo
    let diff = Ponto::diff(self.p, c.p);

    if diff > self.r + c.r {
      return false;
    }

    return true;
  }
}
