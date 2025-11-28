use bstr::ByteSlice;

pub fn reverse(input: &str) -> String {
    let g = input.as_bytes().graphemes();
    return g.rev().collect();
}
