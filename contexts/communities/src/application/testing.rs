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
use std::sync::{Mutex, MutexGuard};

pub mod contracts;

/// Doble de `CommunityRepository`: guarda en memoria, o falla siempre.
///
/// Casi igual a `InMemoryCommunityAdapter` mientras no haya persistencia real.
/// No se fusionan: este es un doble de application; aquel, un adaptador de
/// infraestructura (TDD-2).
pub struct FakeCommunityAdapter {
    communities: Mutex<Vec<Community>>,
    failing: bool,
}

impl FakeCommunityAdapter {
    pub fn empty() -> Self {
        Self {
            communities: Mutex::new(Vec::new()),
            failing: false,
        }
    }

    /// Simula una falla técnica no declarada (árbol B).
    pub fn failing() -> Self {
        Self {
            communities: Mutex::new(Vec::new()),
            failing: true,
        }
    }

    fn store(&self) -> Result<MutexGuard<'_, Vec<Community>>, Fault> {
        if self.failing {
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
        let communities = self.store()?;
        Ok(communities
            .iter()
            .find(|community| community.slug().as_str() == slug)
            .cloned())
    }

    fn save(&self, community: &Community) -> Result<(), Fault> {
        let mut communities = self.store()?;
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
    }
}
