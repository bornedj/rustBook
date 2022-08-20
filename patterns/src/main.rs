use patterns;
fn main () {
    patterns::match_pattern();
    patterns::if_let();
    patterns::while_let();
    patterns::destructure_for();

    // patterns that can be refutable or  irrefutable. Irrefutable patterns are patterns that cannot fail to match
    patterns::match_shallow();
    patterns::range_with_char();
    patterns::destruct_struct();
}