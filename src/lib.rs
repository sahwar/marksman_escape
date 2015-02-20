use std::str::Bytes;
use std::num::Int;

const LONGEST_ESCAPE: usize = 6;
static ZERO : [u8; LONGEST_ESCAPE + 1] = [0; LONGEST_ESCAPE + 1];
// Basic set of escapes: &, <, >, "
static AMP  : [u8; LONGEST_ESCAPE + 1] = [0x61, 0x6d, 0x70, 0x3b, 0, 0, 0];
static LT   : [u8; LONGEST_ESCAPE + 1] = [0x6c, 0x74, 0x3b, 0, 0, 0, 0];
static GT   : [u8; LONGEST_ESCAPE + 1] = [0x67, 0x74, 0x3b, 0, 0, 0, 0];
static QUOT : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x33, 0x34, 0x3b, 0, 0, 0];
// Less basic set, but important nevertheless: ' and `
// Grave is supposedly allowed in IE as attribute value wrapper.
static APOS : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x33, 0x39, 0x3b, 0, 0, 0];
static GRV  : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x39, 0x36, 0x3b, 0, 0, 0];
// Now these only matter in cases where attributes are not quoted.
static BANG : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x33, 0x33, 0x3b, 0, 0, 0];
static USD  : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x33, 0x36, 0x3b, 0, 0, 0];
static PERC : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x33, 0x37, 0x3b, 0, 0, 0];
static LPR  : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x34, 0x30, 0x3b, 0, 0, 0];
static RPR  : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x34, 0x31, 0x3b, 0, 0, 0];
static PLUS : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x34, 0x33, 0x3b, 0, 0, 0];
static EQ   : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x36, 0x31, 0x3b, 0, 0, 0];
static AT   : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x36, 0x34, 0x3b, 0, 0, 0];
static LBR  : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x39, 0x31, 0x3b, 0, 0, 0];
static RBR  : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x39, 0x33, 0x3b, 0, 0, 0];
static LBRK : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x31, 0x32, 0x33, 0x3b, 0, 0];
static RBRK : [u8; LONGEST_ESCAPE + 1] = [0x23, 0x31, 0x32, 0x35, 0x3b, 0, 0];

pub struct Escaped<'a> {
    inner: Bytes<'a>,
    idx: usize,
    inner_buffer: &'static [u8; LONGEST_ESCAPE + 1]
}

impl<'a> Escaped<'a> {
    pub fn new(i: &'a str) -> Escaped<'a> {
        Escaped {
            inner: i.bytes(),
            idx: 0,
            inner_buffer: &ZERO
        }
    }

    #[inline(always)]
    fn buf(&mut self, escape: &'static [u8; LONGEST_ESCAPE + 1]) -> Option<u8> {
        self.idx = 0;
        self.inner_buffer = escape;
        Some(0x26)
    }
}


impl<'a> Iterator for Escaped<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.inner_buffer[self.idx] != 0 {
            let r = self.inner_buffer[self.idx];
            self.idx += 1;
            return Some(r);
        }
        if let Some(ch) = self.inner.next() {
            match ch {
                0x26     => self.buf(&AMP),
                0x3E     => self.buf(&GT),
                0x3C     => self.buf(&LT),
                0x21     => self.buf(&BANG),
                0x22     => self.buf(&QUOT),
                0x24     => self.buf(&USD),
                0x25     => self.buf(&PERC),
                0x27     => self.buf(&APOS),
                0x28     => self.buf(&LPR),
                0x29     => self.buf(&RPR),
                0x2B     => self.buf(&PLUS),
                0x3D     => self.buf(&EQ),
                0x40     => self.buf(&AT),
                0x5B     => self.buf(&LBR),
                0x5D     => self.buf(&RBR),
                0x60     => self.buf(&GRV),
                0x7B     => self.buf(&LBRK),
                0x7D     => self.buf(&RBRK),
                _        => Some(ch)
            }
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, u) = self.inner.size_hint();
        (l, if let Some(u_) = u {u_.checked_mul(LONGEST_ESCAPE)} else {None})
    }
}
