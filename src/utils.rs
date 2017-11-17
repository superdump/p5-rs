fn map(iv: f64, il: f64, iu: f64, ol: f64, ou: f64) -> f64 {
    ol + (ou - ol) * (iv - il) / (iu - il)
}