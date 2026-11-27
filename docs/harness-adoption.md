# Adopción del harness craft-backend

> **Dónde está cada cosa.** Aquí, lo vivo: cabecera, filas abiertas del inventario (§2), dudas abiertas (§4), reglas (§5.1), el índice de las rebanadas pendientes (§5.3) con sus dependencias (§5.4) y sus cambios de contrato (§5.5), cierre de los tests de arquitectura (§5.6), decisiones (§5.7, una línea por decisión), el proceso de los puntos de replanificación (§5.9) y las fricciones sin reportar (§6). En [`harness-adoption/archive.md`](harness-adoption/archive.md), lo cerrado: el resumen de la auditoría (§1), las filas resueltas o descartadas, §3, las dudas resueltas, §5.2, las rebanadas hechas, las filas hechas de §5.5, P1 y P2, las sincronizaciones S1 a S4 (§5.10), la historia de §5.6, el texto completo de las decisiones (§5.7), las fricciones corregidas y el método. En [`harness-adoption/costs.md`](harness-adoption/costs.md), el registro de rebanadas con sus costos (§5.8). En `harness-adoption/slices/<Rn>.md`, el detalle de cada rebanada pendiente; en `harness-adoption/archive/`, el de las que se cierran desde R21 (las anteriores siguen en `archive.md`).
>
> **Qué lee cada sesión.** Una sesión de rebanada (`/craft-backend:adoption R<n>`) lee este documento entero y el archivo de su rebanada, `harness-adoption/slices/R<n>.md`; `archive.md` y `costs.md`, solo si los necesita. Al cerrar la rebanada, en el mismo PR, mueve ese archivo a `harness-adoption/archive/` (`git mv`) y borra su línea del índice de §5.3; si con eso otra rebanada queda sin dependencias pendientes, su estado pasa a *lista*.

Este archivo guarda el estado entre sesiones de la adopción de los principios craft-backend (HEX, ERR, TDD, VER, RS) en este repositorio.

| Dato | Valor |
|---|---|
| Fecha de la auditoría | 2026-11-19; auditorías completas de P1: 2026-11-23 y de P2: 2026-11-26 |
| Plugin | `craft-backend@ai-harness` 0.12.0 en la auditoría; `tools/architecture` de `v1.0.0-rc.4` desde S4 |
| Commit auditado | `d0b0eeb` (rama `ci/sync-development`); P1: `bab68c5` (`main`, después de R10b); P2: `0eaab25` (`main`, después de R19) |
| Revisor | `craft-backend:architecture-reviewer`, auditoría completa en solo lectura |
| Hallazgos | 131: 26 de severidad alta, 55 media, 50 baja. Desglose por auditoría y revisión en [`archive.md`](harness-adoption/archive.md#cabecera-y-prioridades-lo-resuelto). |

## Cómo mantener este archivo

- Al resolver un hallazgo, cambiar su **Estado** de `abierto` a `resuelto (<commit>)`. Si se descarta, poner `descartado` y el motivo. En el mismo PR, la fila pasa a `harness-adoption/archive.md`, a la tabla de su regla; lo mismo con una fricción corregida. El archivo de una rebanada hecha pasa de `slices/` a `archive/`. Una fila nueva de §5.8 va en `harness-adoption/costs.md`.
- Los IDs de hallazgo (`HEX-1.3`, `ERR-4.2`…) son estables: no se renumeran. Un hallazgo nuevo recibe el siguiente número libre de su regla.
- Después de cada slice de refactor, volver a auditar con architecture-reviewer y actualizar la cabecera (fecha, commit, conteos).
- Tests de caracterización en `app/tests/` (PR #18): son la red de seguridad del refactor y tienen que seguir pasando.
- Cada rebanada se hace con `/craft-backend:adoption R<n> [notas]`. Las reglas propias de este proyecto, que la skill no trae, están en §5.1 ("Ritual de rebanada").

## Prioridades inmediatas

1. **ERR-4.1:** `.env` ya no se versiona (0df9baa), pero `JWT_SECRET` sigue en el historial: **rotar el secreto** en cada entorno donde se haya usado.
2. **HEX-2 / HEX-7 (fase 4):** sacar `uuid`, `rand` y `chrono` del núcleo (R20a, R20b y R21a hechas; queda R22 para `uuid`, y `rand` sale con la separación de R28), la condición para separar crates («Precondiciones (P2)» de `slices/R27.md` y `slices/R28.md`).
3. **RS-1 / HEX-1 / HEX-2:** separar cada contexto en crates por capa, empezando por `communities` (R27). HEX-1.1 ya está resuelto (f6bc1fd).

## 1. Resumen

En [`archive.md`](harness-adoption/archive.md#1-resumen): es la foto de la auditoría inicial. Los conteos de hoy están en la cabecera.

## 2. Inventario

### HEX (hexagonal-architecture) y VER

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| HEX-1.6 | HEX-1 Autenticación | app/src/middleware/auth.rs:17 | El middleware vive en el composition root, no en el `api` de `iam`. Inyecta `VerifiedIdentity` (de `shared::domain`, línea 36) en vez de un tipo del borde. Desde R10b consume `VerifyAccessTokenUseCase`, pero importa `iam::infrastructure::http::errors::error_mapper` (línea 12), un detalle del `api` de iam. | alta | `IamApi::protect(routes)` en `iam-api`, que inserta `edge_http::AuthenticatedAccount`. | abierto: desde R11 (7655856) el middleware importa `iam::infrastructure::http::unexpected` en lugar del mapper; sigue siendo un detalle del HTTP de iam (R28) |
| HEX-1.9 | HEX-1 Dependencias | contexts/iam/src/infrastructure/mod.rs:1; contexts/communities/src/infrastructure/mod.rs:1 | El adaptador driving (`http`) es un submódulo de `infrastructure`: `api` e `infrastructure` no están separados y pueden importarse entre sí. | alta | Crate `{ctx}-api` separado (RS-1). | abierto: R27 (communities) y R28 (iam) |
| HEX-2.1 | HEX-2 Núcleo sin framework | contexts/iam/Cargo.toml:11-17 | `domain` y `application` comparten crate con `uuid`, `argon2`, `rand`, `jsonwebtoken`, `serde`, `chrono` y `axum`, así que todos están disponibles en el núcleo. | alta | Crates `iam-domain` e `iam-application` con solo el kernel. | abierto: R28. *(P2)* Antes salen `uuid` (R20a, hecho en d461bbc) y `chrono` (R21); quedan `argon2`, `axum`, `jsonwebtoken`, `rand` y `serde`, que la separación deja fuera de domain y application. *(R20b)* `rand` ya no se usa en el dominio (8976a8e): solo en `infrastructure/security` (Argon2 y `RandVerificationCodeAdapter`). *(R21a)* `chrono` salió (a93bb86) |
| HEX-2.5 | HEX-2 Núcleo sin framework | contexts/communities/Cargo.toml:11-14 | `uuid`, `axum`, `serde` y `http` (este último, visto en S1) están disponibles en `domain` y `application`. | alta | Separar crates por capa. | abierto: R27. *(P2)* Antes salen `uuid` (R22, con HEX-2.7) y `http` (R22, RS-1.5); quedan `axum` y `serde`, que la separación deja en `communities-api` |
| HEX-2.6 | HEX-2 Núcleo sin framework | contexts/communities/src/domain/model/community/community_id.rs:2 | El dominio usa `uuid`. | media | `IdGenerator` + value object sobre `String`. | abierto: R22. *(P2)* `owner_id.rs` también lo usa: HEX-2.7. *(S4)* Entrada `hex_2_core_imports_…` del archivo: R22 −1 |
| HEX-2.7 | HEX-2 Núcleo sin framework | contexts/communities/src/domain/model/community/owner_id.rs:2 | `OwnerId` valida con `uuid` (`Uuid::parse_str`, `is_nil`). Entró en R16 (3646f25) sin que fallara nada: en legacy, la entrada `hex_2_core … communities#dependencies.uuid` es por manifiesto y ya existía (VER-1.8). HEX-2.6 solo nombraba `community_id.rs`, así que `uuid` habría seguido en el dominio después de R22 y R27 habría creado `communities-domain` con una violación nueva. Hallado en P2. | media | `OwnerId(String)`, validado a mano con std (forma con guiones, hexadecimal, sin nil), con `InvalidOwnerId { Nil, WrongFormat }` (D2 a) e `impl FromStr` (RS-2.6). | abierto: R22. *(S4)* Entrada `hex_2_core_imports_…` del archivo: R22 −1 |
| HEX-6.1 | HEX-6 Transporte | iam/…/requests/sign_up.rs:5, sign_in.rs:5, verify.rs:5, identify.rs:5; responses/signed_up.rs:5, signed_in.rs:5, identified.rs:5; communities/…/requests/create.rs:5, responses/created.rs:5 | Los DTOs HTTP no son `{Caso}HttpRequest`/`{Caso}HttpResponse`, y `CreateRequest` colisiona con el vocabulario de puerto saliente. | baja | `RegisterAccountHttpRequest`, `CreateCommunityHttpResponse` y demás. | abierto: R27 (communities) y R28 (iam) |
| HEX-7.3 | HEX-7 Inmutabilidad | contexts/iam/src/domain/model/account/mod.rs *(R20b; antes account.rs:94-140)* | El agregado es mutable (`&mut self`): `change_password`, `change_email`, `deactivate`, `confirm_registration`… | media | `fn confirm(&self, …) -> Result<Self, …>`, que devuelve el estado nuevo. | abierto: R21b. *(P2)* Con HEX-7.13, solo `confirm_registration` tiene llamador de producción: los demás se borran en lugar de volverse inmutables |
| HEX-7.6 | HEX-7 Identificador | contexts/iam/src/domain/model/account/account_factory.rs:63 (y `CodeValidation::generate` en la l.59); contexts/communities/src/application/commands/create_community/handler.rs:72 *(P2; antes account_factory.rs:21 y create_community.rs:48)* | El id se genera en el dominio (`AccountId::generate`, `CommunityId::generate`) en vez de recibirse de `IdGenerator`. | media | Inyectar `Arc<dyn IdGenerator>` en el handler y pasar el id. | abierto: R22 (communities). iam resuelto en R20a (d461bbc): `RegisterAccountHandler` recibe el `IdGenerator` y `AccountFactory::create`, el id; el composition root cablea `UuidIdGeneratorAdapter` (5b49e36). *(R20b)* `CodeValidation::generate` se borró (8976a8e): el código llega de `VerificationCodeService`, y `Account::create` recibe el id y el código |
| HEX-7.8 | HEX-7 Servicio de dominio | contexts/communities/src/domain/services/membership_policy.rs:2 | `MembershipPolicy` es un enum de opciones (value object) archivado en `services/`. | baja | Moverlo a `model/community/`. | abierto: R22 (D4: se retira) |
| HEX-7.9 | HEX-7 Comportamiento | contexts/communities/src/domain/model/community/community.rs:19-56 | Agregado anémico: `membership_policy` siempre es `None` y `MembershipPolicyChanged` (membership_policy_changed.rs:4) no se usa. | baja | Agregar el comportamiento con su evento, o retirar lo muerto. | abierto: R22 (D4: se retira) |
| HEX-7.13 | HEX-7 Comportamiento | contexts/iam/src/domain/model/account/mod.rs *(R20b; antes account.rs:94-124)* | `activate`, `deactivate`, `suspend`, `change_password`, `change_email` y `change_username` no tienen llamador de producción; solo los usan sus tests. Además, `activate` decide con `self.status.as_str() == "registered"` (l.116), comparando la representación de texto, que es también el formato de identify. Hallado en P2. | baja | Criterio de D4 (sin comportamiento ni uso): borrarlos con sus tests en R21. Si alguno se conserva, que decida con `matches!(self.status, AccountStatus::Registered { .. })`. | abierto: R21b |
| HEX-7.14 | HEX-7 Identificador | contexts/iam/src/domain/model/account/account_id.rs:26-39; contexts/communities/src/domain/model/community/community_id.rs:12-25 | `AccountId::from_uuid`/`from_str` y `AccountIdError` (errors/account_id.rs:4) no tienen llamador de producción desde f6bc1fd (R16), y `CommunityId::from_uuid`/`from_str` e `InvalidCommunityId` (errors/invalid_community_id.rs:4) nunca lo tuvieron. Sostienen el `uuid` de HEX-2.2 y HEX-2.6 y los `#[allow(clippy::should_implement_trait)]` de account_id.rs:35 y community_id.rs:21. Hallado en P2. | baja | Borrarlos con sus tests, en lugar de convertirlos en `FromStr`: los ids nacen del `IdGenerator`. | abierto: R22 (communities). iam resuelto en R20a (bae89e9): `from_uuid`, `from_str` con su `allow`, `as_uuid` y `AccountIdError` borrados con sus 4 tests |
| HEX-8.1 | HEX-8 Kernel sin dependencias | shared/Cargo.toml:7-8 | El kernel depende de `chrono` y `serde`. | alta | Kernel solo `std`. | abierto: R25 (`serde`). *(R21a)* `chrono` salió (a93bb86) |
| HEX-8.6 | HEX-8 Sin modelos | shared/src/domain/model/verified_identity.rs:2; shared/src/application/auth_context.rs:2 | Tipos de identidad en el kernel. | media | `AuthenticatedAccount` en `edge/http`. | abierto: R18 (a603969) borró `AuthContext` (sin usos tras quitar `ListPublicCommunities`); queda `VerifiedIdentity` (R27) |
| HEX-8.8 | HEX-8 Kernel | shared/src/infrastructure/http/api_error_response.rs:4 | Un tipo de transporte HTTP (con `serde`) en el kernel. | media | `edge/http` con `Problem`. | abierto: R25 *(P2; antes R30)*: sale del kernel como una copia por contexto, y R30 borra las copias. *(S4)* Entrada `hex_2_core_imports_…` del archivo (`serde`): R25 −1 |
| VER-1.7 | VER-1 Lints | Cargo.toml:40 (`[workspace.lints.clippy]`; `unwrap_used` en la l.43) *(P2; antes :41)* | `[workspace.lints.clippy]` solo prohíbe `unwrap_used`. Nada impide un `todo!()` (RS-2.2) o un `dbg!` (ERR-4.2) nuevo. Hallado en P1. | baja | `todo = "deny"` y `dbg_macro = "deny"`. | abierto: R21b. *(R21a)* Ya no queda ningún `todo!()` en producción (78a7776, RS-2.2 resuelto): la condición se cumple |

### ERR (error-handling)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| ERR-1.8 | ERR-1 Canal declarado | app/src/config/app.rs:11 | `AppConfig::load()` declara `Result<_, ConfigError>` pero termina el proceso dentro con `process::exit(1)`: la rama `Err` de main.rs:5 nunca corre. Hallado en P1. | baja | `let jwt = JwtConfig::from_env()?;`. | abierto: R26 |
| ERR-2.12 | ERR-2 Error por regla | contexts/iam/src/domain/errors/account.rs:4 | `AccountError`, un error con nombre de agregado, tiene una sola variante (`InvalidVerification`) y un solo productor (`confirm_registration`). Es el patrón de error global de dominio que ERR-2.11 propuso reemplazar: una variante nueva obliga a cada `match` del handler a traducir variantes que esa operación no produce. Hallado en P2. | baja | Un error propio de la transición (`InvalidVerification`, como `CannotAuthenticate`), que devuelve el nuevo `confirm(&self, …) -> Result<Self, InvalidVerification>`. | abierto: R21b |
| ERR-3.4 | ERR-3 Error declarado | contexts/iam/src/application/ports/outbound/account_repository.rs (`save`); register_account/handler.rs:92 *(P2; antes :88)* | `save -> Result<(), Fault>` no declara el duplicado: con una carrera entre `find_*` y `save`, un adaptador con índice único respondería un 500 en lugar del `Conflict`. Hoy no pasa: el adaptador en memoria no tiene índices. Hallado en la revisión de R11. | baja | Al llegar la persistencia real, `AccountRepositoryError::Duplicate { resource }` declarado en el puerto, con su caso en la batería, traducido a la hoja `Conflict`. | abierto: sin rebanada (con el primer adaptador persistente) |
| ERR-4.1 | ERR-4 Secretos | .env:1 | `JWT_SECRET` está versionado; `.gitignore:1-2` no excluye `.env`. | alta | Sacar `.env` del índice y rotar el secreto; `.env.example` sin valores. | en curso (0df9baa): fuera del índice; falta rotar el secreto |
| ERR-4.6 | ERR-4 Problem Details | shared/src/infrastructure/http/api_error_response.rs:4; app/src/middleware/auth.rs:29, 42 | Las respuestas de error usan `{code,message}` y el 401 va con cuerpo vacío, no en RFC 9457. | media | `edge_http::Problem` con un helper por hoja. | abierto: R30 (D5 decidida) |
| ERR-4.7 | ERR-4 Manejador global | app/src/server.rs:24-27 | No hay manejador del árbol B, ni `FaultReporter`, ni `CatchPanicLayer`. Los `todo!`/`expect` cortan la conexión y las fallas técnicas no se registran. | media | `CatchPanicLayer` + `unexpected()` que reporta y devuelve un 500 genérico. | abierto: R5 (29890cb) existe el trait `FaultReporter`; desde R11 (7655856) `unexpected()` vive en `iam/…/infrastructure/http/unexpected.rs` y lo usan los `fold` de iam y el middleware (el mapper, hasta R19), con el `Fault` en stderr como interino; R17 (9614aaa) sacó el de communities a `communities/…/infrastructure/http/unexpected.rs`, que usan el `fold` de create y el mapper; faltan el adaptador, `edge/http` y `CatchPanicLayer` (R26, R30). *(S4)* Con `edge/http` de rc.4, `unexpected(&fault, reporter).with_code(codes::INTERNAL_ERROR)` reemplaza a los dos `unexpected.rs` (R30) |
| ERR-4.8 | ERR-4 Identidad faltante | contexts/communities/src/infrastructure/http/create_community.rs:27 (hasta R17, `handlers/community_handlers.rs:18`) | `Extension<VerifiedIdentity>` obligatoria: en una ruta sin proteger responde 500, no 401. | baja | `Option<Extension<AuthenticatedAccount>>` → 401. | abierto: R27 |
| ERR-4.12 | ERR-4 Manejador global | app/src/main.rs:11 | Un `Fault` al arrancar se imprime con `Display`, que es opaco a propósito: el operador solo ve "unexpected failure", no la causa (p. ej. el hash del señuelo). Hallado en P1. | baja | Imprimir `detail()` o pasarlo por el `FaultReporter`. | abierto: R26 |
| ERR-4.13 | ERR-4 `fold` exhaustivo | contexts/iam/src/infrastructure/http/register_account.rs:39; contexts/communities/src/infrastructure/http/create_community.rs:53, 68 (desde R17) | El borde elige `IAM_PASSWORD_TOO_SHORT` solo porque `field == "password"`: la hoja no lleva el motivo. Si `PasswordPolicyError` gana una variante, el cliente recibe "too short" sin que falle ningún test. Hallado en la revisión de R11. | baja | Un motivo en la hoja o un conjunto cerrado de motivos, junto con Problem Details. *(S4)* La forma de referencia de 1.0.0-rc.3.2: `Validation { field: Field }` con un `enum Field` cerrado por contexto, y un campo con varios motivos lleva el motivo en su variante (`Field::Owner(OwnerRejection)`, con `enum OwnerRejection { Nil, WrongFormat }`). No depende de Problem Details. | abierto: **R22b** *(S4, propuesta; antes R30; confirmada en D15)*. *(R17)* El mismo patrón en communities: el borde elige por `field` (`slug`, `name`) y `resource` (`slug`); hoy cada campo tiene un solo motivo (`InvalidCommunitySlug` e `InvalidCommunityName` son structs unitarios). El owner ya tiene su conjunto cerrado local, la hoja `InvalidOwner { Nil, WrongFormat }` (D2 a); cuando `Validation` lleve un motivo cerrado, puede fundirse en `Validation { field: "owner_id", reason }` |

### TDD (tdd)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| TDD-1.8 | TDD-1 Test que lo exija | app/src/containers/iam.rs | La propagación del `Fault` del hash del señuelo al arrancar (R9) no tiene test: `IamContainer::initialize` construye Argon2 por dentro y no admite un doble que falle. Hallado en R9 y registrado en P1. | baja | Test de arranque con un hasher que falla, cuando `IamApi` reciba sus adaptadores. | abierto: R28 |
| TDD-2.9 | TDD-2 Batería completa | contexts/communities/src/application/testing/contracts/community_repository.rs:44 | `snapshot` compara `id`, `slug`, `name` e `is_public`, pero no el dueño: `Community` no expone `owner_id()` (el campo es `_owner_id`, community.rs:12). Un adaptador que pierda o confunda el dueño pasa la batería, y ningún test de handler puede afirmar que la comunidad se guardó con el `owner_id` del comando. Hallado en P2. | media | `Community::owner_id() -> &OwnerId`, el dueño en `snapshot`, y afirmarlo en `create_community_successfully`. | abierto: R22 |
| TDD-2.10 | TDD-2 Doble completo | contexts/iam/src/application/testing.rs:214 | `SpyPasswordHashingAdapter` no tiene `failing()` ni `failing_on`, aunque su puerto puede fallar. Hallado en P2. | baja | Delegar en el `FixedPasswordHashingAdapter` interno: `failing()`/`failing_on(op)`. | abierto: R23 |

### RS (rust-stack)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| RS-1.1 | RS-1 Crate por capa | Cargo.toml:2-5 | Un crate por contexto (`contexts/*`), con las capas como módulos; no hay `kernel/` ni `edge/http`. | alta | `contexts/{ctx}/{domain,application,infrastructure,api}` (migración incremental). | abierto: R27 y R28 |
| RS-1.2 | RS-1 Kernel | shared/Cargo.toml:2 | El kernel se llama `shared` y tiene capas propias (`application`, `infrastructure/http`). | media | `kernel/` (solo std) + `edge/http`. | abierto: R29 |
| RS-1.5 | RS-1 Dependencias | contexts/communities/Cargo.toml:12 | `http` está declarado y no se usa: desde 58958bc (R17), los `StatusCode` de communities vienen de `axum::http`. Sostiene la entrada `hex_2_core … communities#dependencies.http 2` sin ningún uso. Hallado en P2. | baja | Quitarlo del manifiesto y borrar su entrada en el mismo PR (§5.1, "Una dependencia, una entrada"). | abierto: R22 |
| RS-1.6 | RS-1 Dependencias | app/Cargo.toml:10, 12 | `serde` y `tower` están en `[dependencies]` de `app`, y ningún archivo de producción los usa: `tower::ServiceExt` solo aparece en el `mod tests` de `middleware/auth.rs:57`, y `tower` ya está en `[dev-dependencies]` con `util`. Como `anyhow` (RS-1.4). Hallado en P2. | baja | Quitar los dos de `[dependencies]`; `tower` sigue en `[dev-dependencies]`. | abierto: R26 |
| RS-2.6 | RS-2 Lints | contexts/communities/src/domain/model/community/owner_id.rs:12 | `OwnerId::from_str` es un método inherente bajo `#[allow(clippy::should_implement_trait)]`, copiado de `AccountId` en R16 (3646f25). Silencia el lint en lugar de implementar `FromStr`. R20 quita los de `model/account` y R22 los de `CommunityId`, pero ninguna rebanada nombraba el de `OwnerId`. Hallado en la revisión de R16. | baja | `impl FromStr for OwnerId` (`type Err = InvalidOwnerId`) y quitar el `allow`; los llamadores pasan a `.parse()` o `OwnerId::from_str` por el trait. | abierto: R22 |
| RS-3.1 | RS-3 Puertos async | iam/…/ports/outbound/*.rs; communities/…/community_repository.rs:6; los 5 `ports/inbound/*.rs` (`list_public_communities_use_case.rs` se borró en R18 y `verify_access_token_use_case.rs` llegó en R10b) | Todos los puertos son síncronos: un adaptador con E/S real (Postgres) no cabe. Además, Argon2 bloquea el runtime dentro de handlers async. | media | `BoxFuture<'a, Result<…>>` del kernel. | abierto: R16b (d3ab538, fdc1941) dio a las 4 baterías de contrato la forma async, con `run: block_on`; los puertos pasan a `BoxFuture` en R23 y R24. *(P2)* `spawn_blocking` para Argon2 pasa a R28: en legacy, `tokio` en `contexts/iam/Cargo.toml` sería una entrada nueva de `hex_2_core` |

## 3. Reglas que se cumplen

En [`archive.md`](harness-adoption/archive.md#3-reglas-que-se-cumplen).

## 4. Dudas y hallazgos plausibles sin confirmar

No cuentan en el inventario. Si se confirman, pasan a él con el siguiente ID libre de su regla.

- **El señuelo y los parámetros de Argon2 (R2).** `verify` usa los parámetros del hash guardado. Si `Argon2::default()` cambia al actualizar el crate, el señuelo (generado al arrancar con los parámetros nuevos) y los hashes viejos dejan de costar lo mismo. Hoy no pasa; habría que revisarlo al subir `argon2` o al cambiar sus parámetros.
- **`Account` deriva `Debug` con `HashedPassword` dentro** (account.rs:14). Cualquier log futuro con `{:?}` filtra el hash. No viola nada hoy, quitando el `dbg!` de ERR-4.2. *(P2)* También imprime el código de verificación pendiente, dentro de `AccountStatus::Registered`.
- **Tests de caracterización en `app/tests/`.** Prueban reglas de negocio (longitudes, formatos) por el endpoint, y con adaptadores reales en vez de `Stub…UseCase`. Es la violación típica de TDD-2, pero están ahí a propósito como red de seguridad para el refactor. No los conté.
- **Verificar dos veces** devuelve `InvalidVerification` (400). Podría ser `Conflict`, pero depende de la semántica del producto (ERR-2).
- **Tratar `shared/` como el kernel.** Según rust-stack, los tests de arquitectura ignorarían `shared` como crate *legacy*. Se auditó contra HEX-8 porque hoy cumple ese papel. Si se decide que no es el kernel, HEX-8.1 a HEX-8.9 pasan a ser hallazgos de HEX-1/HEX-2 (un crate compartido entre contextos).
- **`contexts/iam/Cargo.lock` y `.idea/` versionados.** No violan ningún principio; no se contaron.
- **Enumeración por `identify` y `verify` (P1).** `/auth/identify` (público) y `/auth/verify` responden 404 o 200 según exista la cuenta, e `identify` además expone el estado. Queda la enumeración que D6 cerró en el login.
- **Código de verificación sin límite de intentos (P1).** Son 6 dígitos y nada limita los intentos.
- **Nadie entrega el código de verificación (P1).** Ningún puerto lo envía al usuario, así que la verificación solo se completa en los tests.
- **`PasswordHashingService::verify` recibe `&str` (R20b).** Desde R20b, `hash` responde un `HashedPassword`, pero `verify` sigue recibiendo el hash como `&str`. No viola ninguna regla: el hash guardado ya es un `HashedPassword` en la cuenta, y el señuelo también. Lo vio el revisor de R20b. Si se decide simetría, cabe en R23, que cambia la firma del puerto.
- **`HashedPasswordError` como respuesta de cliente (400).** Solo se produce si el hasher devuelve algo raro, así que sería árbol B. Queda dentro de ERR-1.1 y no aparte.

## 5. Plan de rebanadas

Las rebanadas se citan como **R1…R31** (con las intermedias R10a, R10b, R12b, R13b, R16b, R20a, R20b, R22b y R29b) y las sincronizaciones, como **S1…S4**. Cada una es un PR que tiene que estar verde en CI. En ese mismo PR se actualiza el inventario (`resuelto (<commit>)`) y, al terminar, se vuelve a auditar con `craft-backend:architecture-reviewer`. El avance de cada rebanada se anota en §5.8.

Datos verificados que condicionan el plan:
- Un contexto legacy no puede depender de un crate `edge/*`. Todas las capas virtuales comparten el manifiesto, y `(Domain, Edge)` está prohibido. En `iam` aparecería una entrada nueva en `contexts/iam/Cargo.toml`. En `communities` seguiría viva la entrada de HEX-1.1.
- `hex_2_core_depends_only_on_std_and_the_kernel` cuenta también las `[dev-dependencies]`. Después de la separación, los tests de application no pueden usar tokio.
- Los tests de caracterización fijan `IAM_INVALID_ACCOUNT_ID(_FORMAT)` en `/communities/create`, los bodies `{code,message}` exactos y el 401 con cuerpo vacío. Además documentan como "CURRENT BEHAVIOR" el duplicado por mayúsculas (HEX-7.1).
- `AccountStatus::from_str` no se usa fuera de los tests: ningún adaptador persiste en texto. *(P2)* `AccountFactory::reconstitute` ya no existe (R19, 0a3c49a).
- Decidido por el usuario: `account_lifecycle.rs` admite **adaptación mecánica**.

### 5.1 Reglas transversales

- **Trinquete.** Cuando un PR mueve o renombra un archivo que figura en `baseline.txt`, en ese mismo PR la violación tiene que desaparecer. Si no, el archivo no se toca. Antes de abrir cada PR: `cargo test -p architecture`.
- **Red de seguridad.** `app/tests/` no se modifica, salvo en los cambios de contrato listados en §5.5.
- **Ritual de rebanada (`/craft-backend:adoption R<n>`).** Lo que este proyecto agrega a la skill. Antes estaba en `.claude/commands/slice.md`:
  - commits en inglés, con Conventional Commits;
  - si el archivo de la rebanada (`slices/R<n>.md`) lo indica, se quitan los `#[allow(...)]` que apuntan a sus hallazgos;
  - cada rebanada termina en un PR contra `main`, aunque la invocación no lo pida, con la fila de §5.8 en `Tokens: pendiente` hasta que el usuario pase el `/cost`.
- **`account_lifecycle.rs` (decidido: adaptación mecánica).** Se permite cambiar imports, nombres, `.await`/`block_on` y reemplazar `code()` por un `matches!` sobre la variante equivalente. Ningún escenario ni aserto cambia de significado. El PR enumera cada adaptación.
- **Helpers de `app/tests/common/` (decidido en P1: adaptación mecánica, como lifecycle).** Se permite cambiar imports, nombres, la construcción de la app (como el `.expect("app builds")` de R9) y `.await`, sin que cambie ningún escenario ni aserto. El PR enumera cada adaptación. Los cambios de contrato de §5.5 (p. ej. `assert_error` en R30) siguen siendo cambios de contrato, no adaptaciones.
- **Fallas por operación en los dobles (desde R8).** Cada `Fake…Adapter` tiene `failing()` y `failing_on({Concepto}Operation)`, y cada llamada al puerto de un handler tiene su `fails_with_unexpected_when_<operación>_fails`. `communities` lo adopta en R16.
- **TDD.** El "rojo" de los tests de arquitectura es la entrada de la línea base. Al corregir la violación, el test falla con "Ya no violan: borrar", y el PR borra esa línea. Los tests que hoy afirman sobre `code()` se reemplazan por la versión que afirma sobre la variante: primero se escribe la nueva, en rojo, y después se borra la vieja (ERR-2.6).
- **Una dependencia, una entrada.** Desde S1, las claves de los tests de dependencias son `ruta/Cargo.toml#sección.paquete`, con `n` = número de capas virtuales que la ven. La rebanada que quita un paquete de un `Cargo.toml` borra sus entradas en el mismo PR, aunque el plan asignara el cierre a otra rebanada.
- **Movimientos sin cambios de contenido (desde R21a, regla del usuario).** Un archivo que se mueve o se renombra lo hace en un commit propio, con el contenido idéntico (`git show -M` da 100 % de similitud). Solo cambian ahí las declaraciones de módulo y las rutas de los imports que lo nombran. Reunir varios archivos en uno, o cambiar lo que dice, va en otro commit. Con el trinquete, la violación de un archivo de `baseline.txt` se corrige antes en el lugar, así que su entrada cae por el import o por el borrado del archivo, nunca por un cambio de ruta.
- **Firma que pasa a ser falible (desde S3, craft-backend 1.0.0-rc.3).** El commit de la firma falla fuerte con `.expect("pendiente: …")`, nunca descarta el `Fault`, y el `?` llega en el commit siguiente, con su test en rojo contra el commit padre. Reemplaza la secuencia de R10a, que descartaba el `Fault` de forma explícita. `rs_2_expect_names_the_invariant` admite `"pendiente:"`, y el revisor lo reporta si llega al final de la rebanada.
- **Baterías de contrato nuevas (desde S3).** Nacen con la forma final aunque el puerto siga síncrono: casos `pub async fn`, macro con `run` invocada con `run: block_on` y `#![expect(clippy::unused_async, reason = "puerto todavía síncrono: …")]` (receta "Puerto todavía síncrono" del README del stack).
- **La línea base solo crece en una sincronización con el harness.** Esa sincronización va en un PR propio, sin código de producción, y con el análisis de la línea base vieja frente a la nueva: entradas que se mantienen, las que cambian de clave, las que desaparecen y las nuevas, con ID del inventario y rebanada (ver §5.10). **Nunca en una rebanada.** Si una entrada desaparece sin un cambio de formato que lo explique, el PR se detiene: puede ser un cierre en falso.

### 5.2 Errores y estructura: recomendación

En [`archive.md`](harness-adoption/archive.md#52-errores-y-estructura-recomendación): la recomendación ya se aplicó en las fases 1 a 3.

### 5.3 Secuencia

Tamaños: XS (<50 líneas), S (<200), M (<500), L (>500 o muchos archivos).

Las fases 0 a 3, R20a, S4, R20b y R21a están hechas (el resultado de R21a está en [`slices/R21.md`](harness-adoption/slices/R21.md), que pasa a `archive/` con R21b): su detalle está en [`archive.md`](harness-adoption/archive.md#53-secuencia-rebanadas-hechas).

Una línea por rebanada pendiente, en el orden del plan. El detalle (alcance, IDs, línea base, rojos, riesgos, contrato, tamaño y notas) está en `harness-adoption/slices/<Rn>.md`. *Lista* quiere decir que sus dependencias (§5.4) ya están hechas.

| Rebanada | Estado | Depende de | Detalle |
|---|---|---|---|
| R21b. Agregado inmutable (R21a: hecha) | lista | R20b y R21a (hechas) | [`R21.md`](harness-adoption/slices/R21.md) |
| R22. Dominio de `communities` | lista | R20a (hecha) | [`R22.md`](harness-adoption/slices/R22.md) |
| R22b. `Field` cerrado en las hojas | pendiente | R20b (hecha), R22 | [`R22b.md`](harness-adoption/slices/R22b.md) |
| R23. iam async | pendiente | R21b; paralela a R24 | [`R23.md`](harness-adoption/slices/R23.md) |
| R24. communities async | pendiente | R22; paralela a R23 | [`R24.md`](harness-adoption/slices/R24.md) |
| R25. `shared` sin dependencias | lista | R21a (hecha) | [`R25.md`](harness-adoption/slices/R25.md) |
| R26. `edge/http` e `infrastructure/stderr` | lista | R5 (hecha); paralela a las fases 3 a 5 | [`R26.md`](harness-adoption/slices/R26.md) |
| **P3.** Punto de replanificación (§5.9) | pendiente | R22, R22b, R24, R25 y R26 | aquí, abajo |
| R27. Separar `communities` en crates por capa | pendiente | P3 (R22, R22b, R24, R25, R26) | [`R27.md`](harness-adoption/slices/R27.md) |
| R28. Separar `iam` y crear `IamApi::protect` | pendiente | R27, R20b (hecha), R21a (hecha), R21b, R22b, R23, R25, R26 | [`R28.md`](harness-adoption/slices/R28.md) |
| R29. `shared` pasa a ser `kernel/` | pendiente | R28 (y `shared/` sin entradas, desde R25) | [`R29.md`](harness-adoption/slices/R29.md) |
| R29b. `app/` pasa a `apps/server` (D9) | pendiente | R29 | [`R29b.md`](harness-adoption/slices/R29b.md) |
| R30. Problem Details (D5) | pendiente | R28 (y R26); paralela a R29 | [`R30.md`](harness-adoption/slices/R30.md) |
| R31. El username no distingue mayúsculas (D13 b) | pendiente | R30 | [`R31.md`](harness-adoption/slices/R31.md) |

> **Punto de replanificación P3 (después de R26, antes de R27).** Ver §5.9. *(S4: S4 ya se hizo, antes de R20b; P3 no la espera.)* R26 corre en paralelo desde R5, así que P3 se hace cuando están hechas R22, R22b, R24, R25 y R26, las precondiciones de la separación. Es la última revisión antes de las rebanadas L.

### 5.4 Dependencias y paralelismo

Las dependencias de cada rebanada pendiente están en el índice de §5.3. El diagrama completo, con las rebanadas hechas, y las notas de paralelismo están en [`archive.md`](harness-adoption/archive.md#54-dependencias-y-paralelismo-diagrama-completo). Siguen rigiendo:

- `[P1]`, `[P2]` y `[P3]` son los puntos de replanificación de §5.9. No se arranca la rebanada siguiente sin cerrar el punto.
- R21 se partió (R21a, #PR_R21A): R21b queda después, y R25 solo dependía de R21a.

### 5.5 Cambios de contrato HTTP (todos explícitos)

| Rebanada | Cambio | Tests de caracterización que cambian |
|---|---|---|
| R26 | Panic: 500 en lugar de conexión cortada | ninguno |
| R28 | Solo si cambia el fallback: ruta desconocida sin token, de 401 a 404 | `unknown_route_without_token_returns_401` |
| R25 | Ninguno: `ApiErrorResponse` se copia a cada contexto, idéntico | ninguno |
| R22b | Ninguno: `Field` cerrado cambia los tipos de las hojas, no las respuestas (propuesta en S4, confirmada en D15) | ninguno |
| R29b | Ninguno: `app/` pasa a `apps/server` (D9) | ninguno: los tests se mueven sin cambiar (adaptación mecánica, si hace falta, de los imports) |
| R30 | D5: Problem Details en todos los errores y en los 401 (`application/problem+json`, `title` genérico por status, el mensaje de hoy en `detail`, `code` como miembro de extensión con los mismos valores; los 401 del middleware, con cuerpo). Los status y los `code` no cambian. *(S4)* Con `unexpected(&f, r).with_code(codes::INTERNAL_ERROR)` de rc.4, el 500 queda sin el mensaje de D7 en `detail` (D14: decidido) | `assert_error` (`app/tests/common/mod.rs`) y los dos `raw_body == ""` (`communities_api.rs:47`, `auth_api.rs:643`) |
| R31 | D13 b: el username no distingue mayúsculas. El alta con un username que solo difiere en mayúsculas pasa de 201 a 409 `IAM_ACCOUNT_USERNAME_ALREADY_EXISTS`; authenticate e identify con el username en otras mayúsculas encuentran la cuenta (de 401 `IAM_LOGIN_FAILED` y de 404, a la respuesta de la cuenta); identify responde el username registrado | **Nuevos:** `register_rejects_same_username_with_different_casing_with_409`, `authenticate_finds_the_account_with_a_username_in_another_casing` e `identify_with_a_username_in_another_casing_returns_the_registered_username`. Ningún test de `app/tests/` ni de `account_lifecycle.rs` fija hoy que el username distingue mayúsculas (buscado en P2), así que no cambia ninguno |

### 5.6 Cierre de cada test de arquitectura

Desde S1 la columna cuenta entradas con el formato nuevo (una por dependencia en los `Cargo.toml`). Entre paréntesis va el total de violaciones que admiten, sumando las `n`.

Aquí, solo los tests con entradas abiertas. Los tests en cero, la conciliación de P2 (las entradas, por número), las de R1, P1, S2, S3 y S4 y las sumas por rebanada anteriores están en [`archive.md`](harness-adoption/archive.md#56-cierre-de-cada-test-de-arquitectura-historia).

| Test | Entradas hoy (después de R21a) | Justo antes de S1 | Queda en cero en |
|---|---|---|---|
| `hex_8_kernel_depends_only_on_std` | 1 | 1 | R25 (R21a −1 `chrono`, a93bb86; R25 −1 `serde`) |
| `hex_2_core_depends_only_on_std_and_the_kernel` | 10 (19) | 3 | R28 *(P2)*: R20a −1 (`uuid` de iam; d461bbc), R21a −2 (`chrono`; a93bb86), R22 −2 (`uuid` y `http` de communities), R25 −1 (`serde` de `shared`), R27 −2 (`axum`, `serde`), R28 −5 (`argon2`, `axum`, `jsonwebtoken`, `rand`, `serde`). Antes de P2: R21 −2, R25 −1, R27 −4, R28 −6 |
| `hex_2_server_frameworks_only_at_the_edge` | 2 (6) | 2 | R28 (R27, R28) |
| `hex_6_transport_types_carry_the_transport_name` | 9 | 9 | R28 (R27, R28) |
| `hex_2_core_imports_only_std_and_the_kernel` | 3 | — (nuevo en S4) | R20b: −1 (`code_validation.rs`; 8976a8e), R21a (−2: `account_event.rs`, 78a7776, y `shared/src/domain/domain_event.rs`, 464dc52), R22 (−2, `community_id.rs` y `owner_id.rs`) y R25 (−1, `api_error_response.rs`). Con el orden del plan, R25 |

**Precondiciones de R27, R28 y R29 (P2).** Están en el archivo de cada rebanada (`slices/R27.md`, `slices/R28.md` y `slices/R29.md`, «Precondiciones (P2)»).

**Suma por rebanada desde S4 (35, después de R20a):** R20b −1, R21 −9, R22 −4, R25 −3, R27 −5 y R28 −13. La fila #11 ya se borró en R20a; las #31 a #36 son de S4. *(R20b)* R20b borró la #33 (8976a8e): quedan 34 entradas (48 violaciones). *(R21a)* R21a borró 9 (78a7776, 464dc52, a93bb86, d3d984b y 8b63dd4): quedan 25 entradas (38 violaciones), y `hex_8_model_markers_have_no_items` queda en cero. Las precondiciones de R27 y R28 no cambian: R22 deja `communities-domain` sin `uuid` también por archivo, y R20b y R21 dejan `iam-domain` sin `rand` ni `chrono`.

Los hallazgos que no son de código no tienen rebanada:
- ERR-4.1: rotar el secreto, una acción del usuario.
- TDD-1.5, TDD-1.6 y TDD-1.7: descartados por D8.

### 5.7 Decisiones del usuario

Todas decididas; el texto completo, con su planteo, está en [`archive.md`](harness-adoption/archive.md#57-decisiones-del-usuario-texto-completo). Una línea por decisión, con la rebanada que la aplica:

- **D1** (R13, P1): verificar dos veces sigue en 400 `IAM_ACCOUNT_INVALID_VERIFICATION`; el 409 se revisa con R30.
- **D2** (R16, P1), opción a): `OwnerId` valida el formato UUID y rechaza el nil, con los códigos `IAM_INVALID_ACCOUNT_ID(_FORMAT)`.
- **D3** (R18, R1): se elimina `ListPublicCommunities`.
- **D4** (R21, R22, P1): se retira el andamiaje de eventos sin uso (`AccountEvent`, `record_event`/`pull_events`, `MembershipPolicy`, `MembershipPolicyChanged`); sin `take_events` ni `Clock`.
- **D5** (R30, P2): Problem Details (`application/problem+json`), con `code` como miembro de extensión y los valores de hoy, `title` genérico por status, el mensaje de hoy en `detail` y cuerpo en los 401 del middleware. D14 la ajusta en el 500.
- **D6** (R2, R1): el sign-in de una cuenta inexistente verifica contra un hash señuelo generado al arrancar.
- **D7** (R5 a R30, R1): cuerpo interino del 500, `{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}`.
- **D8** (P1): TDD-1.5 a TDD-1.7, `descartado (histórico; TDD-1 rige desde R1)`.
- **D9** (R29b, P2): `app/` pasa a `apps/server`.
- **D10** (R12b): verify, authenticate e identify buscan con el identificador normalizado; uno que no se puede construir responde como una cuenta inexistente.
- **D11** (R13b, S3), opción a): `VerifyAccountResponse` vacío; la respuesta sigue siendo 200 con cuerpo `null`.
- **D12** (R16b, S3), opción b): las baterías toman la forma async en R16b, antes de R23 y R24.
- **D13** (R31, P2), opción b): la unicidad del username no distingue mayúsculas, y se conserva la forma registrada.
- **D14** (R30, 2026-11-27): el 500 usa `unexpected(&fault, reporter).with_code(codes::INTERNAL_ERROR)` y va sin `detail`.
- **D15** (R22b, 2026-11-27): R22b confirmada, después de R20b y R22 y antes de R27.

### 5.8 Registro de rebanadas

En [`harness-adoption/costs.md`](harness-adoption/costs.md).

### 5.9 Puntos de replanificación

Hay tres: **P1** después de R10 *(hecho, 2026-11-23: ver abajo)*, **P2** después de R19 *(hecho, 2026-11-26: ver abajo)* y **P3** después de R26 (antes de R27; *S4: ya no espera a S4, que se hizo antes de R20b*). En cada uno, antes de abrir la rebanada siguiente:

1. Auditoría completa con `craft-backend:architecture-reviewer` y actualización de la cabecera del documento (fecha, commit, conteos).
2. `cargo test -p architecture`, y comparar las entradas que quedan en `baseline.txt` con la columna "Entradas hoy" de §5.6 menos lo ya cerrado. Si no coinciden, explicar la diferencia antes de seguir.
3. Revisar las decisiones abiertas de §5.7 que bloquean las rebanadas siguientes: D1 (R13) y D2 (R16, si todavía no se hizo) en P1; D4 (R21, R22) en P2, y D5 (R30) en P3. *(P2: D5, D9 y D13 ya se decidieron en P2.)*
4. Comparar tamaño y tokens reales de §5.8 con lo estimado, y partir las rebanadas que vienen si alguna quedó más grande de lo previsto.
5. Reescribir en este documento lo que cambie del plan (secuencia, IDs, línea base) en un PR de solo documentación, antes de seguir.

P1 y P2 están en [`archive.md`](harness-adoption/archive.md#59-puntos-de-replanificación-p1-y-p2).

### 5.10 Sincronizaciones con el harness

En [`archive.md`](harness-adoption/archive.md#510-sincronizaciones-con-el-harness): la tabla y el detalle de S1 a S4.

## 6. Fricciones con el harness

Lo que el harness no cubrió o cubrió mal durante la adopción. Cada fricción se reporta al harness. Al resolverse, se anota la versión que la cierra.

| Fecha | Rebanada | Fricción | Efecto en la adopción | Estado en el harness |
|---|---|---|---|---|
| 2026-11-26 | R20a | La cabecera de `material-exempt.txt` y la skill de adopción dicen que el agente nunca escribe el archivo y que el hook `pre-edit` lo bloquea, pero el hook dejó borrar una exención. No queda claro si borrar es del agente (lo pide el test con "borrar de material-exempt.txt") o también solo del usuario. *(2026-11-27)* La asimetría es el punto: agregar una exención requiere una persona, y quitarla no. | El plan preveía detenerse; el usuario pidió que el agente la borrara, y el `Edit` pasó. | sin reportar |
| 2026-11-26 | S4 | Comando del plugin: `/craft-backend:adoption` pide leer entero `docs/harness-adoption.md`, y el documento (325 KB) pasa el límite de lectura de la herramienta. | Hubo que leerlo por partes, con riesgo de saltear una sección. Este PR lo parte en un documento vivo, `archive.md` y `costs.md`. *(R20b)* La partición (#57) no alcanzó: el documento vivo (104 KB, unos 50k tokens) sigue pasando el tope de una lectura (25k tokens). La primera lectura se cortó en la línea 186 de 622, y el resto entró con una segunda lectura forzada (`allow_large`). Se leyó entero, pero en dos partes. *(2026-11-27)* Un PR de solo documentación pasó el detalle de cada rebanada a `slices/<Rn>.md` (§5.3 queda como índice) y más historia a `archive.md`. Ahora cada sesión lee el vivo y el archivo de su rebanada. **La skill de adopción debería traer esta estructura de serie**, en lugar de que cada proyecto la descubra cuando el documento ya no entra. | sin reportar |
| 2026-11-26 | S4 | Comando del plugin: dice que el PR se abre solo si el usuario lo pidió, y el ritual del proyecto (§5.1) pide un PR contra `main` en cada rebanada, aunque la invocación no lo pida. | Rige §5.1, pero la contradicción se resuelve a mano en cada invocación. La fricción de 2026-11-23 (el PR solo si se pide) figura corregida en rc.2, y en S4 la instrucción sigue. | sin reportar |
| 2026-11-26 | S4 | Comando del plugin: el paso de sincronización no corre `ARCH_REQUIRE_MATERIAL=1`: queda fuera de los pasos de `sincronizar`, aunque una plantilla nueva puede dejar vacío un conjunto o hacer innecesaria una exención. | S4 lo corrió a mano. Sin eso, una exención de más o un conjunto vacío nuevo se habrían visto recién en CI. | sin reportar |
| 2026-11-26 | S4 | Comando del plugin: la comparación de la plantilla con lo copiado solo cubre `tools/architecture`. Lo que el proyecto copió de otras partes de la plantilla (`infrastructure/uuid`, `kernel/src/model.rs`, `edge/http`) no se compara. | S4 lo comparó a mano (§5.3 del archivo, S4): `infrastructure/uuid` sin cambios, `model.rs` admite `Entity` e `Identifier`, `edge/http` trae `UnexpectedProblem`. Sin eso, R21, R26 y R30 habrían seguido con la plantilla de rc.3. | sin reportar |
| 2026-11-27 | R20b | VER-1: ningún test de `tools/architecture` exige que cada puerto de `ports/outbound` tenga su batería en `testing/contracts/`, que cada doble la invoque, ni que el doble de un puerto falible tenga `failing()` (TDD-2). Lo propuso el revisor de R20b. | Hoy se cumple, revisado a mano en cada rebanada. | sin reportar |

### Propuestas para reportar

La descripción de cada fricción está en la tabla de arriba; aquí, lo que se le propone al harness. La numeración sigue la de P2: las nueve primeras y la de `unexpected()` sin `code` quedaron corregidas en craft-backend 1.0.0-rc.4 y están en `harness-adoption/archive.md`, con el texto anterior de esta lista.

10. **Borrar una exención de material (R20a).** Que la skill diga si el borrado que pide el test es del agente o del usuario, y que el hook lo aplique igual: si es del agente, que la skill lo diga; si no, que el hook lo bloquee como bloquea el agregado.
11. **El documento de seguimiento pasa el límite de lectura (S4, R20b).** Que la skill traiga de serie la estructura que este proyecto adoptó: un documento vivo con §5.3 como índice, un archivo por rebanada en `slices/<Rn>.md`, que la sesión lee junto al vivo y mueve a `archive/` al cerrar, y un archivo de lo cerrado. Como mínimo, que el comando diga qué secciones leer siempre (cabecera, §5.1, la rebanada, §5.6, §5.7) y cuáles solo si la rebanada las toca.
12. **"PR solo si el usuario lo pidió" contra el ritual del proyecto (S4).** Que la instrucción ceda ante las convenciones del proyecto, si las hay.
13. **`ARCH_REQUIRE_MATERIAL` fuera del paso de sincronización (S4).** Sumarlo a los pasos de `sincronizar`, después del llenado de la línea base.
14. **La comparación fuera de `tools/architecture` (S4).** Comparar también los crates de la plantilla que el proyecto copió (`kernel`, `edge/http`, `infrastructure/*`) y listar sus cambios entre versiones.
15. **Baterías y dobles sin test (R20b).** Un test de `tools/architecture` que exija que cada puerto de `ports/outbound` tenga su batería en `testing/contracts/`, que cada doble la invoque y que el doble de un puerto falible tenga `failing()`.
