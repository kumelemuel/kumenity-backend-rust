//! Contrato de `CommunityRepository`: el comportamiento del que depende el
//! handler. Lo pasan el doble y todos los adaptadores reales.
//!
//! El puerto no declara errores (ERR-3): cualquier falla es `Fault`, así que no
//! hay casos por variante ni escenarios de la fábrica para provocarlas.
//!
//! Forma async de la receta "Puerto todavía síncrono" (D12, R16b): los casos y
//! los helpers son `async fn` y la macro recibe `run`. Los helpers, y los casos
//! que llaman al puerto directamente, lo hacen sin `.await` hasta R24, que lo
//! pasa a `BoxFuture`.
#![expect(
    clippy::unused_async,
    reason = "puerto todavía síncrono: se quita en su rebanada de asincronía"
)]

use crate::{
    application::ports::outbound::community_repository::CommunityRepository,
    domain::model::community::{
        community::Community, community_id::CommunityId, community_name::CommunityName,
        community_slug::CommunitySlug, owner_id::OwnerId,
    },
};

fn slug(value: &str) -> CommunitySlug {
    CommunitySlug::new(value.into()).expect("invariante: slug de prueba válido")
}

fn community_with_id(id: CommunityId, slug_value: &str) -> Community {
    Community::create(
        id,
        OwnerId::from_str("6f1c2a4e-8b3d-4c5e-9f7a-1b2c3d4e5f60")
            .expect("invariante: dueño de prueba válido"),
        slug(slug_value),
        CommunityName::new("Rust Chile".into()).expect("invariante: nombre de prueba válido"),
        true,
    )
}

fn community(slug_value: &str) -> Community {
    community_with_id(CommunityId::generate(), slug_value)
}

/// `Community` no implementa `PartialEq`: se compara por lo que expone.
fn snapshot(community: &Community) -> (CommunityId, CommunitySlug, CommunityName, bool) {
    (
        community.id().clone(),
        community.slug().clone(),
        community.name().clone(),
        community.is_public(),
    )
}

async fn save(adapter: &impl CommunityRepository, community: &Community) {
    adapter
        .save(community)
        .expect("contrato: save no falla con el adaptador sano");
}

async fn find(
    adapter: &impl CommunityRepository,
    value: &str,
) -> Option<(CommunityId, CommunitySlug, CommunityName, bool)> {
    adapter
        .find_by_slug(value)
        .expect("contrato: find_by_slug no falla con el adaptador sano")
        .as_ref()
        .map(snapshot)
}

/// Lo que se guarda se encuentra por su slug, igual a como se guardó.
pub async fn saves_and_finds_by_slug<R: CommunityRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();
    let saved = community("rust-chile");

    save(&adapter, &saved).await;

    assert_eq!(find(&adapter, "rust-chile").await, Some(snapshot(&saved)));
}

/// "No existe" no es un error: `find_by_slug` devuelve `None` (ERR-1).
pub async fn unknown_slug_is_none_not_an_error<R: CommunityRepository>(
    new_adapter: impl Fn() -> R,
) {
    let adapter = new_adapter();

    assert_eq!(find(&adapter, "rust-chile").await, None);
}

/// El slug se compara entero: ni prefijo ni extensión de uno guardado.
pub async fn find_by_slug_matches_the_whole_slug<R: CommunityRepository>(
    new_adapter: impl Fn() -> R,
) {
    let adapter = new_adapter();
    save(&adapter, &community("rust-chile")).await;

    assert_eq!(find(&adapter, "rust").await, None);
    assert_eq!(find(&adapter, "rust-chile-sur").await, None);
}

/// Con varias comunidades, cada slug encuentra la suya.
pub async fn finds_each_community_by_its_own_slug<R: CommunityRepository>(
    new_adapter: impl Fn() -> R,
) {
    let adapter = new_adapter();
    let chile = community("rust-chile");
    let peru = community("rust-peru");
    save(&adapter, &chile).await;
    save(&adapter, &peru).await;

    assert_eq!(find(&adapter, "rust-chile").await, Some(snapshot(&chile)));
    assert_eq!(find(&adapter, "rust-peru").await, Some(snapshot(&peru)));
}

/// Guardar un agregado con un id existente lo reemplaza: no lo duplica ni deja
/// visible el estado anterior.
pub async fn save_replaces_the_community_with_the_same_id<R: CommunityRepository>(
    new_adapter: impl Fn() -> R,
) {
    let adapter = new_adapter();
    let id = CommunityId::generate();
    save(&adapter, &community_with_id(id.clone(), "first")).await;
    let second = community_with_id(id, "second");
    save(&adapter, &second).await;

    assert_eq!(find(&adapter, "first").await, None);
    assert_eq!(find(&adapter, "second").await, Some(snapshot(&second)));
}

/// Genera un `#[test]` por caso del contrato de `CommunityRepository`.
///
/// `new_adapter`: expresión que construye un adaptador nuevo y vacío. Se
/// evalúa en un módulo hijo del que invoca la macro (`use super::*`).
///
/// `run`: ejecuta el future de cada caso (`block_on` para el doble y el
/// adaptador en memoria).
///
/// Un caso nuevo se agrega aquí y como `async fn` arriba.
#[macro_export]
macro_rules! community_repository_contract {
    (new_adapter: $new_adapter:expr, run: $run:expr $(,)?) => {
        mod community_repository_contract {
            #[allow(unused_imports)]
            use super::*;
            use $crate::application::testing::contracts::community_repository as contract;

            #[test]
            fn saves_and_finds_by_slug() {
                ($run)(contract::saves_and_finds_by_slug(|| $new_adapter));
            }

            #[test]
            fn unknown_slug_is_none_not_an_error() {
                ($run)(contract::unknown_slug_is_none_not_an_error(|| $new_adapter));
            }

            #[test]
            fn find_by_slug_matches_the_whole_slug() {
                ($run)(contract::find_by_slug_matches_the_whole_slug(|| {
                    $new_adapter
                }));
            }

            #[test]
            fn finds_each_community_by_its_own_slug() {
                ($run)(contract::finds_each_community_by_its_own_slug(|| {
                    $new_adapter
                }));
            }

            #[test]
            fn save_replaces_the_community_with_the_same_id() {
                ($run)(contract::save_replaces_the_community_with_the_same_id(
                    || $new_adapter,
                ));
            }
        }
    };
}
