//! Contrato de `CommunityRepository`: el comportamiento del que depende el
//! handler. Lo pasan el doble y todos los adaptadores reales.
//!
//! El puerto no declara errores (ERR-3): cualquier falla es `Fault`, así que no
//! hay casos por variante ni escenarios de la fábrica para provocarlas.

use crate::{
    application::ports::outbound::community_repository::CommunityRepository,
    domain::model::community::{
        community::Community, community_id::CommunityId, community_name::CommunityName,
        community_slug::CommunitySlug,
    },
};
use iam::domain::model::account::account_id::AccountId;

fn slug(value: &str) -> CommunitySlug {
    CommunitySlug::new(value.into()).expect("invariante: slug de prueba válido")
}

fn community_with_id(id: CommunityId, slug_value: &str) -> Community {
    Community::create(
        id,
        AccountId::generate(),
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

fn save(adapter: &impl CommunityRepository, community: &Community) {
    adapter
        .save(community)
        .expect("contrato: save no falla con el adaptador sano");
}

fn find(
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
pub fn saves_and_finds_by_slug<R: CommunityRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();
    let saved = community("rust-chile");

    save(&adapter, &saved);

    assert_eq!(find(&adapter, "rust-chile"), Some(snapshot(&saved)));
}

/// "No existe" no es un error: `find_by_slug` devuelve `None` (ERR-1).
pub fn unknown_slug_is_none_not_an_error<R: CommunityRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();

    assert_eq!(find(&adapter, "rust-chile"), None);
}

/// El slug se compara entero: ni prefijo ni extensión de uno guardado.
pub fn find_by_slug_matches_the_whole_slug<R: CommunityRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();
    save(&adapter, &community("rust-chile"));

    assert_eq!(find(&adapter, "rust"), None);
    assert_eq!(find(&adapter, "rust-chile-sur"), None);
}

/// Con varias comunidades, cada slug encuentra la suya.
pub fn finds_each_community_by_its_own_slug<R: CommunityRepository>(new_adapter: impl Fn() -> R) {
    let adapter = new_adapter();
    let chile = community("rust-chile");
    let peru = community("rust-peru");
    save(&adapter, &chile);
    save(&adapter, &peru);

    assert_eq!(find(&adapter, "rust-chile"), Some(snapshot(&chile)));
    assert_eq!(find(&adapter, "rust-peru"), Some(snapshot(&peru)));
}

/// Guardar un agregado con un id existente lo reemplaza: no lo duplica ni deja
/// visible el estado anterior.
pub fn save_replaces_the_community_with_the_same_id<R: CommunityRepository>(
    new_adapter: impl Fn() -> R,
) {
    let adapter = new_adapter();
    let id = CommunityId::generate();
    save(&adapter, &community_with_id(id.clone(), "first"));
    let second = community_with_id(id, "second");
    save(&adapter, &second);

    assert_eq!(find(&adapter, "first"), None);
    assert_eq!(find(&adapter, "second"), Some(snapshot(&second)));
}

/// Genera un `#[test]` por caso del contrato de `CommunityRepository`.
///
/// `new_adapter`: expresión que construye un adaptador nuevo y vacío. Se
/// evalúa en un módulo hijo del que invoca la macro (`use super::*`).
///
/// Un caso nuevo se agrega aquí y como función arriba.
#[macro_export]
macro_rules! community_repository_contract {
    (new_adapter: $new_adapter:expr $(,)?) => {
        mod community_repository_contract {
            #[allow(unused_imports)]
            use super::*;
            use $crate::application::testing::contracts::community_repository as contract;

            #[test]
            fn saves_and_finds_by_slug() {
                contract::saves_and_finds_by_slug(|| $new_adapter);
            }

            #[test]
            fn unknown_slug_is_none_not_an_error() {
                contract::unknown_slug_is_none_not_an_error(|| $new_adapter);
            }

            #[test]
            fn find_by_slug_matches_the_whole_slug() {
                contract::find_by_slug_matches_the_whole_slug(|| $new_adapter);
            }

            #[test]
            fn finds_each_community_by_its_own_slug() {
                contract::finds_each_community_by_its_own_slug(|| $new_adapter);
            }

            #[test]
            fn save_replaces_the_community_with_the_same_id() {
                contract::save_replaces_the_community_with_the_same_id(|| $new_adapter);
            }
        }
    };
}
