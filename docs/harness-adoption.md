# Adopción del harness craft-backend

Este archivo guarda el estado entre sesiones de la adopción de los principios craft-backend (HEX, ERR, TDD, VER, RS) en este repositorio.

| Dato | Valor |
|---|---|
| Fecha de la auditoría | 2026-11-19 |
| Plugin | `craft-backend@ai-harness` 0.12.0 |
| Commit auditado | `d0b0eeb` (rama `ci/sync-development`) |
| Revisor | `craft-backend:architecture-reviewer`, auditoría completa en solo lectura |
| Hallazgos | 93: 25 de severidad alta, 48 media, 20 baja |

## Cómo mantener este archivo

- Al resolver un hallazgo, cambiar su **Estado** de `abierto` a `resuelto (<commit>)`. Si se descarta, poner `descartado` y el motivo.
- Los IDs de hallazgo (`HEX-1.3`, `ERR-4.2`…) son estables: no se renumeran. Un hallazgo nuevo recibe el siguiente número libre de su regla.
- Después de cada slice de refactor, volver a auditar con architecture-reviewer y actualizar la cabecera (fecha, commit, conteos).
- Tests de caracterización en `app/tests/` (PR #18): son la red de seguridad del refactor y tienen que seguir pasando.

## Prioridades inmediatas

1. **ERR-4.1:** `.env` con `JWT_SECRET` está versionado. Sacarlo del índice, agregarlo a `.gitignore` y **rotar el secreto**, porque ya está en el historial.
2. **ERR-4.2:** `dbg!(&account)` en `register_account.rs:55` imprime el email y el hash de la contraseña en cada registro.
3. **ERR-1.1 / RS-2.1:** `SystemError`, con su `From` genérico, mezcla los errores de negocio con los técnicos. Hay que reemplazarlo por `{Verb}{Noun}Error` por caso de uso y `Fault`/`Failure<E>`.
4. **VER-1 / RS-4:** `tools/architecture` ya existe, con trinquete sobre `tools/architecture/baseline.txt` (ad558fc). Faltan `[workspace.lints]` (RS-2.5) y un CI que corra fmt, clippy y test (VER-1.2).

## 1. Resumen

| Regla | Alta | Media | Baja | Total |
|---|---|---|---|---|
| HEX-1 Dependencias hacia adentro | 6 | 3 | 0 | 9 |
| HEX-2 Núcleo sin framework | 3 | 3 | 0 | 6 |
| HEX-4 Puertos entrantes | 0 | 2 | 2 | 4 |
| HEX-5 Puertos por rol | 0 | 2 | 0 | 2 |
| HEX-6 Vocabulario | 0 | 0 | 2 | 2 |
| HEX-7 El dominio decide | 1 | 5 | 3 | 9 |
| HEX-8 Kernel delgado | 3 | 4 | 2 | 9 |
| VER-1 Verificación automática | 2 | 0 | 0 | 2 |
| ERR-1 Dos árboles | 2 | 4 | 1 | 7 |
| ERR-2 Contrato por caso de uso | 2 | 5 | 3 | 10 |
| ERR-3 Semántica de adaptadores | 0 | 3 | 0 | 3 |
| ERR-4 Transporte en el borde | 2 | 5 | 1 | 8 |
| TDD-1 Test primero | 0 | 4 | 3 | 7 |
| TDD-2 Tests por capa y dobles | 1 | 2 | 2 | 5 |
| RS-1 Crate por capa | 1 | 1 | 1 | 3 |
| RS-2 `Fault` y `panic!` | 1 | 4 | 0 | 5 |
| RS-3 Puertos async | 0 | 1 | 0 | 1 |
| RS-4 Tests de arquitectura | 1 | 0 | 0 | 1 |
| **Total** | **25** | **48** | **20** | **93** |

Cada fila del inventario cuenta como un hallazgo. Cuando una misma violación se repite igual en varios archivos, la fila lista todas las ubicaciones.

`shared/` se audita como si fuera el kernel, porque hoy cumple ese papel aunque no tenga la forma que pide RS-1. Ver la sección 4.

## 2. Inventario

### HEX (hexagonal-architecture) y VER

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| HEX-1.1 | HEX-1 Dependencias | contexts/communities/Cargo.toml:12 | `communities` declara `iam` como dependencia: un contexto depende de otro. | alta | Quitar `iam`; `communities` modela su propio `OwnerId`. | abierto |
| HEX-1.2 | HEX-1 Dependencias | contexts/communities/src/domain/model/community/community.rs:7 | El dominio de `communities` importa `iam::…::AccountId` y lo guarda como `_owner_id` (línea 12). | alta | Value object `OwnerId` propio del contexto. | abierto |
| HEX-1.3 | HEX-1 Dependencias | contexts/communities/src/application/commands/create_community/create_community.rs:17 | El handler importa `AccountId` de `iam` y valida con él el sujeto del token (línea 46). | alta | Validar con `OwnerId::new` del propio contexto. | abierto |
| HEX-1.4 | HEX-1 Dependencias | contexts/communities/src/infrastructure/http/errors/error_mapper.rs:14 | El borde HTTP de `communities` importa códigos de error del dominio de `iam`. | media | Códigos propios en `communities-api/src/codes.rs`. | abierto |
| HEX-1.5 | HEX-1 Autenticación | shared/src/application/ports/outbound/token_verifier.rs:6 | `TokenVerifier` vive en un `shared` que ven todos los contextos: es la violación típica que nombra la regla. | alta | `VerifyAccessTokenUseCase` en `iam/application/queries` + puerto `AccessTokenService`. | abierto |
| HEX-1.6 | HEX-1 Autenticación | app/src/middleware/auth.rs:10 | El middleware vive en el composition root, no en el `api` de `iam`. Inyecta `VerifiedIdentity` (de `shared::domain`) en vez de un tipo del borde. | alta | `IamApi::protect(routes)` en `iam-api`, que inserta `edge_http::AuthenticatedAccount`. | abierto |
| HEX-1.7 | HEX-1 Dependencias | contexts/iam/src/infrastructure/http/errors/error_mapper.rs:6 | El adaptador driving (HTTP) importa `crate::domain::errors::error_codes`: `api` ve `domain`. | media | `api` solo ve el contrato `{Verb}{Noun}Error` de `application`. | abierto |
| HEX-1.8 | HEX-1 Dependencias | contexts/communities/src/infrastructure/http/errors/error_mapper.rs:5 | Igual que HEX-1.7: el `api` de `communities` importa códigos de `domain`. | media | Igual que HEX-1.7. | abierto |
| HEX-1.9 | HEX-1 Dependencias | contexts/iam/src/infrastructure/mod.rs:1; contexts/communities/src/infrastructure/mod.rs:1 | El adaptador driving (`http`) es un submódulo de `infrastructure`: `api` e `infrastructure` no están separados y pueden importarse entre sí. | alta | Crate `{ctx}-api` separado (RS-1). | abierto |
| HEX-2.1 | HEX-2 Núcleo sin framework | contexts/iam/Cargo.toml:7-13 | `domain` y `application` comparten crate con `uuid`, `argon2`, `rand`, `jsonwebtoken`, `serde`, `chrono` y `axum`, así que todos están disponibles en el núcleo. | alta | Crates `iam-domain` e `iam-application` con solo el kernel. | abierto |
| HEX-2.2 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/account_id.rs:3 | El dominio usa `uuid::Uuid`. | alta | El id llega de `kernel::IdGenerator`; `uuid` solo en infraestructura. | abierto |
| HEX-2.3 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/code_validation.rs:2 | El dominio usa `rand` para generar el código (líneas 29-33). | media | Generar el código con un puerto o capacidad y pasárselo a `Account::create`. | abierto |
| HEX-2.4 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/events/account_event.rs:1 | El dominio usa `chrono`. | media | El instante llega de `Clock`. | abierto |
| HEX-2.5 | HEX-2 Núcleo sin framework | contexts/communities/Cargo.toml:7-9 | `uuid`, `axum` y `serde` están disponibles en `domain` y `application`. | alta | Separar crates por capa. | abierto |
| HEX-2.6 | HEX-2 Núcleo sin framework | contexts/communities/src/domain/model/community/community_id.rs:2 | El dominio usa `uuid`. | media | `IdGenerator` + value object sobre `String`. | abierto |
| HEX-4.1 | HEX-4 Casos de uso | iam/…/authenticate_account.rs:20; register_account.rs:18; verify_account.rs:14; identify_account.rs:16; communities/…/create_community.rs:21; list_public_communities.rs:14 | Las implementaciones no se llaman `{Verb}{Noun}Handler` (`AuthenticateAccount`, `RegisterAccount`…). | media | Renombrar a `AuthenticateAccountHandler` y demás, en `handler.rs`. | abierto |
| HEX-4.2 | HEX-4 Casos de uso | contexts/communities/src/application/ports/inbound/create_community_use_case.rs:10 | `execute` recibe un segundo argumento, `VerifiedIdentity`, del dominio del kernel: el contrato expone un tipo de `domain` fuera del `Command`. | media | Campo `owner_id: String` en `CreateCommunityCommand`. | abierto |
| HEX-4.3 | HEX-4 Casos de uso | contexts/communities/src/application/ports/inbound/list_public_communities_use_case.rs:6 | Le falta `Send + Sync`, recibe un `AuthContext` que no usa (línea 10) y no está cableado. | baja | Firma `execute(query)`, con `Send + Sync`, o eliminarlo. | abierto |
| HEX-4.4 | HEX-4 Casos de uso | contexts/iam/src/application/errors/authenticate_account.rs:6 | El `Error` del caso de uso vive en `errors/`, no junto al handler. | baja | `commands/authenticate_account/error.rs`. | abierto |
| HEX-5.1 | HEX-5 Rol | iam/…/ports/outbound/account_repository.rs:6; password_hasher.rs:1; token_generator.rs:3; communities/…/community_repository.rs:6 | Sufijo `Port`, prohibido. En `PasswordHasher` y `TokenGenerator`, al quitarlo, no queda rol. | media | `AccountRepository`, `CommunityRepository`, `PasswordHashingService`, `AccessTokenService`. | abierto |
| HEX-5.2 | HEX-5 Adaptadores | iam/…/in_memory/account_repository.rs:14; communities/…/in_memory/community_repository.rs:13; iam/…/argon2_password_hasher.rs:8; iam/…/jwt_service.rs:15 | Los adaptadores no siguen `{Tecnología}{Concepto}Adapter`: repiten el rol o terminan en `Service`. | media | `InMemoryAccountAdapter`, `InMemoryCommunityAdapter`, `Argon2PasswordHashingAdapter`, `JwtAccessTokenAdapter`. | abierto |
| HEX-6.1 | HEX-6 Transporte | iam/…/requests/sign_up.rs:5, sign_in.rs:5, verify.rs:5, identify.rs:5; responses/signed_up.rs:5, signed_in.rs:5, identified.rs:5; communities/…/requests/create.rs:5, responses/created.rs:5 | Los DTOs HTTP no son `{Caso}HttpRequest`/`{Caso}HttpResponse`, y `CreateRequest` colisiona con el vocabulario de puerto saliente. | baja | `RegisterAccountHttpRequest`, `CreateCommunityHttpResponse` y demás. | abierto |
| HEX-6.2 | HEX-6 Vocabulario | contexts/communities/src/application/queries/list_public_communities/response.rs:5 | `CommunityResult` usa el sufijo `Result`, reservado para control de flujo. | baja | `PublicCommunity` o `CommunitySummary`. | abierto |
| HEX-7.1 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/register_account/register_account.rs:37 | La unicidad se consulta con el texto crudo (`cmd.email`, y `cmd.username` en la línea 43), no con el value object normalizado. Resultado: se registran duplicados por mayúsculas o espacios, como confirma el test `register_accepts_same_email_with_different_casing`. | alta | Construir `Email`/`Username` antes y buscar por el valor normalizado (como hace `CreateCommunity` con el slug). | abierto |
| HEX-7.2 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:64 | `if !account.can_authenticate()` es un `if` de negocio en el handler. | media | `account.authenticate() -> Result<_, CannotAuthenticate>` en el agregado. | abierto |
| HEX-7.3 | HEX-7 Inmutabilidad | contexts/iam/src/domain/model/account/account.rs:89-134 | El agregado es mutable (`&mut self`): `change_password`, `change_email`, `deactivate`, `confirm_registration`… | media | `fn confirm(&self, …) -> Result<Self, …>`, que devuelve el estado nuevo. | abierto |
| HEX-7.4 | HEX-7 Eventos | contexts/iam/src/domain/model/account/account.rs:32-46 | `record_event`/`pull_events` son `todo!()`; ninguna transición registra eventos (`AccountEvent::Registered` nunca se emite). | media | `create` registra `Registered`, más `take_events`; o quitar el andamiaje. | abierto |
| HEX-7.5 | HEX-7 Crear/reconstituir | contexts/iam/src/domain/model/account/account_factory.rs:14 | `AccountFactory::create`/`reconstitute` (línea 32) reciben `String`. Además, `Account::new` es público (account.rs:49) y permite saltarse las reglas de alta. | media | `Account::create`/`Account::restore` con value objects; `new` privado. | abierto |
| HEX-7.6 | HEX-7 Identificador | contexts/iam/src/domain/model/account/account_factory.rs:21; contexts/communities/src/application/commands/create_community/create_community.rs:47 | El id se genera en el dominio (`AccountId::generate`, `CommunityId::generate`) en vez de recibirse de `IdGenerator`. | media | Inyectar `Arc<dyn IdGenerator>` en el handler y pasar el id. | abierto |
| HEX-7.7 | HEX-7 Mapeo de persistencia | contexts/iam/src/domain/model/account/account_status.rs:74 | `from_str` define en el dominio el formato persistido `"registered:123123"`. | baja | El adaptador traduce de `String` a `AccountStatus`. | abierto |
| HEX-7.8 | HEX-7 Servicio de dominio | contexts/communities/src/domain/services/membership_policy.rs:2 | `MembershipPolicy` es un enum de opciones (value object) archivado en `services/`. | baja | Moverlo a `model/community/`. | abierto |
| HEX-7.9 | HEX-7 Comportamiento | contexts/communities/src/domain/model/community/community.rs:19-56 | Agregado anémico: `membership_policy` siempre es `None` y `MembershipPolicyChanged` (membership_policy_changed.rs:4) no se usa. | baja | Agregar el comportamiento con su evento, o retirar lo muerto. | abierto |
| HEX-8.1 | HEX-8 Kernel sin dependencias | shared/Cargo.toml:7-8 | El kernel depende de `chrono` y `serde`. | alta | Kernel solo `std`. | abierto |
| HEX-8.2 | HEX-8 Marcadores | shared/src/domain/domain_event.rs:5 | El marcador `DomainEvent` exige `DateTime<Utc>` de `chrono`. | media | Sin dependencias; el instante llega de `Clock`. | abierto |
| HEX-8.3 | HEX-8 Kernel | shared/src/error.rs:9 | `ErrorLayout` y `SystemError` (línea 16), un error global por capas, viven en el kernel. | alta | `Fault` + `Failure<E>` (RS-2). | abierto |
| HEX-8.4 | HEX-8 Marcadores de error | shared/src/domain/domain_error.rs:3; shared/src/infrastructure/infrastructure_error.rs:3 | Traits `DomainError` e `InfrastructureError`: Rust no los necesita (y aquí nadie los usa). | media | Eliminarlos. | abierto |
| HEX-8.5 | HEX-8 Sin puertos | shared/src/application/ports/outbound/token_verifier.rs:6 | Un puerto de negocio (`TokenVerifier`) y su `TokenError` (error.rs:2) están en el kernel; no es una capacidad de la lista. | alta | Moverlo a `iam` (ver HEX-1.5). | abierto |
| HEX-8.6 | HEX-8 Sin modelos | shared/src/domain/model/verified_identity.rs:2; shared/src/application/auth_context.rs:2 | Tipos de identidad en el kernel. | media | `AuthenticatedAccount` en `edge/http`. | abierto |
| HEX-8.7 | HEX-8 Kernel | shared/src/application/common_application_error.rs:2 | `CommonApplicationError` está muerto y es un error compartido de aplicación. | baja | Eliminarlo. | abierto |
| HEX-8.8 | HEX-8 Kernel | shared/src/infrastructure/http/api_error_response.rs:4 | Un tipo de transporte HTTP (con `serde`) en el kernel. | media | `edge/http` con `Problem`. | abierto |
| HEX-8.9 | HEX-8 Marcadores | shared/src/domain/aggregate_root.rs:6-8 | `AggregateRoot` no es un marcador: impone `record_event(&mut self)`, `pull_events` y `version`, una API mutable. | baja | Marcador vacío; los eventos se manejan con `take_events(self)` en el agregado. | abierto |
| VER-1.1 | VER-1 Tests de arquitectura | Cargo.toml:2-5 | No existe `tools/architecture`: ninguna regla de dependencias o de nombres tiene test. | alta | Agregar `tools/architecture` (la plantilla de rust-stack). | resuelto (ad558fc) |
| VER-1.2 | VER-1 CI | .github/workflows/sync-development.yml:22-58 | El único workflow sincroniza ramas; no corre `fmt`, `clippy` ni `test`. | alta | Workflow con el CI mínimo de RS-4. | abierto |

### ERR (error-handling)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| ERR-1.1 | ERR-1 Dos árboles | shared/src/error.rs:16 | `SystemError` mezcla los fallos de negocio con los técnicos (`Infrastructure`) en un tipo. El `impl<T: ErrorLayout> From<T>` (línea 32) hace que `?` los absorba implícitamente. | alta | `Failure<{Verb}{Noun}Error>` y `Fault` separados. | abierto |
| ERR-1.2 | ERR-1 Errores de dominio | iam/src/domain/errors/account.rs:27, account_id.rs:20, account_status.rs:20, code_validation.rs:18, email.rs:18, hashed_password.rs:18, password_policy.rs:20, username.rs:18; communities/src/domain/errors/community_error.rs:38, invalid_community_id.rs:14, invalid_community_name.rs:14, invalid_community_slug.rs:14 | Los errores de dominio implementan `std::error::Error`: un `?` los manda al canal técnico. | alta | Solo `Debug` + `Display`. | abierto |
| ERR-1.3 | ERR-1 Errores de application | iam/src/application/errors/account_repository.rs:14, authenticate_account.rs:24, token_generator.rs:14; communities/src/application/errors/community_creation.rs:20, community_repository.rs:14 | Las hojas y los errores de puerto implementan `std::error::Error`. | media | Quitar el `impl`. | abierto |
| ERR-1.4 | ERR-1 Fallo silenciado | contexts/iam/src/domain/model/account/account.rs:127-129 | `let _ = self.transition_status(…).map_err(…)` descarta el `Result` y responde `Ok`, aunque la transición falle. | media | Propagar con `?`, o `expect("invariante: Registered→Active siempre válida")`. | abierto |
| ERR-1.5 | ERR-1 Conversión implícita | contexts/iam/src/domain/model/account/account_factory.rs:39-43 | En `reconstitute`, un dato persistido corrupto se convierte en error de negocio (400) con `?`. | media | En el adaptador, conversión a `Fault` a mano y con contexto. | abierto |
| ERR-1.6 | ERR-1 Sin árbol A | contexts/communities/src/application/ports/inbound/list_public_communities_use_case.rs:11 | El caso de uso no tiene fallos de negocio, pero declara un tipo de error (`SystemError`). | baja | `Result<Response, Fault>`. | abierto |
| ERR-1.7 | ERR-1 Firma del puerto | iam/…/ports/outbound/account_repository.rs:7-9; password_hasher.rs:2-3; communities/…/community_repository.rs:7-10 | `find_*` devuelve `Option` sin `Result`, `hash` devuelve `String` y `verify`, `bool`: un adaptador real no puede reportar una falla técnica y tiene que entrar en pánico. | media | `Result<Option<T>, Fault>`, `Result<String, Fault>`. | abierto |
| ERR-2.1 | ERR-2 Contrato por caso de uso | iam/…/ports/inbound/authenticate_account_use_case.rs:10, identify_account_use_case.rs:7, register_account_use_case.rs:10, verify_account_use_case.rs:5; communities/…/create_community_use_case.rs:11 | Ningún caso de uso tiene su `{Verb}{Noun}Error` cerrado: todos devuelven el global `SystemError`. | alta | `RegisterAccountError { Validation, Conflict }` y demás, junto al handler. | abierto |
| ERR-2.2 | ERR-2 Sin envolver | shared/src/error.rs:17-19 | Los errores se envuelven (`Domain(Box<dyn ErrorLayout>)`), y el error de dominio llega tal cual al contrato (p. ej. `AccountError::EmailAlreadyExists.into()` en register_account.rs:38). | alta | Traducir a una hoja plana (`Conflict { resource }`). | abierto |
| ERR-2.3 | ERR-2 Contrato incompleto | contexts/iam/src/application/errors/authenticate_account.rs:6 | `AuthenticateAccountError` no cubre `NotFound` (viene de `AccountError`) ni la falla de token: hay contratos parciales mezclados. | media | Un enum cerrado con todas las variantes que el caso produce. | abierto |
| ERR-2.4 | ERR-2 Nombre | contexts/communities/src/application/errors/community_creation.rs:6 | `CommunityCreationError` en vez de `CreateCommunityError`, y fuera del directorio del caso. | baja | `commands/create_community/error.rs`. | abierto |
| ERR-2.5 | ERR-2 Variante correcta | contexts/iam/src/domain/model/account/account_status.rs:89 | Un estado ilegible se reporta como `InvalidTransition`. | baja | Dato corrupto → `Fault` en el adaptador. | abierto |
| ERR-2.6 | ERR-2 Tests por variante | contexts/iam/src/application/commands/register_account/register_account.rs:125 (patrón en todos los tests de handler) | Los tests afirman sobre `err.code()` (un string), no sobre la variante. | baja | `matches!(r, Err(Failure::Expected(…)))`. | abierto |
| ERR-2.7 | ERR-2 Test por variante | contexts/iam/src/application/commands/register_account/register_account.rs:65 | Falta el test del handler para contraseña corta (`fails_when_password_is_too_short`). Falla de repositorio sin test de endpoint (solo el unit test del mapper). | media | Agregar ambos. | abierto |
| ERR-2.8 | ERR-2 Test por variante | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:76 | La falla del generador de tokens no tiene test de handler (`fails_when_token_generation_fails`) ni de endpoint. | media | Doble que falle + test. | abierto |
| ERR-2.9 | ERR-2 Test por variante | contexts/iam/src/application/commands/verify_account/verify_account.rs:41 | Faltan el test de handler para código fuera de rango (`fails_when_code_is_out_of_range`) y los de handler y endpoint para la falla del repositorio (`fails_when_repository_fails`). | media | Agregar los tests. | abierto |
| ERR-2.10 | ERR-2 Test por variante | contexts/communities/src/application/commands/create_community/create_community.rs:62 | El owner con formato inválido o nil no tiene test de handler (`fails_when_owner_id_is_invalid`), solo de endpoint. La falla de repositorio no tiene test de endpoint. | media | Agregar los tests. | abierto |
| ERR-3.1 | ERR-3 Respondió vs no pudo | contexts/iam/src/infrastructure/services/jwt_service/jwt_service.rs:85-92 | Un token inválido o vencido se devuelve como `Err(TokenError)`; es un veredicto, no un fallo. | media | `Ok(AccessTokenVerdict::Invalid)`. | abierto |
| ERR-3.2 | ERR-3 Errores para transporte | iam/src/application/errors/account_repository.rs:16-27, token_generator.rs:16-27; communities/src/application/errors/community_repository.rs:16-27 | Los errores de puerto traen un `code()` y un `message()` pensados para el cliente HTTP. | media | Sin texto de usuario; el borde decide. | abierto |
| ERR-3.3 | ERR-3 Falla técnica = árbol B | iam/src/application/errors/account_repository.rs:6, token_generator.rs:6; communities/src/application/errors/community_repository.rs:6 | Fallas técnicas no declarables (persistencia, codificación JWT) modeladas como errores tipados del contrato. | media | `Fault`, propagado con `?`. | abierto |
| ERR-4.1 | ERR-4 Secretos | .env:1 | `JWT_SECRET` está versionado; `.gitignore:1-2` no excluye `.env`. | alta | Sacar `.env` del índice y rotar el secreto; `.env.example` sin valores. | abierto |
| ERR-4.2 | ERR-4 Logging en el núcleo | contexts/iam/src/application/commands/register_account/register_account.rs:55 | `dbg!(&account)` escribe en stderr la cuenta completa (email y hash de la contraseña). | alta | Eliminarlo. | abierto |
| ERR-4.3 | ERR-4 Secretos | app/src/config/jwt.rs:3; app/src/config/app.rs:3 | `#[derive(Debug)]` sobre una configuración que contiene el secreto. | media | Sin `Debug`. | abierto |
| ERR-4.4 | ERR-4 Códigos en el borde | iam/src/domain/errors/error_codes.rs:1-12; iam/src/application/errors/error_codes.rs:1-4; communities/src/domain/errors/error_codes.rs:1-9; communities/src/application/errors/error_codes.rs:1-2 | Los códigos estables y los mensajes de cliente viven en el núcleo. | media | `{ctx}-api/src/codes.rs`. | abierto |
| ERR-4.5 | ERR-4 `fold` exhaustivo | contexts/iam/src/infrastructure/http/errors/error_mapper.rs:40-60; contexts/communities/src/infrastructure/http/errors/error_mapper.rs:37-46 | El status HTTP se decide comparando strings, con un comodín `_ => 500` (líneas 59 y 45). | media | `match` exhaustivo sobre el enum del contrato. | abierto |
| ERR-4.6 | ERR-4 Problem Details | shared/src/infrastructure/http/api_error_response.rs:4; app/src/middleware/auth.rs:22,28 | Las respuestas de error usan `{code,message}` y el 401 va con cuerpo vacío, no en RFC 9457. | media | `edge_http::Problem` con un helper por hoja. | abierto |
| ERR-4.7 | ERR-4 Manejador global | app/src/server.rs:24-27 | No hay manejador del árbol B, ni `FaultReporter`, ni `CatchPanicLayer`. Los `todo!`/`expect` cortan la conexión y las fallas técnicas no se registran. | media | `CatchPanicLayer` + `unexpected()` que reporta y devuelve un 500 genérico. | abierto |
| ERR-4.8 | ERR-4 Identidad faltante | contexts/communities/src/infrastructure/http/handlers/community_handlers.rs:18 | `Extension<VerifiedIdentity>` obligatoria: en una ruta sin proteger responde 500, no 401. | baja | `Option<Extension<AuthenticatedAccount>>` → 401. | abierto |

### TDD (tdd)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| TDD-1.1 | TDD-1 Test que lo exija | contexts/communities/src/application/queries/list_public_communities/list_public_communities.rs:46-143 | Los tests del handler están comentados (desde `56e43e8`) y apuntan a tipos que ya no existen: el handler no tiene ningún test. | media | Rojo primero con un doble que implemente `get_public_list`, o eliminar el caso. | abierto |
| TDD-1.2 | TDD-1 Test que lo exija | contexts/iam/src/infrastructure/persistence/in_memory/account_repository.rs:14; contexts/communities/src/infrastructure/persistence/in_memory/community_repository.rs:13 | Adaptadores en memoria sin tests propios; `find_by_id` de communities no se ejercita en ningún test. | media | Batería de contrato (TDD-2.1). | abierto |
| TDD-1.3 | TDD-1 Test que lo exija | contexts/iam/src/infrastructure/security/password_hasher/argon2_password_hasher.rs:20; contexts/iam/src/infrastructure/services/jwt_service/jwt_service.rs:52 | Adaptadores cubiertos solo indirectamente (`account_lifecycle.rs`, tests HTTP). | baja | Test del adaptador contra su tecnología. | abierto |
| TDD-1.4 | TDD-1 Test que lo exija | contexts/iam/src/domain/model/account/account_status.rs:74 | Los caminos de error de `from_str` (código no numérico, estado desconocido) no tienen test. | baja | Tests de dominio. | abierto |
| TDD-1.5 | TDD-1 Orden | commits `030479b`, `9d4c54e`, `2ad183c`, `3a0e403`, `9f33df1`/`2d68755`, `022885c` | Commits `feat`/`feature` que cambian producción sin agregar ni modificar ningún test. | media | El test, antes o en el mismo commit. | abierto |
| TDD-1.6 | TDD-1 Refactor | commit `56e43e8` "chore: redesign of architecture" | 260 archivos `.rs`, cero tests nuevos. Cambia firmas (use cases con `VerifiedIdentity`), comenta los tests de TDD-1.1 y borra unos 30 tests de membresía junto con su funcionalidad. No es un refactor sin cambio de comportamiento. | media | Separar el refactor de los cambios de comportamiento, y cada cambio con su test. | abierto |
| TDD-1.7 | TDD-1 Orden | commit `f12e2a9` | Tests de caracterización escritos después de la producción. Es legítimo como red de seguridad, pero no es rojo primero. | baja | Ninguna; dejarlo explícito en el PR. | abierto |
| TDD-2.1 | TDD-2 Batería de contrato | iam/…/ports/outbound/account_repository.rs:6, password_hasher.rs:1, token_generator.rs:3; communities/…/community_repository.rs:6; shared/…/token_verifier.rs:6 | Ningún puerto tiene batería de contrato; dobles y adaptadores no comparten comportamiento verificado. | alta | `testing/contracts/{puerto}.rs` + macro, contra el doble y el adaptador. | abierto |
| TDD-2.2 | TDD-2 Nombre de dobles | iam/…/account_repository.rs:23, password_hasher.rs:10, token_generator.rs:14; communities/…/community_repository.rs:24 | Dobles sin el sufijo `Adapter` (`FakeAccountRepository`…). | baja | `{Prefijo}{Concepto}Adapter`. | abierto |
| TDD-2.3 | TDD-2 Semántica del prefijo | iam/…/password_hasher.rs:10; token_generator.rs:14; account_repository.rs:23; communities/…/community_repository.rs:24 | `Fake` sin estado. `FakePasswordHasher` y `FakeTokenGenerator` devuelven respuestas fijas. Los repositorios se configuran por flags y `save` no guarda nada (account_repository.rs:98-106). | media | `FixedPasswordHashingAdapter`, `FixedAccessTokenAdapter`, `StubAccountAdapter`, o hacerlos `Fake` con estado real. | abierto |
| TDD-2.4 | TDD-2 Doble fiel | contexts/iam/src/application/ports/outbound/password_hasher.rs:19-21 | `verify` compara texto plano contra el hash, y el repositorio devuelve una cuenta cuyo email no es el consultado (account_repository.rs:95). El test de éxito tiene que mandar el hash como contraseña (authenticate_account.rs:163). | media | Doble coherente + batería de contrato. | abierto |
| TDD-2.5 | TDD-2 Doble completo | contexts/communities/src/application/ports/outbound/community_repository.rs:80 | El doble tiene `todo!()` en `get_public_list`, helpers muertos copiados de iam (líneas 59 y 68) y el campo `_activated` sin uso (línea 26). | baja | Implementar el método o quitarlo del puerto. | abierto |

### RS (rust-stack)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| RS-1.1 | RS-1 Crate por capa | Cargo.toml:2-5 | Un crate por contexto (`contexts/*`), con las capas como módulos; no hay `kernel/` ni `edge/http`. | alta | `contexts/{ctx}/{domain,application,infrastructure,api}` (migración incremental). | abierto |
| RS-1.2 | RS-1 Kernel | shared/Cargo.toml:2 | El kernel se llama `shared` y tiene capas propias (`application`, `infrastructure/http`). | media | `kernel/` (solo std) + `edge/http`. | abierto |
| RS-1.3 | RS-1 Herencia | Cargo.toml:1-11 | Sin `[workspace.package]` ni `[workspace.lints]`; versiones repetidas por crate (`uuid` en contexts/iam/Cargo.toml:7 y contexts/communities/Cargo.toml:7, `serde` en cuatro crates). | baja | Herencia del workspace. | abierto |
| RS-2.1 | RS-2 Árbol B = `Fault` | shared/src/error.rs:16 | No existen `Fault` ni `Failure<E>`; el árbol B viaja como `SystemError::Infrastructure`. | alta | `kernel::Fault` y `kernel::Failure`. | abierto |
| RS-2.2 | RS-2 `panic!` solo invariantes | iam/…/account.rs:36,40,44; account_id.rs:13,17; account_status.rs:18,22; events/account_event.rs:10,14,18; communities/…/in_memory/community_repository.rs:27 | 11 `todo!()` en código de producción. El de `get_public_list` haría caer la petición si se cablea. | media | Implementar o eliminar los `impl` de trait vacíos. | abierto |
| RS-2.3 | RS-2 `unwrap` prohibido | iam/…/authenticate_account.rs:55; verify_account.rs:30; identify_account.rs:44; account_status.rs:79 (dos veces), 81; app/src/config/jwt.rs:15; app/src/main.rs:12,14 | `unwrap` en producción. En `account_status.rs` panica ante un dato persistido; en `jwt.rs`, ante un `JWT_EXPIRATION_TIME` no numérico. | media | `let Some(..) else`, `?` o `ConfigError`. | abierto |
| RS-2.4 | RS-2 `expect("invariante: …")` | iam/…/argon2_password_hasher.rs:26,32; iam/…/in_memory/account_repository.rs:28,36,44; communities/…/in_memory/community_repository.rs:31,39,47 | Los mensajes no empiezan con "invariante:". Argon2 usa `expect` para una falla técnica (línea 26) y para un hash persistido corrupto (línea 32), que deberían ser `Fault`. | media | `Fault` en Argon2; "invariante: mutex sin envenenar" en los mutex. | abierto |
| RS-2.5 | RS-2 Lints | Cargo.toml:1-11 | Sin `[workspace.lints]` con `unwrap_used = "deny"` ni `unsafe_code`. | media | Agregarlos. | abierto |
| RS-3.1 | RS-3 Puertos async | iam/…/ports/outbound/*.rs; communities/…/community_repository.rs:6; shared/…/token_verifier.rs:6; los 6 `ports/inbound/*.rs` | Todos los puertos son síncronos: un adaptador con E/S real (Postgres) no cabe. Además, Argon2 bloquea el runtime dentro de handlers async. | media | `BoxFuture<'a, Result<…>>` del kernel. | abierto |
| RS-4.1 | RS-4 Tests de arquitectura | Cargo.toml:2-5 | No hay `cargo test -p architecture` (ver VER-1.1). | alta | `tools/architecture`. | resuelto (ad558fc) |

## 3. Reglas que se cumplen

Sin ninguna violación:
- **HEX-3:** ningún `domain` declara puertos ni usa primitivas de E/S. Los traits del dominio son marcadores o value objects, y `PasswordPolicy` es puro.

Partes que sí se cumplen dentro de reglas que tienen hallazgos:
- **HEX-2 (composition root):** `app/src/containers/*.rs` expone `Arc<dyn …UseCase>`, nunca el tipo concreto, y el cableado solo ocurre ahí.
- **HEX-4 (inyección):** los adaptadores HTTP inyectan el `UseCase` (`State<Arc<dyn RegisterAccountUseCase>>`), nunca el handler; cada `UseCase` tiene un solo `execute`.
- **HEX-7 (servicio de dominio):** `PasswordPolicy` no tiene estado ni E/S y no se llama `…Service`. Ningún tipo de `domain` ni de `application` termina en `Service`.
- **ERR-1 (`find_*`):** las búsquedas devuelven `Option`, y es el handler quien decide entre `NotFound` y `Conflict`.
- **ERR-3 (éxito degradado):** no aplica; no hay colaboradores que se llamen después de confirmar el estado.
- **ERR-4 (`HttpStatus` fuera del núcleo):** `StatusCode` solo aparece en `infrastructure/http` y en `app`.
- **TDD-2 (sin framework):** los tests de dominio y de handler no levantan axum ni tokio, y no usan librerías de mocks.
- **RS-3:** no se usa `async-trait`.

## 4. Dudas y hallazgos plausibles sin confirmar

No cuentan en el inventario. Si se confirman, pasan a él con el siguiente ID libre de su regla.

- **Enumeración de usuarios (ERR-4).** `authenticate` responde 404 a un usuario inexistente y 401 a una contraseña incorrecta, así que se puede saber qué nombres de usuario existen. Es decisión de producto (ERR-2), pero conviene revisarla.
- **`Account` deriva `Debug` con `HashedPassword` dentro** (account.rs:14). Cualquier log futuro con `{:?}` filtra el hash. No viola nada hoy, quitando el `dbg!` de ERR-4.2.
- **Orden en `RegisterAccount`.** Hashea con Argon2 (register_account.rs:49) antes de validar la política de contraseña dentro de la factory. Es orquestación cuestionable (costo y contraseña en claro pasada al dominio), no claramente un `if` de negocio en el handler.
- **Tests de caracterización en `app/tests/`.** Prueban reglas de negocio (longitudes, formatos) por el endpoint, y con adaptadores reales en vez de `Stub…UseCase`. Es la violación típica de TDD-2, pero están ahí a propósito como red de seguridad para el refactor. No los conté.
- **Verificar dos veces** devuelve `InvalidVerification` (400). Podría ser `Conflict`, pero depende de la semántica del producto (ERR-2).
- **Tratar `shared/` como el kernel.** Según rust-stack, los tests de arquitectura ignorarían `shared` como crate *legacy*. Se auditó contra HEX-8 porque hoy cumple ese papel. Si se decide que no es el kernel, HEX-8.1 a HEX-8.9 pasan a ser hallazgos de HEX-1/HEX-2 (un crate compartido entre contextos).
- **`contexts/iam/Cargo.lock` y `.idea/` versionados.** No violan ningún principio; no se contaron.
- **`HashedPasswordError` como respuesta de cliente (400).** Solo se produce si el hasher devuelve algo raro, así que sería árbol B. Queda dentro de ERR-1.1 y no aparte.

## Método

- Se leyó entero cada archivo `.rs` y `Cargo.toml` versionado, y cada `archivo:línea` se verificó contra su contenido en `d0b0eeb`.
- No se ejecutaron `cargo test` ni `clippy` (auditoría en solo lectura). El repo no tiene tests de arquitectura que correr.
- El orden de los commits se revisó con `git log --reverse`.
