# Adopción del harness craft-backend

> **Dónde está cada cosa.** Aquí, lo vivo: cabecera, filas abiertas del inventario (§2), dudas abiertas (§4), reglas (§5.1), rebanadas pendientes (§5.3) con sus dependencias (§5.4) y sus cambios de contrato (§5.5), cierre de los tests de arquitectura (§5.6), decisiones (§5.7), el proceso de los puntos de replanificación (§5.9) y las fricciones sin reportar (§6). En [`harness-adoption/archive.md`](harness-adoption/archive.md), lo cerrado: el resumen de la auditoría (§1), las filas resueltas o descartadas, §3, las dudas resueltas, §5.2, las rebanadas hechas, las filas hechas de §5.5, P1 y P2, las sincronizaciones S1 a S4 (§5.10), las fricciones corregidas y el método. En [`harness-adoption/costs.md`](harness-adoption/costs.md), el registro de rebanadas con sus costos (§5.8).

Este archivo guarda el estado entre sesiones de la adopción de los principios craft-backend (HEX, ERR, TDD, VER, RS) en este repositorio.

| Dato | Valor |
|---|---|
| Fecha de la auditoría | 2026-11-19; auditorías completas de P1: 2026-11-23 y de P2: 2026-11-26 |
| Plugin | `craft-backend@ai-harness` 0.12.0 en la auditoría; `tools/architecture` de `v1.0.0-rc.4` desde S4 |
| Commit auditado | `d0b0eeb` (rama `ci/sync-development`); P1: `bab68c5` (`main`, después de R10b); P2: `0eaab25` (`main`, después de R19) |
| Revisor | `craft-backend:architecture-reviewer`, auditoría completa en solo lectura |
| Hallazgos | 131: 26 de severidad alta, 55 media, 50 baja. 94 de la auditoría inicial (RS-1.4 agregado en R2), 16 de la de P1 (ERR-4.9 a ERR-4.12, HEX-7.10 a HEX-7.12, HEX-8.10, ERR-1.8, TDD-1.8, TDD-2.6 y VER-1.3 a VER-1.7), 3 de la revisión de R11 (ERR-3.4, ERR-4.13, TDD-2.7), 1 de R12 (ERR-4.14), 1 de la revisión de R13 (ERR-2.11), 3 de S3, la sincronización con `v1.0.0-rc.3` (HEX-4.5, HEX-8.11, HEX-8.12), 2 de la revisión de R16, inventariados en R16b (RS-2.6, TDD-2.8), y 11 de la auditoría de P2 (HEX-2.7, HEX-7.13, HEX-7.14, VER-1.8, VER-1.9, ERR-2.12, TDD-2.9 a TDD-2.11, RS-1.5 y RS-1.6) |

## Cómo mantener este archivo

- Al resolver un hallazgo, cambiar su **Estado** de `abierto` a `resuelto (<commit>)`. Si se descarta, poner `descartado` y el motivo. En el mismo PR, la fila pasa a `harness-adoption/archive.md`, a la tabla de su regla; lo mismo con una rebanada hecha (su detalle) y una fricción corregida. Una fila nueva de §5.8 va en `harness-adoption/costs.md`.
- Los IDs de hallazgo (`HEX-1.3`, `ERR-4.2`…) son estables: no se renumeran. Un hallazgo nuevo recibe el siguiente número libre de su regla.
- Después de cada slice de refactor, volver a auditar con architecture-reviewer y actualizar la cabecera (fecha, commit, conteos).
- Tests de caracterización en `app/tests/` (PR #18): son la red de seguridad del refactor y tienen que seguir pasando.
- Cada rebanada se hace con `/craft-backend:adoption R<n> [notas]`. Las reglas propias de este proyecto, que la skill no trae, están en §5.1 ("Ritual de rebanada").

## Prioridades inmediatas

1. **ERR-4.1:** `.env` ya no se versiona (0df9baa), pero `JWT_SECRET` sigue en el historial: **rotar el secreto** en cada entorno donde se haya usado.
2. ~~**ERR-1.1 / RS-2.1:** retirar `SystemError`.~~ Hecho en R19 (11f8c92): los casos de uso devuelven `Failure<{Verb}{Noun}Error>` y el árbol B es `Fault`.
3. **HEX-2 / HEX-7 (fase 4):** sacar `uuid`, `rand` y `chrono` del núcleo (R20a, R20b, R21, R22), la condición para separar crates (§5.6, "Precondiciones de R27 y R28").
4. **RS-1 / HEX-1 / HEX-2:** separar cada contexto en crates por capa, empezando por `communities` (R27). HEX-1.1 ya está resuelto (f6bc1fd).

Ya resueltas: ERR-4.2 (`dbg!`), ERR-4.3 (`Debug` con el secreto), VER-1.1/RS-4.1 (`tools/architecture` con trinquete sobre `baseline.txt`), VER-1.2 (CI con fmt, clippy y test), RS-2.5 (`[workspace.lints]`) ERR-1.2/ERR-1.3 (los errores ya no implementan `std::error::Error`) HEX-8.4/HEX-8.7 (código muerto del kernel) y RS-1.3/RS-1.4 (herencia del workspace, sin `anyhow`). En P1, ERR-4.9 (un `JWT_SECRET` vacío o corto ya no arranca el servidor; PR #40).

## 1. Resumen

En [`archive.md`](harness-adoption/archive.md#1-resumen): es la foto de la auditoría inicial. Los conteos de hoy están en la cabecera.

## 2. Inventario

### HEX (hexagonal-architecture) y VER

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| HEX-1.6 | HEX-1 Autenticación | app/src/middleware/auth.rs:17 | El middleware vive en el composition root, no en el `api` de `iam`. Inyecta `VerifiedIdentity` (de `shared::domain`, línea 36) en vez de un tipo del borde. Desde R10b consume `VerifyAccessTokenUseCase`, pero importa `iam::infrastructure::http::errors::error_mapper` (línea 12), un detalle del `api` de iam. | alta | `IamApi::protect(routes)` en `iam-api`, que inserta `edge_http::AuthenticatedAccount`. | abierto: desde R11 (7655856) el middleware importa `iam::infrastructure::http::unexpected` en lugar del mapper; sigue siendo un detalle del HTTP de iam (R28) |
| HEX-1.9 | HEX-1 Dependencias | contexts/iam/src/infrastructure/mod.rs:1; contexts/communities/src/infrastructure/mod.rs:1 | El adaptador driving (`http`) es un submódulo de `infrastructure`: `api` e `infrastructure` no están separados y pueden importarse entre sí. | alta | Crate `{ctx}-api` separado (RS-1). | abierto: R27 (communities) y R28 (iam) |
| HEX-2.1 | HEX-2 Núcleo sin framework | contexts/iam/Cargo.toml:11-17 | `domain` y `application` comparten crate con `uuid`, `argon2`, `rand`, `jsonwebtoken`, `serde`, `chrono` y `axum`, así que todos están disponibles en el núcleo. | alta | Crates `iam-domain` e `iam-application` con solo el kernel. | abierto: R28. *(P2)* Antes salen `uuid` (R20a, hecho en d461bbc) y `chrono` (R21); quedan `argon2`, `axum`, `jsonwebtoken`, `rand` y `serde`, que la separación deja fuera de domain y application |
| HEX-2.3 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/code_validation.rs:2 | El dominio usa `rand` para generar el código (líneas 29-33). | media | Generar el código con un puerto o capacidad y pasárselo a `Account::create`. | abierto: R20b. *(S4)* Entrada `hex_2_core_imports_only_std_and_the_kernel` del archivo: R20b −1 |
| HEX-2.4 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/events/account_event.rs:1 | El dominio usa `chrono`. | media | El instante llega de `Clock`. | abierto: R21 (D4: se borra `AccountEvent`). *(S4)* Entrada `hex_2_core_imports_…` del archivo: R21 −1 |
| HEX-2.5 | HEX-2 Núcleo sin framework | contexts/communities/Cargo.toml:11-14 | `uuid`, `axum`, `serde` y `http` (este último, visto en S1) están disponibles en `domain` y `application`. | alta | Separar crates por capa. | abierto: R27. *(P2)* Antes salen `uuid` (R22, con HEX-2.7) y `http` (R22, RS-1.5); quedan `axum` y `serde`, que la separación deja en `communities-api` |
| HEX-2.6 | HEX-2 Núcleo sin framework | contexts/communities/src/domain/model/community/community_id.rs:2 | El dominio usa `uuid`. | media | `IdGenerator` + value object sobre `String`. | abierto: R22. *(P2)* `owner_id.rs` también lo usa: HEX-2.7. *(S4)* Entrada `hex_2_core_imports_…` del archivo: R22 −1 |
| HEX-2.7 | HEX-2 Núcleo sin framework | contexts/communities/src/domain/model/community/owner_id.rs:2 | `OwnerId` valida con `uuid` (`Uuid::parse_str`, `is_nil`). Entró en R16 (3646f25) sin que fallara nada: en legacy, la entrada `hex_2_core … communities#dependencies.uuid` es por manifiesto y ya existía (VER-1.8). HEX-2.6 solo nombraba `community_id.rs`, así que `uuid` habría seguido en el dominio después de R22 y R27 habría creado `communities-domain` con una violación nueva. Hallado en P2. | media | `OwnerId(String)`, validado a mano con std (forma con guiones, hexadecimal, sin nil), con `InvalidOwnerId { Nil, WrongFormat }` (D2 a) e `impl FromStr` (RS-2.6). | abierto: R22. *(S4)* Entrada `hex_2_core_imports_…` del archivo: R22 −1 |
| HEX-6.1 | HEX-6 Transporte | iam/…/requests/sign_up.rs:5, sign_in.rs:5, verify.rs:5, identify.rs:5; responses/signed_up.rs:5, signed_in.rs:5, identified.rs:5; communities/…/requests/create.rs:5, responses/created.rs:5 | Los DTOs HTTP no son `{Caso}HttpRequest`/`{Caso}HttpResponse`, y `CreateRequest` colisiona con el vocabulario de puerto saliente. | baja | `RegisterAccountHttpRequest`, `CreateCommunityHttpResponse` y demás. | abierto: R27 (communities) y R28 (iam) |
| HEX-7.3 | HEX-7 Inmutabilidad | contexts/iam/src/domain/model/account/account.rs:94-140 *(P2; antes 89-134)* | El agregado es mutable (`&mut self`): `change_password`, `change_email`, `deactivate`, `confirm_registration`… | media | `fn confirm(&self, …) -> Result<Self, …>`, que devuelve el estado nuevo. | abierto: R21. *(P2)* Con HEX-7.13, solo `confirm_registration` tiene llamador de producción: los demás se borran en lugar de volverse inmutables |
| HEX-7.4 | HEX-7 Eventos | contexts/iam/src/domain/model/account/account.rs:32-46 | `record_event`/`pull_events` son `todo!()`; ninguna transición registra eventos (`AccountEvent::Registered` nunca se emite). | media | `create` registra `Registered`, más `take_events`; o quitar el andamiaje. | abierto: R21 (D4: se retira) |
| HEX-7.5 | HEX-7 Crear/reconstituir | contexts/iam/src/domain/model/account/account_factory.rs:53 *(P2; antes :14)* | `AccountFactory::create`/`reconstitute` (línea 32) reciben `String`. Además, `Account::new` es público (account.rs:49) y permite saltarse las reglas de alta. *(P2)* `reconstitute` se borró (0a3c49a) y `create` recibe un `AccountDraft` con value objects (77e0ca4, R12), más `hashed_password: String` (l.55). Siguen `String` en `draft` (l.41), el hash como `String` y `Account::new` público. | media | `Account::create`/`Account::restore` con value objects; `new` privado. | abierto: R20b (parcial de paso en R12 y R19) |
| HEX-7.6 | HEX-7 Identificador | contexts/iam/src/domain/model/account/account_factory.rs:63 (y `CodeValidation::generate` en la l.59); contexts/communities/src/application/commands/create_community/handler.rs:72 *(P2; antes account_factory.rs:21 y create_community.rs:48)* | El id se genera en el dominio (`AccountId::generate`, `CommunityId::generate`) en vez de recibirse de `IdGenerator`. | media | Inyectar `Arc<dyn IdGenerator>` en el handler y pasar el id. | abierto: R22 (communities). iam resuelto en R20a (d461bbc): `RegisterAccountHandler` recibe el `IdGenerator` y `AccountFactory::create`, el id; el composition root cablea `UuidIdGeneratorAdapter` (5b49e36). `CodeValidation::generate` sigue en R20b |
| HEX-7.7 | HEX-7 Mapeo de persistencia | contexts/iam/src/domain/model/account/account_status.rs:76 | `from_str` define en el dominio el formato persistido `"registered:123123"`. | baja | El adaptador traduce de `String` a `AccountStatus`. | abierto: R20b |
| HEX-7.8 | HEX-7 Servicio de dominio | contexts/communities/src/domain/services/membership_policy.rs:2 | `MembershipPolicy` es un enum de opciones (value object) archivado en `services/`. | baja | Moverlo a `model/community/`. | abierto: R22 (D4: se retira) |
| HEX-7.9 | HEX-7 Comportamiento | contexts/communities/src/domain/model/community/community.rs:19-56 | Agregado anémico: `membership_policy` siempre es `None` y `MembershipPolicyChanged` (membership_policy_changed.rs:4) no se usa. | baja | Agregar el comportamiento con su evento, o retirar lo muerto. | abierto: R22 (D4: se retira) |
| HEX-7.13 | HEX-7 Comportamiento | contexts/iam/src/domain/model/account/account.rs:94-124 | `activate`, `deactivate`, `suspend`, `change_password`, `change_email` y `change_username` no tienen llamador de producción; solo los usan sus tests. Además, `activate` decide con `self.status.as_str() == "registered"` (l.116), comparando la representación de texto, que es también el formato de identify. Hallado en P2. | baja | Criterio de D4 (sin comportamiento ni uso): borrarlos con sus tests en R21. Si alguno se conserva, que decida con `matches!(self.status, AccountStatus::Registered { .. })`. | abierto: R21 |
| HEX-7.14 | HEX-7 Identificador | contexts/iam/src/domain/model/account/account_id.rs:26-39; contexts/communities/src/domain/model/community/community_id.rs:12-25 | `AccountId::from_uuid`/`from_str` y `AccountIdError` (errors/account_id.rs:4) no tienen llamador de producción desde f6bc1fd (R16), y `CommunityId::from_uuid`/`from_str` e `InvalidCommunityId` (errors/invalid_community_id.rs:4) nunca lo tuvieron. Sostienen el `uuid` de HEX-2.2 y HEX-2.6 y los `#[allow(clippy::should_implement_trait)]` de account_id.rs:35 y community_id.rs:21. Hallado en P2. | baja | Borrarlos con sus tests, en lugar de convertirlos en `FromStr`: los ids nacen del `IdGenerator`. | abierto: R22 (communities). iam resuelto en R20a (bae89e9): `from_uuid`, `from_str` con su `allow`, `as_uuid` y `AccountIdError` borrados con sus 4 tests |
| HEX-8.1 | HEX-8 Kernel sin dependencias | shared/Cargo.toml:7-8 | El kernel depende de `chrono` y `serde`. | alta | Kernel solo `std`. | abierto: desde S1, `chrono` sale en R21 y `serde` en R25 |
| HEX-8.2 | HEX-8 Marcadores | shared/src/domain/domain_event.rs:5 | El marcador `DomainEvent` exige `DateTime<Utc>` de `chrono`. | media | Sin dependencias; el instante llega de `Clock`. | abierto: R21 (D4). Desde S3, que el marcador declare métodos es HEX-8.11. *(S4)* Entrada `hex_2_core_imports_…` del archivo (`chrono`): R21 −1 |
| HEX-8.6 | HEX-8 Sin modelos | shared/src/domain/model/verified_identity.rs:2; shared/src/application/auth_context.rs:2 | Tipos de identidad en el kernel. | media | `AuthenticatedAccount` en `edge/http`. | abierto: R18 (a603969) borró `AuthContext` (sin usos tras quitar `ListPublicCommunities`); queda `VerifiedIdentity` (R27) |
| HEX-8.8 | HEX-8 Kernel | shared/src/infrastructure/http/api_error_response.rs:4 | Un tipo de transporte HTTP (con `serde`) en el kernel. | media | `edge/http` con `Problem`. | abierto: R25 *(P2; antes R30)*: sale del kernel como una copia por contexto, y R30 borra las copias. *(S4)* Entrada `hex_2_core_imports_…` del archivo (`serde`): R25 −1 |
| HEX-8.9 | HEX-8 Marcadores | shared/src/domain/aggregate_root.rs:6-8 | `AggregateRoot` no es un marcador: impone `record_event(&mut self)`, `pull_events` y `version`, una API mutable. Además trae el helper mutable `EventRecorder` (líneas 11-32). | baja | Marcador vacío; los eventos se manejan con `take_events(self)` en el agregado. | abierto: R21. Desde S3 tiene entrada en `hex_8_model_markers_have_no_items`. El `AggregateRoot` de la plantilla ya es `pub trait AggregateRoot {}` |
| HEX-8.10 | HEX-8 Marcadores | shared/src/domain/value_object.rs:1 | `ValueObject` no es un marcador: impone `new(Value)` y `value()`. Obliga a los `todo!()` de `AccountId` (account_id.rs:13,17) y `AccountStatus` (account_status.rs:18,22), 4 de los 10 de RS-2.2. Hallado en P1. | media | Marcador vacío, o borrarlo, con `new`/`as_str` propios en cada value object. | abierto: R21 (R20 ya quita los `impl` de `AccountId` y `AccountStatus`). Desde S3 tiene entrada en `hex_8_model_markers_have_no_items`. Además de esos dos, el trait lo implementan `Email`, `Username`, `HashedPassword` y `CodeValidation`, y unos 12 archivos llaman a `new`/`value` a través de él, incluido `account_lifecycle.rs` (adaptación mecánica: el import). *(P2)* Son 11 archivos que no implementan el trait (incluido `account_lifecycle.rs:43`), más los 6 `impl`. La plantilla lo deja en `pub trait ValueObject: Clone + Eq {}` |
| HEX-8.11 | HEX-8 Marcadores | shared/src/domain/domain_event.rs:3 | `DomainEvent` no es un marcador: declara `event_type`, `occurred_at`, `aggregate_id` y `event_version`. Solo lo implementa `AccountEvent`, que D4 retira. Hallado en S3. | baja | Borrarlo con `AccountEvent` (D4), o dejar `pub trait DomainEvent: Clone {}` como en la plantilla. | abierto: R21 |
| HEX-8.12 | HEX-8 Marcadores | shared/src/domain/entity.rs:1 | `Entity` no es un marcador: declara `type Id`, `fn id` y `fn same_identity_as`. Solo lo implementa `Account` (account.rs:24). La plantilla no trae `Entity` (su `model.rs` tiene `ValueObject`, `AggregateRoot` y `DomainEvent`). Hallado en S3. | baja | *(S4)* Dejarlo como marcador vacío (`pub trait Entity {}`), sin borrarlo: desde 1.0.0-rc.3.7, `Entity` es un marcador admitido y un proyecto que lo usa lo conserva sin ítems. Antes de S4: borrarlo. | abierto: R21 |
| VER-1.7 | VER-1 Lints | Cargo.toml:40 (`[workspace.lints.clippy]`; `unwrap_used` en la l.43) *(P2; antes :41)* | `[workspace.lints.clippy]` solo prohíbe `unwrap_used`. Nada impide un `todo!()` (RS-2.2) o un `dbg!` (ERR-4.2) nuevo. Hallado en P1. | baja | `todo = "deny"` y `dbg_macro = "deny"`. | abierto: R21 (cuando se borra el último `todo!()` de RS-2.2) |

### ERR (error-handling)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| ERR-1.8 | ERR-1 Canal declarado | app/src/config/app.rs:11 | `AppConfig::load()` declara `Result<_, ConfigError>` pero termina el proceso dentro con `process::exit(1)`: la rama `Err` de main.rs:5 nunca corre. Hallado en P1. | baja | `let jwt = JwtConfig::from_env()?;`. | abierto: R26 |
| ERR-2.5 | ERR-2 Variante correcta | contexts/iam/src/domain/model/account/account_status.rs:82, 90 | Un estado ilegible se reporta como `InvalidTransition`. | baja | Dato corrupto → `Fault` en el adaptador. | abierto: R20b (se borra `from_str`) |
| ERR-2.12 | ERR-2 Error por regla | contexts/iam/src/domain/errors/account.rs:4 | `AccountError`, un error con nombre de agregado, tiene una sola variante (`InvalidVerification`) y un solo productor (`confirm_registration`). Es el patrón de error global de dominio que ERR-2.11 propuso reemplazar: una variante nueva obliga a cada `match` del handler a traducir variantes que esa operación no produce. Hallado en P2. | baja | Un error propio de la transición (`InvalidVerification`, como `CannotAuthenticate`), que devuelve el nuevo `confirm(&self, …) -> Result<Self, InvalidVerification>`. | abierto: R21 |
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
| TDD-2.7 | TDD-2 Doble completo | contexts/iam/src/application/commands/register_account/handler.rs (`StubPasswordHashingAdapter`) | El brazo `AccountCreationError::HashedPassword` (→ `Fault`; desde R12 (77e0ca4), el `HashedPasswordError` de `AccountFactory::create`) solo se alcanza con `StubPasswordHashingAdapter`, que incumple a propósito `hash_is_a_valid_hashed_password` y no corre la batería. La causa es la firma `hash -> Result<String, Fault>`, que admite un estado que el contrato prohíbe. Declarado con su motivo. Hallado en la revisión de R11. | baja | `PasswordHashingService::hash -> Result<HashedPassword, Fault>` y `AccountFactory::create` recibe un `HashedPassword`: desaparecen el error, el brazo y el stub. | abierto: R20b (asignado en P2: `AccountFactory::create` cambia ahí, y `Account::create` recibe el `HashedPassword`) |
| TDD-2.9 | TDD-2 Batería completa | contexts/communities/src/application/testing/contracts/community_repository.rs:44 | `snapshot` compara `id`, `slug`, `name` e `is_public`, pero no el dueño: `Community` no expone `owner_id()` (el campo es `_owner_id`, community.rs:12). Un adaptador que pierda o confunda el dueño pasa la batería, y ningún test de handler puede afirmar que la comunidad se guardó con el `owner_id` del comando. Hallado en P2. | media | `Community::owner_id() -> &OwnerId`, el dueño en `snapshot`, y afirmarlo en `create_community_successfully`. | abierto: R22 |
| TDD-2.10 | TDD-2 Doble completo | contexts/iam/src/application/testing.rs:214 | `SpyPasswordHashingAdapter` no tiene `failing()` ni `failing_on`, aunque su puerto puede fallar. Hallado en P2. | baja | Delegar en el `FixedPasswordHashingAdapter` interno: `failing()`/`failing_on(op)`. | abierto: R23 |
| TDD-2.11 | TDD-2 Nombre del test | contexts/iam/src/domain/model/account/account.rs:265 | `reconstituted_user_preserves_status` ya no prueba ninguna restauración: arma la cuenta con el helper (`AccountStatus::from_str` + `Account::new`) y afirma el getter. El nombre promete `reconstitute`, que se borró en 0a3c49a. El revisor de R19 lo dejó en el texto de §5.3, sin ID. Hallado en P2. | baja | Reescribirlo contra `Account::restore`: `restored_account_preserves_status`. | abierto: R20b |

### RS (rust-stack)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| RS-1.1 | RS-1 Crate por capa | Cargo.toml:2-5 | Un crate por contexto (`contexts/*`), con las capas como módulos; no hay `kernel/` ni `edge/http`. | alta | `contexts/{ctx}/{domain,application,infrastructure,api}` (migración incremental). | abierto: R27 y R28 |
| RS-1.2 | RS-1 Kernel | shared/Cargo.toml:2 | El kernel se llama `shared` y tiene capas propias (`application`, `infrastructure/http`). | media | `kernel/` (solo std) + `edge/http`. | abierto: R29 |
| RS-1.5 | RS-1 Dependencias | contexts/communities/Cargo.toml:12 | `http` está declarado y no se usa: desde 58958bc (R17), los `StatusCode` de communities vienen de `axum::http`. Sostiene la entrada `hex_2_core … communities#dependencies.http 2` sin ningún uso. Hallado en P2. | baja | Quitarlo del manifiesto y borrar su entrada en el mismo PR (§5.1, "Una dependencia, una entrada"). | abierto: R22 |
| RS-1.6 | RS-1 Dependencias | app/Cargo.toml:10, 12 | `serde` y `tower` están en `[dependencies]` de `app`, y ningún archivo de producción los usa: `tower::ServiceExt` solo aparece en el `mod tests` de `middleware/auth.rs:57`, y `tower` ya está en `[dev-dependencies]` con `util`. Como `anyhow` (RS-1.4). Hallado en P2. | baja | Quitar los dos de `[dependencies]`; `tower` sigue en `[dev-dependencies]`. | abierto: R26 |
| RS-2.2 | RS-2 `panic!` solo invariantes | iam/…/account.rs:36,40,44; account_id.rs:13,17; account_status.rs:18,22; events/account_event.rs:10,14,18; communities/…/in_memory/community_repository.rs:27 | 11 `todo!()` en código de producción. El de `get_public_list` haría caer la petición si se cablea. | media | Implementar o eliminar los `impl` de trait vacíos. | abierto: R18 (a603969) quitó el de `in_memory/community_repository.rs` (hoy `in_memory_community.rs`, sin ninguno); quedan los 10 de iam, con las líneas exactas: `account_id.rs` (R20a), `account_status.rs` (R20b), `account.rs` y `account_event.rs` (R21). R20a (bae89e9) quitó los 2 de `account_id.rs`: quedan 8 |
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
- **`identify` sin normalizar (P1).** La auditoría de P1 lo reportó como HEX-7 (identify_account.rs:30, 36). No se registró porque D10 lo mantenía. D10 se decidió en R12b: `identify` normaliza, sin un ID propio (lo cubre D10 en R12b).
- **`HashedPasswordError` como respuesta de cliente (400).** Solo se produce si el hasher devuelve algo raro, así que sería árbol B. Queda dentro de ERR-1.1 y no aparte.

## 5. Plan de rebanadas

Las rebanadas se citan como **R1…R31** (con las intermedias R10a, R10b, R12b, R13b, R16b, R20a, R20b, R22b y R29b) y las sincronizaciones, como **S1…S4**. Cada una es un PR que tiene que estar verde en CI. En ese mismo PR se actualiza el inventario (`resuelto (<commit>)`) y, al terminar, se vuelve a auditar con `craft-backend:architecture-reviewer`. El avance de cada rebanada se anota en §5.8.

Datos verificados que condicionan el plan:
- ~~`hex_4_use_case_contracts_do_not_expose_domain_types` no actúa sobre los contextos legacy.~~ Desde S1 (ai-harness `v1.0.0-rc.1`) sí actúa: en legacy, el dominio de application es `crate::domain`. Revisa 17 tipos de contrato y pasa en verde.
- Un contexto legacy no puede depender de un crate `edge/*`. Todas las capas virtuales comparten el manifiesto, y `(Domain, Edge)` está prohibido. En `iam` aparecería una entrada nueva en `contexts/iam/Cargo.toml`. En `communities` seguiría viva la entrada de HEX-1.1.
- `hex_2_core_depends_only_on_std_and_the_kernel` cuenta también las `[dev-dependencies]`. Después de la separación, los tests de application no pueden usar tokio.
- Los tests de caracterización fijan `IAM_INVALID_ACCOUNT_ID(_FORMAT)` en `/communities/create`, los bodies `{code,message}` exactos y el 401 con cuerpo vacío. Además documentan como "CURRENT BEHAVIOR" el duplicado por mayúsculas (HEX-7.1).
- `AccountStatus::from_str` no se usa fuera de los tests: ningún adaptador persiste en texto. *(P2)* `AccountFactory::reconstitute` ya no existe (R19, 0a3c49a).
- Decidido por el usuario: `account_lifecycle.rs` admite **adaptación mecánica**.
- `tools/architecture/baseline.txt` tiene 63 líneas: 8 de comentario y **55 entradas**. Coincide con la suma de §5.6. El "56" del encargo original era un error de conteo; ninguna entrada queda sin rebanada. *(En R1. Después de R3 y R4 quedaron 36, y S1 las reescribe como 47 con el formato nuevo. S2 (`v1.0.0-rc.2`) las conserva, idénticas, y S3 (`v1.0.0-rc.3`) conserva las 31 que quedaban después de R13 y suma 11: ver §5.10.)*

### 5.1 Reglas transversales

- **Trinquete.** Cuando un PR mueve o renombra un archivo que figura en `baseline.txt`, en ese mismo PR la violación tiene que desaparecer. Si no, el archivo no se toca. Antes de abrir cada PR: `cargo test -p architecture`.
- **Red de seguridad.** `app/tests/` no se modifica, salvo en los cambios de contrato listados en §5.5.
- **Ritual de rebanada (`/craft-backend:adoption R<n>`).** Lo que este proyecto agrega a la skill. Antes estaba en `.claude/commands/slice.md`:
  - commits en inglés, con Conventional Commits;
  - si la rebanada lo indica en §5.3, se quitan los `#[allow(...)]` que apuntan a sus hallazgos;
  - cada rebanada termina en un PR contra `main`, aunque la invocación no lo pida, con la fila de §5.8 en `Tokens: pendiente` hasta que el usuario pase el `/cost`.
- **`account_lifecycle.rs` (decidido: adaptación mecánica).** Se permite cambiar imports, nombres, `.await`/`block_on` y reemplazar `code()` por un `matches!` sobre la variante equivalente. Ningún escenario ni aserto cambia de significado. El PR enumera cada adaptación.
- **Helpers de `app/tests/common/` (decidido en P1: adaptación mecánica, como lifecycle).** Se permite cambiar imports, nombres, la construcción de la app (como el `.expect("app builds")` de R9) y `.await`, sin que cambie ningún escenario ni aserto. El PR enumera cada adaptación. Los cambios de contrato de §5.5 (p. ej. `assert_error` en R30) siguen siendo cambios de contrato, no adaptaciones.
- **Fallas por operación en los dobles (desde R8).** Cada `Fake…Adapter` tiene `failing()` y `failing_on({Concepto}Operation)`, y cada llamada al puerto de un handler tiene su `fails_with_unexpected_when_<operación>_fails`. `communities` lo adopta en R16.
- **TDD.** El "rojo" de los tests de arquitectura es la entrada de la línea base. Al corregir la violación, el test falla con "Ya no violan: borrar", y el PR borra esa línea. Los tests que hoy afirman sobre `code()` se reemplazan por la versión que afirma sobre la variante: primero se escribe la nueva, en rojo, y después se borra la vieja (ERR-2.6).
- ~~**Verificación manual mientras haya legacy.**~~ Desde S1, el test de contratos (HEX-4) actúa sobre legacy: una filtración de `crate::domain` en `commands/*/{command,response,error}.rs` o en `ports/inbound/` es una violación nueva en la misma rebanada que la introduce. Ya no hace falta el `grep` a mano en R11–R18.
- **Una dependencia, una entrada.** Desde S1, las claves de los tests de dependencias son `ruta/Cargo.toml#sección.paquete`, con `n` = número de capas virtuales que la ven. La rebanada que quita un paquete de un `Cargo.toml` borra sus entradas en el mismo PR, aunque el plan asignara el cierre a otra rebanada.
- **Firma que pasa a ser falible (desde S3, craft-backend 1.0.0-rc.3).** El commit de la firma falla fuerte con `.expect("pendiente: …")`, nunca descarta el `Fault`, y el `?` llega en el commit siguiente, con su test en rojo contra el commit padre. Reemplaza la secuencia de R10a, que descartaba el `Fault` de forma explícita. `rs_2_expect_names_the_invariant` admite `"pendiente:"`, y el revisor lo reporta si llega al final de la rebanada.
- **Baterías de contrato nuevas (desde S3).** Nacen con la forma final aunque el puerto siga síncrono: casos `pub async fn`, macro con `run` invocada con `run: block_on` y `#![expect(clippy::unused_async, reason = "puerto todavía síncrono: …")]` (receta "Puerto todavía síncrono" del README del stack).
- ~~**Mapper de iam (desde S3).**~~ *(P2: ya no aplica; R19 borró el mapper.)* Su entrada `hex_1_driving_adapters_see_only_use_cases` cuenta los códigos de `crate::domain` que nombra (n = 10). Si una rebanada de la fase 3 borra un brazo del mapper y con él un código de dominio, baja la `n` en el mismo PR.
- **La línea base solo crece en una sincronización con el harness.** Esa sincronización va en un PR propio, sin código de producción, y con el análisis de la línea base vieja frente a la nueva: entradas que se mantienen, las que cambian de clave, las que desaparecen y las nuevas, con ID del inventario y rebanada (ver §5.10). **Nunca en una rebanada.** Si una entrada desaparece sin un cambio de formato que lo explique, el PR se detiene: puede ser un cierre en falso.

### 5.2 Errores y estructura: recomendación

En [`archive.md`](harness-adoption/archive.md#52-errores-y-estructura-recomendación): la recomendación ya se aplicó en las fases 1 a 3.

### 5.3 Secuencia

Tamaños: XS (<50 líneas), S (<200), M (<500), L (>500 o muchos archivos).

Las fases 0 a 3, R20a y S4 están hechas: su detalle está en [`archive.md`](harness-adoption/archive.md#53-secuencia-rebanadas-hechas).

#### Fase 4: dominio

*(Revalidada en P2 contra `0eaab25` y contra la plantilla `v1.0.0-rc.3`.)* Cambios de P2 en la fase:
- R20 se parte de antemano en **R20a** (el id) y **R20b** (el código de verificación y el alta). R10 pasó de 700 líneas como M/L, y R20 era M/L con dos temas independientes.
- `uuid` sale de `contexts/iam/Cargo.toml` en R20a y de `contexts/communities/Cargo.toml` en R22. Antes, la línea base no cambiaba en R20. Ahora es −1 en R20a y −2 en R22, que también quita `http`, sin uso (RS-1.5). Sin eso, R27 crearía `communities-domain` con `uuid` (por `OwnerId`), una entrada nueva que la línea base no admite (ver §5.6, "Precondiciones de R27 y R28").
- R21 conserva su alcance, con un punto de corte si pasa de 500 líneas.
- R22 depende de R20a, que crea `infrastructure/uuid`.
- R23 ya no lleva `tokio`: en legacy, `hex_2_core` cuenta las `[dev-dependencies]` de `contexts/iam/Cargo.toml`. `spawn_blocking` y el `run` sobre un runtime real pasan a R28.

**R20b. Alta y restauración de `Account`, y el código de verificación (P2: segunda parte de R20).**
- **IDs:**
  - HEX-7.5 y HEX-2.3;
  - *(P2)* TDD-2.7: `PasswordHashingService::hash -> Result<HashedPassword, Fault>`, y `Account::create` recibe el `HashedPassword`. Desaparecen `HashedPasswordError` en `create`, su brazo en el handler y el `StubPasswordHashingAdapter`. El caso de la batería `hash_is_a_valid_hashed_password` pasa a cumplirse por tipo;
  - *(P2)* TDD-2.11: el nombre de `reconstituted_user_preserves_status` (abajo);
  - HEX-7.7 y ERR-2.5 (se borra `AccountStatus::from_str`, que no tiene uso fuera de los tests, junto con sus 3 tests). *(R19)* `reconstitute` (ERR-1.5) ya lo borró R19 (0a3c49a). *(P2, verificado)* Los helpers de test que usan `from_str` son `AccountFactory::account` (detrás de `dummy_account` y `dummy_account_with_status`, en `account_factory.rs:176`) y `dummy_account` de `authenticate_account/handler.rs:129`. Pasan a recibir un `AccountStatus` construido directamente (`AccountStatus::Suspended`, `AccountStatus::Registered { code_validation }`…). Cambian las 26 llamadas: 14 en `account.rs`, 9 en `authenticate_account/handler.rs`, 2 en `verify_account/handler.rs` y 1 en `identify_account/handler.rs`. Es un refactor de tests: ningún aserto cambia;
  - RS-2.2 (`account_status.rs`).
- **Cambios:**
  - `Account::create`/`restore` con value objects, y `new` privado. Los helpers de test usan `restore`;
  - *(R19)* `reconstituted_user_preserves_status` (`account.rs:265`) conserva un nombre de cuando existía `reconstitute`: pasa a `restored_account_preserves_status`, sobre `Account::restore`;
  - el código de verificación sale de un puerto `VerificationCodeService` (`RandVerificationCodeAdapter`), y `CodeValidation` deja de usar `rand`. `rand` sigue en `contexts/iam/Cargo.toml`, porque Argon2 usa `rand::rngs::OsRng` para la sal, así que su entrada no cambia hasta R28;
  - *(S3)* la batería nueva del generador de códigos nace con la forma async (§5.1, "Baterías de contrato nuevas"); el puerto va en R23 con los demás;
  - *(S3)* si la firma de `VerificationCodeService` es falible, rige §5.1 ("Firma que pasa a ser falible");
  - se quitan el `should_implement_trait` de `account_status.rs` y el `module_inception` de `model/account`;
  - `AccountStatus` deja de implementar `ValueObject`, y desaparecen sus 2 `todo!()` (HEX-8.10, parcial).
- **Precondición:** R20a hecha (`AccountFactory::create` ya recibe el id). *(S4)* S4 hecha: R20b es la primera rebanada con `v1.0.0-rc.4`.
- **Línea base: −1** *(S4; antes sin cambios)*: `hex_2_core_imports_only_std_and_the_kernel contexts/iam/src/domain/model/account/code_validation.rs` (`rand`, HEX-2.3). La entrada de manifiesto `contexts/iam/Cargo.toml#dependencies.rand 2` no cambia: Argon2 sigue usando `rand` hasta R28.
- **Rojo:**
  - `create_uses_the_given_id_and_code` (dominio);
  - batería del generador de códigos: `generates_six_digit_codes`;
  - *(S4)* el trinquete de `code_validation.rs`.
- **Riesgo:** `verification_code_is_six_digits`.
- **Caracterización:** lifecycle se adapta, si construye la app con el puerto nuevo.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** M.

**R21. Agregado inmutable y eventos.**
- **IDs:**
  - HEX-7.3, HEX-7.4, HEX-8.9, HEX-8.10, HEX-8.11, HEX-8.12, HEX-8.2 y HEX-2.4;
  - *(P2)* HEX-7.13: con el criterio de D4, se borran `activate`, `deactivate`, `suspend`, `change_password`, `change_email` y `change_username`, que no tienen llamador de producción, con sus tests. Así HEX-7.3 solo vuelve inmutable a `confirm_registration`, y R21 se achica. Si el usuario prefiere conservar alguno, decide con `matches!` y no con `as_str()`;
  - *(P2)* ERR-2.12: `confirm` devuelve un error propio de la transición, `InvalidVerification`, y se borra `AccountError`. El `match` de `VerifyAccountHandler` traduce ese único tipo;
  - VER-1.7 (`todo = "deny"` y `dbg_macro = "deny"`, cuando ya no queda ningún `todo!()`). *(P2, verificado)* Después de R20a y R20b quedan 6 `todo!()`, todos de esta rebanada: `account.rs:36,40,44` (`record_event`, `pull_events`, `version`) y `account_event.rs:10,14,18`;
  - *(S1)* HEX-2.1 (parcial: `chrono` en `contexts/iam/Cargo.toml`) y HEX-8.1 (parcial: `chrono` en `shared/Cargo.toml`);
  - RS-2.2 (`account.rs` y `account_event.rs`).
- **Cambios:**
  - `confirm(&self, …) -> Result<Self, …>`;
  - `AggregateRoot` y `ValueObject` como marcadores (HEX-8.9, HEX-8.10), con `model.rs` del template; se borra `EventRecorder`;
  - *(S3, revalidado en P2)* los cuatro marcadores de `shared/src/domain` quedan sin ítems (`hex_8_model_markers_have_no_items`), como en `kernel/src/model.rs` de rc.3, que define exactamente `pub trait ValueObject: Clone + Eq {}`, `pub trait AggregateRoot {}` y `pub trait DomainEvent: Clone {}`:
    - `AggregateRoot` y `ValueObject` quedan como en la plantilla, en un `shared/src/model.rs` (la disposición de `kernel/src`, para que R29 sea un `git mv`);
    - `DomainEvent`: D4 borra `AccountEvent`, su única implementación. Se deja `pub trait DomainEvent: Clone {}`, como en la plantilla, en lugar de borrarlo, para que el kernel coincida con ella (HEX-8.11). Sin `chrono`;
    - ~~`Entity` (HEX-8.12): la plantilla no lo trae. Se borra, y `Account` conserva su `id()` propio.~~ *(S4)* `Entity` (HEX-8.12) **queda como marcador vacío y no se borra**: `pub trait Entity {}` en `shared/src/model.rs`, junto a los otros. Desde 1.0.0-rc.3.7, `Entity` e `Identifier` son marcadores admitidos (`MODEL_MARKERS`), y el `model.rs` de rc.4 dice que un proyecto que los usa los agrega sin ítems. `Account` sigue implementándolo y conserva un `id()` propio, fuera del trait;
    - *(P2, verificado)* `Email`, `Username`, `HashedPassword` y `CodeValidation` pasan `new`/`value` a métodos propios (`AccountId` y `AccountStatus` ya salieron en R20a y R20b). Importan el trait 17 archivos, contando `value_object.rs`. Después de R20 quedan 15, incluido `account_lifecycle.rs` (adaptación mecánica: el import);
  - **D4 decidida: se retira el andamiaje muerto.** Se borran `AccountEvent` (`model/account/events/`), `record_event`/`pull_events` (sus 3 `todo!()`) y `version`, porque no tienen comportamiento ni uso. No se agrega `take_events`;
  - **(P1, por D4) `Clock` no entra en R21.** *(P2)* La plantilla rc.3 lo tiene en `kernel/src/clock.rs`, y `infrastructure/system` trae `SystemClockAdapter`. No se copian: sin eventos nada pide la hora, y un trait sin uso no tiene test que lo exija. Entran con la primera rebanada que necesite la hora. R29 no los necesita: ningún test de `tools/architecture` exige que el kernel defina `Clock`;
  - *(S1)* `chrono` sale de `contexts/iam/Cargo.toml` y de `shared/Cargo.toml`. *(P2, verificado)* Sus únicos usos son `account_event.rs:1` y `shared/src/domain/domain_event.rs:1`; JWT no lo usa.
- **Línea base: −9** *(S4; S3: −7; S1: −3)*: `hex_2_core` `contexts/iam/Cargo.toml#dependencies.chrono` y `shared/Cargo.toml#dependencies.chrono`, `hex_8_kernel_depends` `shared/Cargo.toml#dependencies.chrono`, las 4 de `hex_8_model_markers_have_no_items` (`aggregate_root.rs`, `domain_event.rs`, `entity.rs`, `value_object.rs`) y, desde S4, las 2 de `hex_2_core_imports_only_std_and_the_kernel` de `chrono`: `contexts/iam/src/domain/model/account/events/account_event.rs` (HEX-2.4, se borra por D4) y `shared/src/domain/domain_event.rs` (HEX-8.2, queda `pub trait DomainEvent: Clone {}`). **Cierra `hex_8_model_markers_have_no_items`.**
- **Rojo:**
  - `confirm_returns_the_active_account_and_leaves_the_original_untouched`;
  - el trinquete de las 3 entradas de `chrono` por manifiesto, de las 2 por archivo y de las 4 de los marcadores. Sin eventos (D4) no hay `create_records_registered`.
- **Tamaño:** M/L *(S3; antes M: los marcadores tocan 4 value objects y sus llamadores)*. *(P2)* Si pasa de unas 500 líneas, se parte en dos PRs, como R10: **R21a**, los marcadores, D4 y `chrono` (−9 desde S4), y **R21b**, el agregado inmutable (HEX-7.3), sin cambios en la línea base. R21a va primero, porque R25 necesita `chrono` fuera de `shared`.

**R22. Dominio de `communities`.**
- **IDs:** HEX-2.6, HEX-7.6 (communities), HEX-7.8, HEX-7.9 y *(R16b)* RS-2.6 (`impl FromStr for OwnerId`). *(P2)* HEX-2.7 (`OwnerId` sin `uuid`), HEX-7.14 (communities: se borran `CommunityId::from_uuid`/`from_str` e `InvalidCommunityId`, que nunca tuvieron llamador de producción, en lugar de convertirlos en `FromStr`), TDD-2.9 (el dueño en la batería) y RS-1.5 (`http` sin uso).
- **Precondición (P2):** R20a hecha. Crea `infrastructure/uuid`, que R22 cablea para communities.
- **Cambios:**
  - `CommunityId` sobre `String`, con `IdGenerator` inyectado en `CreateCommunityHandler`. `as_uuid().to_string()` en `create_community/handler.rs:82` pasa a `as_str()`;
  - *(P2)* `OwnerId` deja de usar el crate `uuid`. Valida a mano, con std, la forma con guiones (8-4-4-4-12 dígitos hexadecimales) y rechaza el nil, con los mismos `InvalidOwnerId { Nil, WrongFormat }` (D2 a). Con eso, nada de communities usa `uuid`, que sale del manifiesto. **Diferencia que la rebanada tiene que decidir con el usuario:** `Uuid::parse_str` también acepta la forma sin guiones, con llaves y `urn:uuid:`. Con el validador nuevo, esas formas pasan a `WrongFormat`. Ningún token emitido por iam las usa (el sujeto es el `next_id()` de `UuidIdGeneratorAdapter`, con guiones), y ningún test de `app/tests/` las fija. Si el usuario prefiere conservarlas, el validador también las acepta;
  - **D4 decidida:** se borran `MembershipPolicy` (`domain/services/`, que queda vacío), el campo `membership_policy` de `Community` con su getter, y `MembershipPolicyChanged` (`model/community/events/`). No tienen comportamiento ni uso. HEX-7.8 se resuelve borrando, no moviendo;
  - se quitan los `allow`: `should_implement_trait` de `community_id.rs` (se borra el método, HEX-7.14) y de `owner_id.rs` (RS-2.6, `impl FromStr for OwnerId`), y `module_inception` de `model/community/mod.rs`;
  - *(P2, TDD-2.9)* `Community::owner_id() -> &OwnerId` (el campo `_owner_id` pasa a `owner_id`); `snapshot` de la batería de `CommunityRepository` suma el dueño, y `create_community_successfully` lo afirma;
  - *(P2, RS-1.5)* `http` sale de `contexts/communities/Cargo.toml`: no tiene ningún uso desde R17.
- **Línea base: −4** *(S4; P2: −2; antes sin cambios)*: `hex_2_core_depends_only_on_std_and_the_kernel contexts/communities/Cargo.toml#dependencies.uuid 2` y `…#dependencies.http 2`, y desde S4 las 2 de `hex_2_core_imports_only_std_and_the_kernel` de `uuid`: `contexts/communities/src/domain/model/community/community_id.rs` (HEX-2.6) y `…/owner_id.rs` (HEX-2.7).
- **Rojo:**
  - handler `creates_with_the_generated_id`;
  - dominio: `owner_id_rejects_non_uuid`, `owner_id_rejects_nil` y `owner_id_accepts_a_uuid` siguen en verde con el validador nuevo. Se suman casos de forma: un grupo corto, un carácter no hexadecimal y los guiones en otro lugar. Si el usuario elige rechazar las otras formas, se suma `owner_id_rejects_the_simple_form`;
  - el trinquete de `uuid` y de `http`;
  - batería: con el dueño en `snapshot`, un doble que guarde otro dueño la pone en rojo (por mutación del Fake, como en R16b).
- **Riesgo:** `create_returns_200_with_id…`, que exige un UUID de 36 caracteres; lo cumple el adaptador UUID. `create_with_non_uuid_subject_returns_400` y `create_with_nil_uuid_subject_returns_400` no cambian.
- **Tamaño:** S/M *(P2; antes S: suma `OwnerId` sin `uuid`)*.

**R22b. `Field` cerrado en las hojas (nueva en S4; confirmada por el usuario, D15).**
- **Por qué aquí y no en R30:** la forma de referencia de 1.0.0-rc.3.2 (`Validation { field: Field }`, con `Field::Owner(OwnerRejection)` para un campo con varios motivos) no depende de Problem Details: cambia los tipos de las hojas y los `match` de los `fold`, no el cuerpo de la respuesta. Hacerla antes achica R30, que es M/L y partible, y cierra antes ERR-4.13, cuyo defecto (un campo nuevo que cae en una respuesta inventada) existe desde hoy. Va después de R20b y R22 porque las tres tocan `register_account/handler.rs` y `create_community/handler.rs`, y R22 deja estable `InvalidOwnerId`. No toca crates ni movimientos, así que no espera a R27 ni a R28.
- **IDs:** ERR-4.13 (iam y communities).
- **Cambios:**
  - communities: `enum Field { Slug, Name, Owner(OwnerRejection) }` y `enum OwnerRejection { Nil, WrongFormat }` en `application/errors/`, junto a `Validation { field: Field }`. Se borra la hoja `InvalidOwner` (`application/errors/invalid_owner.rs`) y la variante `CreateCommunityError::InvalidOwner`: el handler traduce `InvalidOwnerId` a `Validation { field: Field::Owner(…) }`;
  - iam: `enum Field { Username, Email, Password, Code }`, con `Validation { field: Field }`. `PasswordPolicyError` tiene hoy una sola variante (`TooShort`), así que `Field::Password` no lleva motivo; si gana otra, pasa a `Field::Password(PasswordRejection)`;
  - los `fold` (`register_account.rs:40`, `verify_account.rs:55`, `create_community.rs:53`) hacen `match` sobre `Field` sin comodín. Desaparecen los tests de un campo desconocido, que ya no se puede construir: `an_unmapped_validation_field_returns_500` de `register_account.rs:123`, `verify_account.rs:183` y `create_community.rs:200`;
  - `Conflict { resource }` y `NotFound { resource }` siguen con `&'static str`, como en la plantilla rc.4 (`Conflict { resource: &'static str }`). Sus `an_unmapped_…_resource_returns_500` se conservan.
- **Línea base:** sin cambios. Ningún test de `tools/architecture` mira el tipo del campo.
- **Rojo:** es un refactor de tipos sin cambio de comportamiento (receta de 1.0.0-rc.3.6: un test que fija comportamiento existente se ve en rojo por mutación, anotada en el commit). Los tests de handler por campo (`fails_when_owner_id_is_invalid`, `fails_when_owner_id_is_nil` y los de validación de iam) y los de endpoint por brazo ya existen, y siguen por nombre.
- **Contrato HTTP:** sin cambios. Los códigos `IAM_INVALID_ACCOUNT_ID(_FORMAT)` del owner (D2 a) y los de cada campo se conservan byte a byte.
- **Tamaño:** S/M.

#### Fase 5: puertos async (paralelizables entre sí)

**R23. iam async.** **R24. communities async.**
- **IDs:** RS-3.1. *(P2)* R23 además TDD-2.10.
- **Lo que queda después de R16b (revalidado en P2):** R16b dejó las 4 baterías (3 de iam y 1 de communities) con la forma final, con `run: block_on` en sus 9 invocaciones y `#![expect(clippy::unused_async)]`. `block_on` ya está en `application/testing.rs` de cada contexto. Queda:
  - `BoxFuture` en `shared/src/future.rs`, copiado de `kernel/src/future.rs` de rc.3 (std solo). Lo agrega la primera de las dos que se haga;
  - los puertos salientes pasan a `BoxFuture<'_, Result<…, Fault>>`. R23: `AccountRepository`, `PasswordHashingService`, `AccessTokenService` y el `VerificationCodeService` de R20b. R24: `CommunityRepository`. `IdGenerator` es una capacidad del kernel y sigue síncrona, como en la plantilla;
  - los puertos entrantes pasan a `BoxFuture`. R23: los 5 de iam (register, verify, authenticate, identify y verify_access_token). R24: `CreateCommunityUseCase`;
  - handlers, dobles y adaptadores en memoria, con `Box::pin(async move { … })`;
  - en las baterías, los `.await` de los helpers y de las llamadas directas al puerto, y se borra el `#![expect(clippy::unused_async)]`, que `unfulfilled_lint_expectations` obliga a quitar. La macro y sus invocaciones no cambian;
  - los tests de handler pasan a `block_on(handler.execute(…))`;
  - los `fold` de `infrastructure/http` y el middleware de `app` hacen `.await`. Con eso desaparece el `poll` con `Waker::noop()` de `infrastructure/http/testing.rs`, que hoy depende de que todo future esté listo en el primer `poll`. Los tests de endpoint corren con `block_on`;
  - `app/src/containers/` no cambia de forma: sigue exponiendo `Arc<dyn …UseCase>`;
  - *(P2)* los comentarios de módulo de `application/testing/contracts.rs` de cada contexto (l.3-6) todavía dicen que R23/R24 pasan los casos a `async fn` y agregan `run`, cosa que ya hizo R16b: se corrigen;
  - *(P2, TDD-2.10, solo R23)* `SpyPasswordHashingAdapter` gana `failing()` y `failing_on(op)`, delegando en el Fixed interno.
- **Lo que no puede ir aquí (P2):** el plan ponía en R23 Argon2 con `spawn_blocking`, el `run` de Argon2 y JWT sobre un runtime real y una batería bajo `#[tokio::test]`. Los tres necesitan `tokio` en `contexts/iam/Cargo.toml`. En legacy, domain y application comparten ese manifiesto, y `hex_2_core_depends_only_on_std_and_the_kernel` cuenta también las `[dev-dependencies]`: sería una entrada nueva, que la línea base no admite. **Pasan a R28**, al manifiesto de `iam-infrastructure`. Hasta entonces, Argon2 bloquea el hilo del runtime dentro del future, como hoy, y sus baterías siguen con `run: block_on`.
- **Rojo:** ninguno nuevo. Es un refactor sin cambio de comportamiento: los mismos tests por nombre antes y después (`cargo test --workspace -- --list`). El rojo se ve por mutación, como en R16b (un handler que no hace `.await` de un puerto no compila; uno que descarta el `Result` pone en rojo su `fails_with_unexpected_when_<operación>_fails`).
- **Caracterización:** lifecycle se adapta (`block_on` o `.await`).
- **Contrato HTTP:** sin cambios.
- **Tamaño:** R23, M; R24, S *(P2; antes M cada una: communities tiene un puerto de cada lado)*.

#### Fase 6 y 7: kernel y estructura

**R25. `shared` sin dependencias.**
- **IDs:** HEX-8.1 (el resto: `serde`; `chrono` sale en R21, desde S1). HEX-8.2 ya quedó resuelto en R21. *(P2)* HEX-8.8: el tipo de transporte sale del kernel en esta rebanada, así que el hallazgo se resuelve aquí. R30 borra las copias.
- **Cambios:** `ApiErrorResponse` se copia temporalmente a cada `infrastructure/http`; R30 lo elimina. *(P2)* Lo importan 7 archivos: en iam, `unexpected.rs` y los 4 `fold` (`register_account.rs`, `verify_account.rs`, `authenticate_account.rs`, `identify_account.rs`); en communities, `unexpected.rs` y `create_community.rs`. Va una copia por contexto, en `infrastructure/http/api_error_response.rs`, con `serde`, que ya está en los dos manifiestos de contexto. Se borra `shared/src/infrastructure/` entero, que solo contiene ese archivo, y `serde` sale de `shared/Cargo.toml`.
- **Revalidado en P2 contra rc.3:** el kernel de la plantilla (`kernel/Cargo.toml`) no tiene dependencias, así que la rebanada sigue siendo correcta. No puede esperar a R30 y borrar `ApiErrorResponse` sin copiarlo: R30 necesita `edge/http`, y un contexto legacy no puede depender de un crate Edge (§5.2), así que R30 va después de R27 y R28, y R29 necesita a `shared` sin entradas.
- **Precondición:** R21 hecha (saca `chrono`). Si no, `shared/Cargo.toml` conserva una dependencia y `hex_8_kernel_depends_only_on_std` no se cierra.
- **Línea base: −3** *(S4; antes −2)*. *(S1: con las claves por dependencia, son `hex_8_kernel_depends` y `hex_2_core` de `shared/Cargo.toml#dependencies.serde`. Las de `chrono` las cierra R21.)* *(S4)* Más `hex_2_core_imports_only_std_and_the_kernel shared/src/infrastructure/http/api_error_response.rs` (`serde`, HEX-8.8), que se borra con `shared/src/infrastructure/`. Las copias de cada contexto van a `infrastructure/http`, que no es núcleo, así que no suman entradas. **Cierra `hex_8_kernel_depends_only_on_std`.**
- **Rojo:** el trinquete de las 3 entradas. Ningún cuerpo de error cambia: lo fijan `assert_error` y los tests de endpoint por brazo.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** S.

**R26. `edge/http` e `infrastructure/stderr`.**
- **IDs:** ERR-4.7 (parcial), ERR-1.8 y ERR-4.12. *(P2)* RS-1.6: `serde` y `tower` salen de `[dependencies]` de `app`.
- **Cambios:**
  - crates nuevos con `Problem`, `unexpected()`, `AuthenticatedAccount` y `StderrFaultReporterAdapter`, más los tests del template;
  - `CatchPanicLayer` en `app/src/server.rs`;
  - *(P1)* `AppConfig::load()` propaga el `ConfigError` con `?` en lugar de `process::exit` (ERR-1.8), y `main` reporta el `Fault` de arranque con el `FaultReporter`, que muestra la causa (ERR-4.12).
- **Revalidado en P2 contra rc.3:** la plantilla trae los tres crates que esta rebanada copia: `edge/http` (`problem.rs` con `Problem`, `CONTENT_TYPE_PROBLEM` y `unexpected(fault, reporter)`, e `identity.rs` con `AuthenticatedAccount`), `infrastructure/stderr` (`StderrFaultReporterAdapter`, que ya recorre la cadena `source()`, como pedía S1) e `infrastructure/system` (`SystemClockAdapter`). Siguen siendo correctos:
  - en legacy, `edge-http` e `infrastructure-stderr` dependen de `shared` en lugar de `kernel`, como `infrastructure-uuid` en R20;
  - `infrastructure/system` **no** se copia: depende de `Clock`, que por D4 no tiene uso (R21);
  - ~~el `unexpected()` de la plantilla responde `Problem::internal()` sin `code` ni `detail`. R26 copia la plantilla tal cual, y R30 resuelve la diferencia.~~ *(S4)* R26 copia `edge/http` de `v1.0.0-rc.4`: `unexpected(fault, reporter)` reporta el `Fault` y devuelve un `UnexpectedProblem` (exportado en `lib.rs`), que solo admite `.with_code(…)` e implementa `IntoResponse` (`#[must_use]`). Con eso, `unexpected(&f, r).with_code(codes::INTERNAL_ERROR)` es la forma oficial del 500 con `code`, y **reemplaza a los `unexpected.rs` de cada contexto** cuando cada contexto adopte `edge/http` (R30, ver allí). No hace falta un `unexpected()` propio en cada `{ctx}-api`. Los tests de la plantilla que trae: `serializes_as_problem_json_with_extension_code` y `unexpected_reports_the_fault_and_hides_the_detail`, que afirma `{"type":"/problems/unexpected","title":"Unexpected error","status":500,"code":"INTERNAL_ERROR"}`;
  - `CatchPanicLayer` necesita la feature `catch-panic` de `tower-http`, que hoy tiene solo `trace` en `[workspace.dependencies]`.
- **Línea base:** sin cambios. Solo depende de ellos `app`, que es Bootstrap. Con el primer `FaultReporter` de producción, `hex_8_capability_adapters_are_shared_infrastructure` revisa un adaptador más, en `infrastructure/stderr`, donde la regla lo admite.
- **Rojo:**
  - `serializes_as_problem_json_with_extension_code`;
  - `unexpected_reports_the_fault_and_hides_the_detail`;
  - un test de `app`: `a_panicking_route_returns_500`.
- **Contrato HTTP: cambia (sin cobertura previa).** Un panic responde 500 en lugar de cortar la conexión.
- **Tamaño:** M.

> **Punto de replanificación P3 (después de R26, antes de R27).** Ver §5.9. *(S4: S4 ya se hizo, antes de R20b; P3 no la espera.)* R26 corre en paralelo desde R5, así que P3 se hace cuando están hechas R22, R22b, R24, R25 y R26, las precondiciones de la separación. Es la última revisión antes de las rebanadas L.

**R27. Separar `communities` en crates por capa.**
- **IDs:**
  - RS-1.1 (parcial), HEX-2.5;
  - HEX-1.9 (communities);
  - HEX-6.1 (communities: `CreateCommunityHttpRequest/Response`);
  - ERR-4.8 (`Option<Extension<AuthenticatedAccount>>`);
  - HEX-8.6 (se borra `VerifiedIdentity`; el middleware de `app` inserta `AuthenticatedAccount`).
- **Estructura:** `contexts/communities/{domain,application,infrastructure,api}`, con `CommunitiesApi`.
- **Precondición del trinquete:** de `contexts/communities/src`, en la línea base solo quedan `requests/create.rs` y `responses/created.rs`. Se renombran en este mismo PR. *(P2)* Y de `contexts/communities/Cargo.toml`, solo `axum` y `serde` (`hex_2_core`) y `axum` (`hex_2_frameworks`), que en la estructura nueva solo declara `communities-api`. Ver §5.6, "Precondiciones de R27 y R28".
- **Línea base: −5** *(P2; S1: −7; antes −4)*: las 2 `hex_2_core` de `contexts/communities/Cargo.toml` que quedan (`axum`, `serde`; `uuid` y `http` salen en R22), la `hex_2_frameworks` (`axum`, n = 3), porque el manifiesto desaparece, y los dos `hex_6`.
- **Rojo:** `communities-api/tests/create_community.rs`: `responds_401_without_authenticated_account`.
- **Riesgo:** ~~el test de contratos HEX-4 se activa en este PR~~. Desde S1 ya actúa en legacy. El riesgo que queda es que un crate nuevo (`communities-application`) declare más dependencias de las que pide su capa.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** L (sobre todo, archivos movidos).

**R28. Separar `iam` y crear `IamApi::protect`.**
- **IDs:**
  - RS-1.1, HEX-2.1;
  - HEX-1.6, HEX-1.9; ~~HEX-1.7~~ ya resuelto en R19 (11f8c92), que borró el mapper;
  - HEX-6.1 (iam);
  - TDD-1.8 (test de arranque del señuelo);
  - *(P2)* RS-3.1, el resto: `spawn_blocking` en Argon2 (de R23).
- **Cambios:**
  - el middleware sale de `app` y va a `iam-api`;
  - `app` compone `iam.protect(communities.protected())`;
  - lifecycle se muda a `contexts/iam/infrastructure/tests/`;
  - *(P1)* `IamApi` recibe sus adaptadores desde `app`, así que el arranque admite un hasher que falle: test `startup_fails_when_the_decoy_hash_fails` (TDD-1.8).
  - *(P2, desde R23)* Argon2 con `spawn_blocking` en `iam-infrastructure`, que puede declarar `tokio`; sus baterías y las de JWT pasan su `run` a un runtime real (`#[tokio::test]` o un `run` con `Runtime::block_on`). En legacy no se podía: `tokio` en `contexts/iam/Cargo.toml` habría sido una entrada nueva de `hex_2_core`. Rojo: la batería async de Argon2 bajo el runtime.
- **Precondición:** de `contexts/iam/src`, en la línea base solo quedan los 7 `hex_6`. *(P2)* Y de `contexts/iam/Cargo.toml`, solo `argon2`, `axum`, `jsonwebtoken`, `rand` y `serde` (`hex_2_core`) y `axum` (`hex_2_frameworks`), que en la estructura nueva solo declaran `iam-infrastructure` e `iam-api`. Ver §5.6, "Precondiciones de R27 y R28".
- **Línea base: −13** *(P2; S1: −14; antes −9)*: las 5 `hex_2_core` de `contexts/iam/Cargo.toml` que quedan (`argon2`, `axum`, `jsonwebtoken`, `rand`, `serde`; `uuid` sale en R20a y `chrono` en R21), la `hex_2_frameworks` (`axum`, n = 3) y los 7 `hex_6`. **Cierra `hex_2_core…`, `hex_2_server_frameworks…` y `hex_6_transport…`.**
- **Rojo:** `iam-api/tests/authenticate.rs`: `protect_rejects_missing_token_with_401` y `protect_inserts_authenticated_account`.
- **Riesgo:** `unknown_route_without_token_returns_401`. Si `protect` deja de envolver el fallback, la respuesta pasa a 404 y es un cambio de contrato que el usuario decide.
- **Tamaño:** L.

**R29. `shared` pasa a ser `kernel/` y se borra la adaptación legacy.**
- **IDs:** RS-1.2.
- **Cambios:**
  - `git mv shared kernel`;
  - en `tools/architecture/tests` se borra todo lo marcado `legacy:`, más `tests/legacy.rs` y `tests/fixtures/legacy/` *(S1: así lo indica el template desde `v1.0.0-rc.1`; antes eran solo `legacy_layers` y el fallback a `shared/`)*.
- **Precondición:** no queda ninguna entrada de `shared/` (se cumple desde R25).
- **Línea base:** vacía.
- **Tamaño:** S.

**R29b. `app/` pasa a `apps/server` (nueva en P2; D9).**
- **IDs:** ninguno (D9).
- **Precondición:** R29 hecha.
- **Cambios:** `git mv app apps/server`, con `members` del `Cargo.toml` raíz, la ruta de `app` en `[workspace.dependencies]` si figura, `.github/workflows/` y las rutas de este documento y de §5.1 (`app/tests/`, `app/tests/common/`). El nombre del paquete se decide en la rebanada: el de la plantilla (`apps/server/Cargo.toml`) o `app`, si cambiarlo obliga a tocar los `use app::…` de los tests de caracterización.
- **Línea base:** sin cambios. `tools/architecture` clasifica `app/` y `apps/*` como Bootstrap.
- **Red de seguridad:** `app/tests/` se mueve entero, sin cambiar ningún escenario ni aserto. Si hay que cambiar imports por el nombre del paquete, es adaptación mecánica (§5.1).
- **Rojo:** ninguno. Es un movimiento sin cambio de comportamiento; se verifica con los mismos tests, por nombre, antes y después (`cargo test --workspace -- --list`).
- **Contrato HTTP:** sin cambios.
- **Tamaño:** XS (salvo los archivos movidos).

**R30. Problem Details (D5 decidida).**
- **IDs:** ERR-4.6 y ERR-4.7 (resto: los `fold` usan `unexpected(fault, reporter).with_code(codes::INTERNAL_ERROR)`, desde S4). ~~ERR-4.13~~ pasa a R22b (S4). *(P2)* HEX-8.8 se resuelve en R25, que saca `ApiErrorResponse` del kernel; aquí se borran sus copias.
- **Cambios (D5):**
  - todas las respuestas de error pasan a `application/problem+json` con el `Problem` de `edge/http` (R26): `title` genérico por status, el mensaje de cliente de hoy en `detail` y `code` como miembro de extensión, con los mismos valores de `codes.rs`;
  - los 401 del middleware pasan de cuerpo vacío a `Problem::unauthorized()`;
  - ~~el 500 conserva `code: INTERNAL_ERROR` y el mensaje de D7 en `detail`. Se propone que el `unexpected()` de cada `{ctx}-api` reporte el `Fault` y responda `Problem` con esos dos datos. Si S4 trae una forma oficial, se usa esa.~~ *(S4)* S4 trae la forma oficial: `unexpected(&fault, reporter).with_code(codes::INTERNAL_ERROR)` de `edge/http` (1.0.0-rc.3.5) **reemplaza a los `unexpected.rs` de cada contexto** (`contexts/iam/src/infrastructure/http/unexpected.rs` y `contexts/communities/src/infrastructure/http/unexpected.rs`), con sus tests `unexpected_fault_maps_to_internal_server_error`. Los `fold` y el middleware de `app` agregan `.into_response()` donde necesiten un `Response`. El `eprintln!` interino (R5) desaparece: el `Fault` llega al `FaultReporter` (R26). **Diferencia con D5, que decide el usuario:** `UnexpectedProblem` no admite `with_detail`, a propósito (ningún detalle llega al cliente), así que el 500 queda con `code: INTERNAL_ERROR` y **sin** el mensaje de D7 en `detail`. D5 lo pedía. O se acepta el 500 sin `detail` (D5 se ajusta, y es un cambio de contrato más en §5.5), o se pide al harness un `detail` fijo de cliente para `UnexpectedProblem`. *(D14)* **Decidido: se acepta el 500 sin `detail`**, con `code: INTERNAL_ERROR`; es un cambio de contrato de R30 (§5.5);
  - se borran los `ApiErrorResponse` interinos de cada contexto (R25);
  - ~~ERR-4.13: con el `Problem`, `Validation` lleva un motivo cerrado, y la hoja `InvalidOwner` se puede fundir en `Validation { field: "owner_id", reason }`.~~ *(S4)* Pasa a R22b, con la forma de referencia `Field::Owner(OwnerRejection)`. R30 recibe los `fold` con `Field` cerrado.
- **Rojo:** tests HTTP con el formato nuevo, un test por hoja en cada `{ctx}-api`, y `assert_error` con el formato nuevo contra el código de antes.
- **Contrato HTTP: cambia todo el cuerpo de error (D5).** Se modifican `assert_error` (`app/tests/common/mod.rs`, o `apps/server/tests/common/mod.rs` después de R29b) para que lea `code` y `detail` del Problem y afirme el `content-type`, y las dos aserciones `raw_body == ""` de los 401 (`communities_api.rs:47` y `auth_api.rs:643`), que pasan a afirmar el cuerpo Problem. Cambian el cuerpo y el `content-type`, no los status ni los `code`.
- **D1:** aquí se revisa si verificar una cuenta ya verificada pasa de 400 `IAM_ACCOUNT_INVALID_VERIFICATION` a 409. Si cambia, es una decisión del usuario y una fila más de §5.5.
- **Tamaño:** M/L *(P2; antes M)*. Toca los 5 `fold`, el middleware y todos los asertos de cuerpo de error de `app/tests/`. Si el diff pasa de unas 500 líneas, se parte en dos PRs, como R10: (a) `iam`, con el middleware y `assert_error`, y (b) `communities`.

**R31. El username no distingue mayúsculas (nueva en P2; D13 b).**
- **IDs:** D13. No hay un hallazgo en el inventario: es una decisión de producto, como D10.
- **Precondición:** R30 hecha, para que el cambio de contrato se escriba una sola vez con el formato Problem.
- **Cambios:**
  - `Username` conserva la forma con la que se registró (recortada) y expone una clave de comparación sin mayúsculas. Por ejemplo, `Username::key()`: el valor en minúsculas, con `to_lowercase` de std;
  - el puerto busca y compara por la clave: `find_by_username` recibe el `Username` y el adaptador compara claves; la batería suma `finds_by_username_in_any_casing` y `save_keeps_the_registered_casing`;
  - el alta rechaza un username que ya existe con otra combinación de mayúsculas (`Conflict { resource: "username" }`);
  - authenticate e identify encuentran la cuenta con cualquier combinación, e identify responde el username tal como se registró.
- **Rojo:**
  - dominio: `usernames_that_differ_only_in_casing_have_the_same_key` y `username_keeps_its_registered_casing`;
  - handler: `fails_with_conflict_when_username_differs_only_in_casing`, `authenticates_with_a_username_in_another_casing` e `identifies_with_a_username_in_another_casing`;
  - HTTP: los de §5.5.
- **Contrato HTTP: cambia (§5.5).** `POST /auth/register` con un username que solo difiere en mayúsculas de uno existente, de 201 a 409 `IAM_ACCOUNT_USERNAME_ALREADY_EXISTS`. `POST /auth/authenticate` e `/auth/identify` con el username en otras mayúsculas, de 401 `IAM_LOGIN_FAILED` y 404 a la respuesta de esa cuenta. `identify` responde el username registrado, no el enviado.
- **Riesgo:** D6. Authenticate tiene que seguir sin revelar si la cuenta existe: el camino del señuelo no cambia.
- **Tamaño:** S/M.

### 5.4 Dependencias y paralelismo

```
R1 ─┬─ R2 ─────────────────────────────┐
    ├─ R3, R4, R6, R18   (paralelas)   │
    └─ R5 ─┬─ R7 (+R18) ──────┬─ R17 ──┤
           │        R16 ──────┘        │
           ├─ R8 ─ R9 ─ R10 ─[P1]─ R11 ─ R12
           │             └─ R13, R14(+R2), R15 ┴─ R19 ─[P2]─ R20a ─ S4 ─┬─ R20b ─ R21 ─ R23 ─┐
           │                                                            ├─ R22 ─ R24 ────────┤
           │                                                            └─ R22b (R20b, R22) ─┤
           └─ R26 (desde R5, paralela a las fases 3 a 5)                                    │
                                     R25 (tras R21) ────────────────────────────────────────┤
                                  [P3] ─ R27 (R22, R22b, R24, R25, R26) ─ R28 (R20b, R21, R23) ─┬─ R29 ─ R29b
                                                                                                └─ R30 ─ R31
```
- R8, R9 y R10 se pueden hacer en paralelo, pero tocan los mismos handlers de iam. Conviene ir en serie para no tener conflictos.
- R18 (eliminar `ListPublicCommunities`, D3) no depende de nada más que R1 y va antes de R7.
- R7 y R16 son paralelas a toda la rama iam.
- `[P1]`, `[P2]` y `[P3]` son los puntos de replanificación de §5.9. No se arranca la rebanada siguiente sin cerrar el punto.
- R13, R14 y R15 son paralelas entre sí después de R11. R14 necesita R2, R9 y R10.
- *(P1)* R12b va después de R12, R13 y R14, y antes de R19. No bloquea a R15 ni a R16.
- R23 y R24 son paralelas. R26 se puede hacer desde R5 en adelante.
- *(P2)* R20a va primero en la fase 4: crea `infrastructure/uuid`, que usan R20b (por `AccountFactory::create`) y R22. R20b y R22 son paralelas después de R20a. Si R21 se parte, R21a va antes que R21b, y R25 depende de R21a.
- ~~*(P2)* S4 va después de R26 y antes de P3. P3 ahora espera también a S4.~~ *(S4)* S4 va después de R20a y antes de R20b, y P3 ya no la espera. R22b (propuesta en S4) va después de R20b y de R22, y antes de R27.
- *(P2)* R29 y R30 son paralelas después de R28, como antes. R29b (D9) va después de R29, y R31 (D13), después de R30.
- *(S3)* R13b (D11 a) no bloquea a nadie; cierra antes de R17 para que R17 cierre `hex_4_use_case_signatures_…`. R16b (D12 b) va después de R16 y antes de R20.

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

**P1 (2026-11-23): sin cambios.** Las 33 entradas (51 violaciones) que quedan tienen, cada una, un ID del inventario y una rebanada: `hex_1` → HEX-1.1 (R16); `hex_2_core` de communities → HEX-2.5 (R27); `hex_2_core` de iam → HEX-2.1 (`chrono` en R21, el resto en R28); `hex_2_core` y `hex_8_kernel_depends` de `shared` → HEX-8.1 (`chrono` en R21, `serde` en R25); `hex_2_frameworks` → HEX-2.5 (R27) y HEX-2.1 (R28); `hex_4_handlers` → HEX-4.1 (R11, R13, R14, R15, R17); `hex_6` → HEX-6.1 (R27, R28); `hex_8_kernel_traits` de `shared/src/error.rs` → HEX-8.3 (R19). Ninguna incumple. Los hallazgos nuevos de P1 no tienen entrada: ninguna regla de `tools/architecture` los cubre (VER-1.3 a VER-1.6).

**S2 (`v1.0.0-rc.2`, 2026-11-23): sin cambios.** La línea base vuelta a llenar tiene las mismas 47 entradas, con las mismas claves y conteos (§5.10). Ninguna rebanada cambia de alcance: los cierres, los −n por rebanada y la columna "Queda en cero en" siguen como los dejó S1.

**S3 (`v1.0.0-rc.3`, 2026-11-25): +11 entradas (31 → 42; 49 → 72 violaciones).** Se mantienen las 31 que quedaban después de R13, con la misma clave y la misma `n`. Las 11 nuevas vienen de tres reglas nuevas y de una que se amplió, y cada una tiene ID y rebanada (§5.10): `hex_1_driving_…` → HEX-1.7 (R19) y HEX-1.8 (R17 o R19); `hex_4_use_case_signatures_…` → ERR-2.1 (R14, R15, R17), HEX-4.2 (R17) y HEX-4.5 (R13b); `hex_4_use_case_contracts_…` → HEX-4.2 (R17); `hex_8_model_markers_…` → HEX-8.9, HEX-8.10, HEX-8.11 y HEX-8.12 (R21).

**S4 (`v1.0.0-rc.4`, 2026-11-26): +6 entradas (29 → 35; 43 → 49 violaciones).** Se mantienen las 29 que quedaban después de R20a, con la misma clave y la misma `n`. Las 6 nuevas son de una sola regla nueva, `hex_2_core_imports_only_std_and_the_kernel` (1.0.0-rc.3.1, VER-1.8), una por archivo del núcleo que nombra un crate externo, y cada una tiene ID y rebanada (§5.10): HEX-2.3 (R20b), HEX-2.4 y HEX-8.2 (R21), HEX-2.6 y HEX-2.7 (R22), HEX-8.8 (R25). Las demás reglas de rc.4 pasan en verde sin entradas: `err_4_driving_folds_are_exhaustive` (1.0.0-rc.3.2), `hex_1_driving_adapters_see_only_use_cases` con otros contextos legacy (1.0.0-rc.3.3) y el `Response` vacío de `hex_4_use_case_signatures_…` (1.0.0-rc.3.4).

| Test | Entradas hoy (después de S4) | Justo antes de S1 | Queda en cero en |
|---|---|---|---|
| `err_1_domain_errors_do_not_implement_std_error` | 0 | 0 | R3: en cero (eb30039) |
| `err_1_application_errors_do_not_implement_std_error` | 0 | 0 | R3: en cero (eb30039) |
| `hex_1_dependencies_point_inward` | 0 | 1 | R16: en cero (f6bc1fd) |
| `hex_1_driving_adapters_see_only_use_cases` | 0 | — (nuevo en S3) | R19: en cero (R17 −1, R19 −1 el de iam; 11f8c92). R15: la `n` del de iam baja de 10 a 9 (6b09abd); R17: −1, el mapper de communities (838b923) |
| `hex_5_outbound_ports_are_named_by_role` | 0 | 4 | R10a: en cero (R7 a R10); R7: −1 (692bc7c); R8: −1 (ebd0819); R9: −1 (73f9c44); R10a: −1 (517e4cd) |
| `hex_5_adapters_are_named_by_technology_and_concept` | 0 | 8 | R10a: en cero (R7 a R10); R7: −2 (9eacddc, 6b808a2); R8: −2 (de6c308, 9a90d8a); R9: −2 (c04f69d, 17ba68e); R10a: −2 (ce6d64b, c9db5fa) |
| `hex_4_use_case_implementations_are_handlers` | 0 | 6 | R17: en cero (R18, R11, R13 a R15, R17); R18: −1 (a603969); R11: −1 (1482280); R13: −1 (2d16725); R14: −1 (09fb976); R15: −1 (cf2b905); R17: −1 (4a04532) |
| `hex_8_kernel_traits_are_model_markers_or_listed_capabilities` | 0 | 2 | R19: en cero (R4, R10, R19); R4: −2 (80c00e4); R10b: −1 (3c1a6bb); R19: −1 (11f8c92) |
| `hex_8_kernel_depends_only_on_std` | 2 | 1 | R25 (R21 −1 `chrono`, R25 −1 `serde`) |
| `hex_2_core_depends_only_on_std_and_the_kernel` | 12 (22) | 3 | R28 *(P2)*: R20a −1 (`uuid` de iam; d461bbc), R21 −2 (`chrono`), R22 −2 (`uuid` y `http` de communities), R25 −1 (`serde` de `shared`), R27 −2 (`axum`, `serde`), R28 −5 (`argon2`, `axum`, `jsonwebtoken`, `rand`, `serde`). Antes de P2: R21 −2, R25 −1, R27 −4, R28 −6 |
| `hex_2_server_frameworks_only_at_the_edge` | 2 (6) | 2 | R28 (R27, R28) |
| `hex_6_transport_types_carry_the_transport_name` | 9 | 9 | R28 (R27, R28) |
| `hex_4_use_case_contracts_do_not_expose_domain_types` | 0 | 0 | R17: en cero (58958bc, `VerifiedIdentity` en `create_community_use_case.rs`). Desde S1 actúa también en legacy; desde S3, `shared::domain` cuenta como dominio |
| `hex_4_use_case_signatures_are_their_own_contract` | 0 | — (nuevo en S3) | R17: en cero (R13b −1, R14 −1, R15 −1, R17 −1); R13b: −1 (d37fe10); R14: −1 (7932c45); R15: −1 (2c37b03); R17: −1 (58958bc) |
| `hex_8_model_markers_have_no_items` | 4 | — (nuevo en S3) | R21 |
| `hex_2_core_imports_only_std_and_the_kernel` | 6 | — (nuevo en S4) | la última de R20b (−1, `code_validation.rs`), R21 (−2, `account_event.rs` y `shared/src/domain/domain_event.rs`), R22 (−2, `community_id.rs` y `owner_id.rs`) y R25 (−1, `api_error_response.rs`). Con el orden del plan, R25 |

**P2 (2026-11-26, sobre `0eaab25`): sin cambios en la línea base; cambia el reparto.** `cargo test -p architecture` pasa en verde, también con `ARCH_REQUIRE_MATERIAL=1`, así que las 30 entradas siguen violando con su `n` exacta (el trinquete falla si una deja de violar o viola menos). Coinciden con la columna "Entradas hoy".

**Conciliación (P2): las 30 entradas con su ID y su rebanada.**

| # | Entrada | ID | Rebanada (P2) | Antes de P2 |
|---|---|---|---|---|
| 1 | `hex_2_core` `contexts/communities/Cargo.toml#dependencies.axum` 2 | HEX-2.5 | R27 | R27 |
| 2 | `hex_2_core` `contexts/communities/Cargo.toml#dependencies.http` 2 | HEX-2.5, RS-1.5 | **R22** | R27 |
| 3 | `hex_2_core` `contexts/communities/Cargo.toml#dependencies.serde` 2 | HEX-2.5 | R27 | R27 |
| 4 | `hex_2_core` `contexts/communities/Cargo.toml#dependencies.uuid` 2 | HEX-2.5, HEX-2.6, HEX-2.7 | **R22** | R27 |
| 5 | `hex_2_core` `contexts/iam/Cargo.toml#dependencies.argon2` 2 | HEX-2.1 | R28 | R28 |
| 6 | `hex_2_core` `contexts/iam/Cargo.toml#dependencies.axum` 2 | HEX-2.1 | R28 | R28 |
| 7 | `hex_2_core` `contexts/iam/Cargo.toml#dependencies.chrono` 2 | HEX-2.1, HEX-2.4 | R21 | R21 |
| 8 | `hex_2_core` `contexts/iam/Cargo.toml#dependencies.jsonwebtoken` 2 | HEX-2.1 | R28 | R28 |
| 9 | `hex_2_core` `contexts/iam/Cargo.toml#dependencies.rand` 2 | HEX-2.1, HEX-2.3 | R28 (R20b saca `rand` del dominio; Argon2 lo sigue usando) | R28 |
| 10 | `hex_2_core` `contexts/iam/Cargo.toml#dependencies.serde` 2 | HEX-2.1 | R28 | R28 |
| 11 | `hex_2_core` `contexts/iam/Cargo.toml#dependencies.uuid` 2 | HEX-2.1, HEX-2.2 | **R20a**: borrada (d461bbc) | R28 |
| 12 | `hex_2_core` `shared/Cargo.toml#dependencies.chrono` | HEX-8.1, HEX-8.2 | R21 | R21 |
| 13 | `hex_2_core` `shared/Cargo.toml#dependencies.serde` | HEX-8.1, HEX-8.8 | R25 | R25 |
| 14 | `hex_2_server_frameworks` `contexts/communities/Cargo.toml#dependencies.axum` 3 | HEX-2.5, HEX-1.9 | R27 | R27 |
| 15 | `hex_2_server_frameworks` `contexts/iam/Cargo.toml#dependencies.axum` 3 | HEX-2.1, HEX-1.9 | R28 | R28 |
| 16–17 | `hex_6` `contexts/communities/src/infrastructure/http/{requests/create,responses/created}.rs` | HEX-6.1 | R27 | R27 |
| 18–24 | `hex_6` `contexts/iam/src/infrastructure/http/{requests/{identify,sign_in,sign_up,verify},responses/{identified,signed_in,signed_up}}.rs` | HEX-6.1 | R28 | R28 |
| 25 | `hex_8_kernel_depends` `shared/Cargo.toml#dependencies.chrono` | HEX-8.1 | R21 | R21 |
| 26 | `hex_8_kernel_depends` `shared/Cargo.toml#dependencies.serde` | HEX-8.1 | R25 | R25 |
| 27 | `hex_8_model_markers` `shared/src/domain/aggregate_root.rs` | HEX-8.9 | R21 | R21 |
| 28 | `hex_8_model_markers` `shared/src/domain/domain_event.rs` | HEX-8.11 (y HEX-8.2) | R21 | R21 |
| 29 | `hex_8_model_markers` `shared/src/domain/entity.rs` | HEX-8.12 | R21 | R21 |
| 30 | `hex_8_model_markers` `shared/src/domain/value_object.rs` | HEX-8.10 | R21 | R21 |
| 31 | *(S4)* `hex_2_core_imports` `contexts/communities/src/domain/model/community/community_id.rs` (`uuid`) | HEX-2.6 | R22 | — |
| 32 | *(S4)* `hex_2_core_imports` `contexts/communities/src/domain/model/community/owner_id.rs` (`uuid`) | HEX-2.7 | R22 | — |
| 33 | *(S4)* `hex_2_core_imports` `contexts/iam/src/domain/model/account/code_validation.rs` (`rand`) | HEX-2.3 | R20b | — |
| 34 | *(S4)* `hex_2_core_imports` `contexts/iam/src/domain/model/account/events/account_event.rs` (`chrono`) | HEX-2.4 | R21 | — |
| 35 | *(S4)* `hex_2_core_imports` `shared/src/domain/domain_event.rs` (`chrono`) | HEX-8.2 | R21 | — |
| 36 | *(S4)* `hex_2_core_imports` `shared/src/infrastructure/http/api_error_response.rs` (`serde`) | HEX-8.8 | R25 | — |

**Entradas que no cumplían (P2).** Las 30 tenían un ID y una rebanada en el plan, así que ninguna incumplía de forma. Tres tenían una rebanada que no las resolvía, o que las resolvía tarde:
- **#4, `uuid` de communities (R27):** R22 solo nombraba `CommunityId`. Con `OwnerId` todavía sobre `uuid` (HEX-2.7, sin ID hasta P2), R27 habría creado `communities-domain/Cargo.toml` con `uuid`, una entrada nueva que la línea base no admite. **Pasa a R22.**
- **#2, `http` de communities (R27):** no tiene ningún uso desde R17 (RS-1.5). No es un error del plan, pero se puede cerrar antes. **Pasa a R22.**
- **#11, `uuid` de iam (R28):** desde R20a no tiene uso; por §5.1 ("Una dependencia, una entrada"), la rebanada que lo quita del manifiesto borra la entrada. **Pasa a R20a.**

**Precondiciones de R27 y R28 (P2).** Lo que quedaría de cada contexto al llegar, con la secuencia de P2:

- **R27 (`communities`), después de R22, R22b, R24, R25 y R26** *(S4: S4 ya hecha)*. En `contexts/communities/src` quedan solo las 2 entradas `hex_6` (#16 y #17, `requests/create.rs` y `responses/created.rs`), que R27 renombra a `CreateCommunityHttpRequest` y `CreateCommunityHttpResponse` al moverlas a `communities-api`. Del manifiesto quedan `axum` y `serde` (#1 y #3) y `axum` en `hex_2_frameworks` (#14). Desaparecen con el manifiesto, y en la estructura nueva solo los declara `communities-api`, donde las dos reglas lo admiten (verificado en P2: `axum`, `http` y `serde` solo aparecen en `infrastructure/http`, y `application` no usa ningún crate externo). **Cumple, con la condición de que R22 saque `uuid` (#4) y `http` (#2).** Sin R22, `communities-domain` nacería con `uuid` por `OwnerId`.
- **R28 (`iam`), después de R20a, R20b, R21, R22b, R23, R25 y R26** *(S4: S4 ya hecha)*. En `contexts/iam/src` quedan solo los 7 `hex_6` (#18 a #24), que R28 renombra al moverlos a `iam-api`. Del manifiesto quedan `argon2`, `axum`, `jsonwebtoken`, `rand` y `serde` (#5, #6, #8, #9 y #10) y `axum` en `hex_2_frameworks` (#15). En la estructura nueva, `argon2`, `jsonwebtoken` y `rand` solo los declara `iam-infrastructure` (`argon2_password_hashing.rs` usa `rand::rngs::OsRng`, y desde R20b, `RandVerificationCodeAdapter`). `axum` lo declara `iam-api`, y `serde`, `iam-api` (los DTOs) e `iam-infrastructure` (los claims de JWT). **Cumple, con tres condiciones:** que R20a saque `uuid` (#11); que R20b saque `rand` del dominio (`code_validation.rs`), porque si no, `iam-domain` nacería con `rand`, y que R21 saque `chrono` (#7). Además, `tokio` (`spawn_blocking`, de R23) entra recién aquí, en `iam-infrastructure`.
- **`shared/` al llegar a R27:** quedan 0 entradas. R21 cierra #12, #25 y #27 a #30, y R25 cierra #13 y #26. Es la precondición de R29.

**Suma por rebanada desde P2 (30):** R20a −1, R21 −7, R22 −2, R25 −2, R27 −5 y R28 −13.

**Suma por rebanada desde S4 (35, después de R20a):** R20b −1, R21 −9, R22 −4, R25 −3, R27 −5 y R28 −13. La fila #11 ya se borró en R20a; las #31 a #36 son de S4. Las precondiciones de R27 y R28 no cambian: R22 deja `communities-domain` sin `uuid` también por archivo, y R20b y R21 dejan `iam-domain` sin `rand` ni `chrono`.

En total quedan 30 entradas (45 violaciones) después de R19 (32 y 55 después de R17; 36 y 61 después de R16 y de R16b, que no cambia la línea base; 37 y 65 después de R15; 39 y 68 después de R14 y de R12b, que no cambia la línea base; 41 y 70 después de R13b; 42 y 72 después de S3; 31 y 49 después de R13; 32 y 50 después de R11 y de R12, que no cambia la línea base; 33 y 51 después de R10b; 34 y 52 después de R10a; 37 y 55 después de R9; 40 y 58 después de R8; 43 y 61 después de R7; 46 y 64 después de R18; 47 y 65 en S1), que desaparecen entre R15 y R28. Después de R29, `baseline.txt` queda solo con la cabecera.

Suma por rebanada desde S1: R7 −3, R8 −3, R9 −3, R10 −4 (R10a −3, R10b −1), R11 −1, R13 −1, R14 −1, R15 −1, R16 −1, R17 −1, R18 −1, R19 −1, **R21 −3**, R25 −2, **R27 −7** y **R28 −14**: 47. *(S3)* Con las 11 nuevas: R13b −1, R14 −2, R15 −2, R17 −4 (−3 si el mapper de communities sigue hasta R19), R19 −2 (−3) y R21 −7, y suman 58. Las 36 de antes de S1 se repartían entre las mismas rebanadas, con R25 −2, R27 −4 y R28 −9, y sin R21.

**Conciliación (R1).** El encargo hablaba de 56 entradas, pero `tools/architecture/baseline.txt` tiene 55 (63 líneas, de las que 8 son comentario). La columna "Entradas hoy" coincide test por test con `grep -v '^#' baseline.txt | cut -d' ' -f1 | sort | uniq -c`. No falta ninguna entrada: se corrige el conteo, no se asigna ninguna. La suma por rebanada también da 55: R3 −17, R4 −2, R7 −3, R8 −3, R9 −3, R10 −4, R11 −1, R13 −1, R14 −1, R15 −1, R16 −1, R17 −1, R18 −1, R19 −1, R25 −2, R27 −4 y R28 −9.

Los hallazgos que no son de código no tienen rebanada:
- ERR-4.1: rotar el secreto, una acción del usuario.
- TDD-1.5, TDD-1.6 y TDD-1.7: descartados por D8.
- ~~VER-1.3 a VER-1.6: son tests de `tools/architecture`, que solo cambia en una sincronización. Se proponen al harness (§6).~~ Resueltos en el harness y traídos en S3 (`v1.0.0-rc.3`): las violaciones que dejaban sin trinquete ahora tienen entrada.

### 5.7 Decisiones del usuario

Decididas (2026-11-20, en el PR de R1):

- **D3. `ListPublicCommunities` (R18, R7): decidido, eliminar.** Se borran el caso, su puerto de entrada, `get_public_list`, `find_by_id` y `AuthContext`. No se agrega ninguna ruta. R18 se adelanta y va antes de R7.
- **D6. Tiempo de respuesta del sign-in (R2): decidido, sí, dentro de R2.** Cuando el usuario no existe, se verifica Argon2 contra un hash señuelo generado al arrancar, así el 401 no permite enumerar usuarios por tiempo. Test rojo: `verifies_against_the_decoy_hash_when_account_does_not_exist`.
- **D7. Cuerpo interino del 500 por `Fault` (R5 a R30): decidido, el cuerpo que propone el plan.** `{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}`. Rige hasta que R30 lo reemplaza por Problem Details (D5).

Decididas en P1 (2026-11-23):

- **D1. Verificar dos veces (R13): decidido, sigue 400.** Verificar una cuenta ya verificada responde 400 `IAM_ACCOUNT_INVALID_VERIFICATION`. R13 no cambia el contrato. Pasar a 409 se revisa junto con R30.
- **D2. Identidad del dueño en communities (R16): decidido, opción a).** `OwnerId` valida el formato UUID y rechaza el nil, y conserva los códigos `IAM_INVALID_ACCOUNT_ID(_FORMAT)`. R16 no cambia el contrato. Se descartan b) (códigos propios) y c) (`OwnerId` opaco).
- **D4. Eventos (R21, R22): decidido, se retira el andamiaje muerto.** `AccountEvent`, `take_events` (que nunca se escribió: hoy son `record_event`/`pull_events` con `todo!()`) y `MembershipPolicy`, con `MembershipPolicyChanged`, se borran porque no tienen comportamiento ni uso (HEX-7.4, HEX-7.9). Se implementarán cuando algo consuma los eventos. Efecto: R21 sin `create_records_registered` y sin `Clock`; R22 borra en lugar de mover (HEX-7.8).
- **D8. Hallazgos históricos (TDD-1.5 a TDD-1.7): decidido.** Quedan `descartado (histórico; TDD-1 rige desde R1)`.

Abiertas: ninguna desde P2. D14 y D15 se plantearon en S4 y se decidieron después (abajo).

- ~~**D5. Forma de Problem Details (R30).**~~ Decidida en P2 (abajo).
- ~~**D9. Mover `app/` a `apps/server`.**~~ Decidida en P2 (abajo).
- ~~**D10. `identify` no normaliza el email.**~~ Decidida en R12b (abajo).
- ~~**D13. El username distingue mayúsculas.**~~ Decidida en P2 (abajo).

Decididas en R12b (2026-11-25):

- **D10. Normalización de los identificadores (R12b): decidido, `identify` también normaliza.** La API trata igual las mayúsculas y los espacios en todos los endpoints: verify, authenticate e identify buscan la cuenta con el valor normalizado, construyendo `Email` o `Username` como el alta (R12). El email, sin espacios alrededor y en minúsculas; el username, sin espacios alrededor (`Username` no cambia las mayúsculas, así que el username sigue distinguiéndolas, como en el alta). Un identificador que no se puede construir (formato inválido) responde igual que una cuenta inexistente: `NotFound` en verify e identify, y `LoginFailed` en authenticate, que también verifica contra el señuelo (D6). No hay 400 nuevos. `identify_with_different_email_casing_returns_404` pasa a 200 (§5.5). *Antes:* D10 dejaba `identify` sin normalizar, fijado por ese test.

Decididas en S3 (2026-11-25):

- **D11. `VerifyAccount` sin datos (HEX-4.5, R13b): decidido, opción a).** `VerifyAccountResponse` vacío en `commands/verify_account/response.rs`, en la rebanada R13b. El `fold` de `infrastructure/http/verify_account.rs` no lo serializa y sigue respondiendo 200 con cuerpo `null`: sin cambio de contrato. Se descarta b) (esperar a que el harness acepte `()`); la fricción de §6 se reporta igual. *Planteo:* R13 respondió `()`, y `hex_4_use_case_signatures_are_their_own_contract` de `v1.0.0-rc.3` exige un `VerifyAccountResponse`.
- **D12. Baterías async antes de R23/R24: decidido, opción b).** Una rebanada aparte, R16b, da a las 4 baterías la forma final de la receta "Puerto todavía síncrono" con los puertos todavía síncronos. Va después de R16 (para no tocar dos veces la batería de `CommunityRepository`) y antes de R20 (que crea una batería nueva con esa forma). R23 y R24 solo agregan los `.await` y borran el `#![expect(clippy::unused_async)]`. Se descarta a) (R23 y R24 reescriben las baterías junto con los puertos).

Decididas en P2 (2026-11-26):

- **D5. Forma de Problem Details (R30): decidido.**
  - `application/problem+json` (RFC 9457), con `code` como miembro de extensión y **los mismos valores de hoy**, byte a byte: los de `infrastructure/http/codes.rs` e `INTERNAL_ERROR` (D7).
  - `title` genérico por status, como el `Problem` de `edge/http` de la plantilla: `Invalid request` (400), `Unauthorized` (401), `Not found` (404), `Conflict` (409) y `Unexpected error` (500).
  - `detail` lleva el mensaje de cliente de hoy (el `message` de `{code,message}`), sin cambiarlo.
  - Los 401 llevan cuerpo Problem. Los del middleware, que hoy van con cuerpo vacío, pasan a `Problem::unauthorized()`. Hoy no tienen código, así que van sin `code`, salvo que R30 proponga uno y el usuario lo acepte. El 401 `IAM_LOGIN_FAILED` de authenticate ya tiene cuerpo y conserva su `code`.
  - Efecto: R30 modifica `assert_error` (`app/tests/common/mod.rs`) para que lea `code` y `detail` del cuerpo Problem y afirme el `content-type`, y reemplaza las aserciones `raw_body == ""` de los 401 por el cuerpo Problem. Son cambios de contrato (§5.1, §5.5), no adaptaciones.
  - El 500 conserva `code: INTERNAL_ERROR` y el mensaje de D7 en `detail`. El `unexpected()` de la plantilla (rc.3) responde `Problem::internal()` sin `code` ni `detail`, así que R26 o R30 tienen que agregarlos (ver R30). *(S4, pendiente del usuario)* Con rc.4, `unexpected(&f, r).with_code(codes::INTERNAL_ERROR)` pone el `code`, pero `UnexpectedProblem` no admite `detail`: el mensaje de D7 no puede ir en el 500 sin cambiar `edge/http`. Ver R30. *(D14)* Decidido: el 500 va sin `detail`.
  - *Planteo:* si se conservaba `code`, qué iba en `detail` y si los 401 llevaban cuerpo.
- **D9. `app/` pasa a `apps/server`, como en la plantilla: decidido.** Se hace en una rebanada XS propia, **R29b**, después de R29. Fuera del inventario: ningún ID lo exige. `tools/architecture` clasifica `app/` y `apps/*` como Bootstrap, así que la línea base no cambia.
- **D13. El username distingue mayúsculas (registrada en R15): decidido, opción b).** La unicidad del username no distingue mayúsculas, y se conserva la forma original para mostrarla. El alta rechaza `Ana` si existe `ana` (409 `IAM_ACCOUNT_USERNAME_ALREADY_EXISTS`); verify no cambia (busca por email); authenticate e identify encuentran la cuenta con cualquier combinación de mayúsculas, e identify responde el username tal como se registró. Va en una rebanada propia, **R31**, después de R30, con cambio de contrato (§5.5). Se descartan a) (dejarlo así) y c) (normalizar a minúsculas, que cambiaría también lo que responde identify).
  - *Planteo:* `Username` recorta los espacios, pero no cambia las mayúsculas (D10), así que `Ana` y `ana` eran dos cuentas distintas, y authenticate e identify con `ANA` no encontraban a `ana`.

Decididas después de S4 (2026-11-27):

- **D14. El 500 de R30 sin `detail`: decidido, se acepta.** R30 usa `unexpected(&fault, reporter).with_code(codes::INTERNAL_ERROR)` de `edge/http` (rc.4) tal cual, en lugar de los `unexpected.rs` de cada contexto. El 500 queda con `code: INTERNAL_ERROR` y sin el mensaje de D7 en `detail`. Ajusta D5 en ese punto, y es un cambio de contrato más de R30 (§5.5). Se descarta pedir al harness un `detail` fijo de cliente para `UnexpectedProblem`.
  - *Planteo (S4):* `UnexpectedProblem` no admite `with_detail`, a propósito, y D5 pedía el mensaje de D7 en `detail`.
- **D15. R22b: confirmada.** `Field` cerrado en las hojas (`Field::Owner(OwnerRejection)`, ERR-4.13) va en su propia rebanada, después de R20b y R22 y antes de R27, como la propuso S4. R30 recibe los `fold` con `Field` cerrado.

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
| 2026-11-26 | S4 | Comando del plugin: `/craft-backend:adoption` pide leer entero `docs/harness-adoption.md`, y el documento (325 KB) pasa el límite de lectura de la herramienta. | Hubo que leerlo por partes, con riesgo de saltear una sección. Este PR lo parte en un documento vivo, `archive.md` y `costs.md`. | sin reportar |
| 2026-11-26 | S4 | Comando del plugin: dice que el PR se abre solo si el usuario lo pidió, y el ritual del proyecto (§5.1) pide un PR contra `main` en cada rebanada, aunque la invocación no lo pida. | Rige §5.1, pero la contradicción se resuelve a mano en cada invocación. La fricción de 2026-11-23 (el PR solo si se pide) figura corregida en rc.2, y en S4 la instrucción sigue. | sin reportar |
| 2026-11-26 | S4 | Comando del plugin: el paso de sincronización no corre `ARCH_REQUIRE_MATERIAL=1`: queda fuera de los pasos de `sincronizar`, aunque una plantilla nueva puede dejar vacío un conjunto o hacer innecesaria una exención. | S4 lo corrió a mano. Sin eso, una exención de más o un conjunto vacío nuevo se habrían visto recién en CI. | sin reportar |
| 2026-11-26 | S4 | Comando del plugin: la comparación de la plantilla con lo copiado solo cubre `tools/architecture`. Lo que el proyecto copió de otras partes de la plantilla (`infrastructure/uuid`, `kernel/src/model.rs`, `edge/http`) no se compara. | S4 lo comparó a mano (§5.3 del archivo, S4): `infrastructure/uuid` sin cambios, `model.rs` admite `Entity` e `Identifier`, `edge/http` trae `UnexpectedProblem`. Sin eso, R21, R26 y R30 habrían seguido con la plantilla de rc.3. | sin reportar |

### Sin reportar todavía (P2, 2026-11-26)

Se reportaban a ai-harness antes de S4, para que el release que S4 sincroniza pudiera traer la corrección. *(S4)* Las nueve primeras y la de `unexpected()` sin `code` quedan corregidas en craft-backend 1.0.0-rc.4 (tabla de arriba). Sigue sin reportar la 10, y se suman las cuatro del comando del plugin de S4 (11 a 14). Las corregidas están en `harness-adoption/archive.md`.

10. **Borrar una exención de material (R20a).** La cabecera de `material-exempt.txt` y la skill dicen que el agente nunca escribe el archivo, pero el test pide "borrar de material-exempt.txt" y el hook `pre-edit` deja borrar. Propuesta: que la skill diga si el borrado que pide el test es del agente o del usuario, y que el hook lo aplique igual. *(2026-11-27)* De fondo, la asimetría: agregar una exención requiere una persona, y quitarla no. Si el borrado es del agente, que la skill lo diga; si no, que el hook lo bloquee como bloquea el agregado.
11. **El documento de seguimiento pasa el límite de lectura (S4).** El comando pide leer entero `docs/harness-adoption.md`, y con 325 KB no entra en una lectura. Propuesta: que el comando lea por secciones, o que la skill sugiera separar lo vivo de lo cerrado.
12. **"PR solo si el usuario lo pidió" contra el ritual del proyecto (S4).** El comando lo dice en cada invocación, y el ritual del proyecto (§5.1) pide un PR por rebanada. Propuesta: que la instrucción ceda ante las convenciones del proyecto, si las hay.
13. **`ARCH_REQUIRE_MATERIAL` fuera del paso de sincronización (S4).** `sincronizar` no corre `ARCH_REQUIRE_MATERIAL=1`. Propuesta: sumarlo a los pasos, después del llenado de la línea base.
14. **La comparación fuera de `tools/architecture` (S4).** `sincronizar` solo compara `tools/architecture` con la plantilla. Propuesta: comparar también los crates de la plantilla que el proyecto copió (`kernel`, `edge/http`, `infrastructure/*`) y listar sus cambios entre versiones.
