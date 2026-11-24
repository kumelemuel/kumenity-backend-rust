use crate::domain::model::community::community::Community;
use shared::Fault;

/// Persistencia de `Community`. No declara errores (ERR-3): cualquier falla
/// técnica es `Fault` (árbol B).
pub trait CommunityRepository: Send + Sync {
    /// "No existe" no es un error: `None` (ERR-1).
    fn find_by_slug(&self, slug: &str) -> Result<Option<Community>, Fault>;
    fn save(&self, community: &Community) -> Result<(), Fault>;
}
