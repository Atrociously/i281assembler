pub struct Ident(String);

impl Ident {
    const VALID_CHARS: &'static str = "12345678ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";
}
