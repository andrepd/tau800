/// Aqui temos de assegurar que aritmética, etc é modulo 2^6. 
/// Crates que há para ter um u6 / u12 não são práticas de usar
const CHAR_BIT: usize = 6;
pub type Word = u8;
pub type Address = u16;
/// Lá está, aqui é chato. Como converter um u6 para i6? Tem de ser à mão. 
pub type SWord = i8;  // Signed and same size as Word

pub const MAX_WORD: usize = 1 << CHAR_BIT;
pub const RAM_SIZE: usize = 1 << (2*CHAR_BIT);



pub fn word_to_sword(x: Word) -> SWord {
  debug_assert!((x as usize) < MAX_WORD);
  const HALF_MAX: usize = 1 << (CHAR_BIT - 1);
  if x < (HALF_MAX - 1) as Word {
    x as SWord
  } else {
    // Sanity check pls, also não sei fazer testes em rust
    /*-(MAX_WORD as Word - x) as SWord*/
    -((MAX_WORD - x as usize) as isize) as SWord  // Jfc
  }
}
