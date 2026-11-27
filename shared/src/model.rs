/// Marcador de value object: igualdad por valor, inmutable. Sin ítems
/// (HEX-8): cada value object expone su propio `new` y su propio acceso.
pub trait ValueObject: Clone + Eq {}
