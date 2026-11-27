/// Marcador de value object: igualdad por valor, inmutable. Sin ítems
/// (HEX-8): cada value object expone su propio `new` y su propio acceso.
pub trait ValueObject: Clone + Eq {}

/// Marcador de entidad: identidad propia, más allá de sus valores. Sin ítems
/// (HEX-8): cada entidad expone su propio `id()`.
pub trait Entity {}

/// Marcador de raíz de agregado. Sin ítems (HEX-8): cada agregado expone su
/// propio `id()` y su API, y el marcador no impone ninguna.
pub trait AggregateRoot {}

/// Marcador de evento de dominio: un hecho pasado e inmutable que el agregado
/// registra en sus transiciones (HEX-7). Si necesita su instante, lo recibe de
/// `Clock` al crearse.
pub trait DomainEvent: Clone {}
