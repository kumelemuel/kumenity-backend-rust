//! Dobles de los puertos salientes y baterías de contrato (TDD-2): código de
//! primera clase.
//!
//! Los dobles se nombran como adaptadores (HEX-5) con un prefijo de la lista
//! cerrada de TDD-2: `Fake` si tienen estado, `Fixed` o `Stub` si responden
//! fijo, `Spy` si registran llamadas. Cada doble pasa la misma batería de
//! contrato (`contracts`) que los adaptadores reales. Disponibles para los
//! tests de este crate y, con la feature `testing`, para los de otros crates.

use crate::{
    application::ports::outbound::community_repository::CommunityRepository,
    domain::model::community::community::Community,
};
use shared::Fault;
use std::{
    future::Future,
    pin::pin,
    sync::{Mutex, MutexGuard},
    task::{Context, Poll, Waker},
};

pub mod contracts;

/// Ejecuta un future sin runtime externo.
///
/// Pensado para fakes que resuelven de inmediato; no sirve para IO real.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        std::thread::yield_now();
    }
}

/// Operación de `CommunityRepository`, para hacer fallar solo esa en un
/// `FakeCommunityAdapter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommunityOperation {
    FindBySlug,
    Save,
}

/// Doble de `CommunityRepository`: guarda en memoria, o falla.
///
/// `failing()` hace fallar todas las operaciones. Para que un handler llegue a
/// una llamada posterior a la primera y falle esa, `failing_on(op)`: las demás
/// siguen el contrato (buscan lo guardado, `save` guarda). Es la forma de la
/// plantilla (craft-backend 1.0.0-rc.3), con el puerto todavía síncrono.
///
/// Casi igual a `InMemoryCommunityAdapter` mientras no haya persistencia real.
/// No se fusionan: este es un doble de application; aquel, un adaptador de
/// infraestructura (TDD-2).
pub struct FakeCommunityAdapter {
    communities: Mutex<Vec<Community>>,
    /// `failing()`: todas, también las que se agreguen al puerto.
    failing_all: bool,
    failing: Vec<CommunityOperation>,
}

impl FakeCommunityAdapter {
    pub fn empty() -> Self {
        Self {
            communities: Mutex::new(Vec::new()),
            failing_all: false,
            failing: Vec::new(),
        }
    }

    /// Simula una falla técnica no declarada (árbol B) en todas las
    /// operaciones.
    pub fn failing() -> Self {
        Self {
            failing_all: true,
            ..Self::empty()
        }
    }

    /// Además, `operation` responde un `Fault` (árbol B).
    pub fn failing_on(mut self, operation: CommunityOperation) -> Self {
        self.failing.push(operation);
        self
    }

    fn store(
        &self,
        operation: CommunityOperation,
    ) -> Result<MutexGuard<'_, Vec<Community>>, Fault> {
        if self.failing_all || self.failing.contains(&operation) {
            return Err(Fault::new("fake repository outage"));
        }
        Ok(self
            .communities
            .lock()
            .expect("invariante: mutex sin envenenar"))
    }
}

impl CommunityRepository for FakeCommunityAdapter {
    fn find_by_slug(&self, slug: &str) -> Result<Option<Community>, Fault> {
        let communities = self.store(CommunityOperation::FindBySlug)?;
        Ok(communities
            .iter()
            .find(|community| community.slug().as_str() == slug)
            .cloned())
    }

    fn save(&self, community: &Community) -> Result<(), Fault> {
        let mut communities = self.store(CommunityOperation::Save)?;
        // Mismo id: reemplaza, como cualquier repositorio (contrato).
        match communities.iter_mut().find(|c| c.id() == community.id()) {
            Some(existing) => *existing = community.clone(),
            None => communities.push(community.clone()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::community_repository_contract! {
        new_adapter: FakeCommunityAdapter::empty(),
        run: block_on,
    }

    fn community(slug: &str) -> Community {
        use crate::domain::model::community::{
            community_id::CommunityId, community_name::CommunityName,
            community_slug::CommunitySlug, owner_id::OwnerId,
        };
        Community::create(
            CommunityId::generate(),
            OwnerId::from_str("6f1c2a4e-8b3d-4c5e-9f7a-1b2c3d4e5f60")
                .expect("invariante: dueño de prueba válido"),
            CommunitySlug::new(slug.into()).expect("invariante: slug de prueba válido"),
            CommunityName::new("Rust Chile".into()).expect("invariante: nombre de prueba válido"),
            true,
        )
    }

    #[test]
    fn failing_on_save_fails_only_save() {
        let adapter = FakeCommunityAdapter::empty().failing_on(CommunityOperation::Save);

        assert!(adapter.save(&community("rust-chile")).is_err());
        assert!(matches!(adapter.find_by_slug("rust-chile"), Ok(None)));
    }

    #[test]
    fn failing_on_find_by_slug_fails_only_find_by_slug() {
        let adapter = FakeCommunityAdapter::empty().failing_on(CommunityOperation::FindBySlug);

        assert!(adapter.save(&community("rust-chile")).is_ok());
        assert!(adapter.find_by_slug("rust-chile").is_err());
    }
}
