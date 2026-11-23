# Adopción del harness craft-backend

Este archivo guarda el estado entre sesiones de la adopción de los principios craft-backend (HEX, ERR, TDD, VER, RS) en este repositorio.

| Dato | Valor |
|---|---|
| Fecha de la auditoría | 2026-11-19 |
| Plugin | `craft-backend@ai-harness` 0.12.0 |
| Commit auditado | `d0b0eeb` (rama `ci/sync-development`) |
| Revisor | `craft-backend:architecture-reviewer`, auditoría completa en solo lectura |
| Hallazgos | 94: 25 de severidad alta, 48 media, 21 baja (RS-1.4 agregado en R2, fuera de la auditoría) |

## Cómo mantener este archivo

- Al resolver un hallazgo, cambiar su **Estado** de `abierto` a `resuelto (<commit>)`. Si se descarta, poner `descartado` y el motivo.
- Los IDs de hallazgo (`HEX-1.3`, `ERR-4.2`…) son estables: no se renumeran. Un hallazgo nuevo recibe el siguiente número libre de su regla.
- Después de cada slice de refactor, volver a auditar con architecture-reviewer y actualizar la cabecera (fecha, commit, conteos).
- Tests de caracterización en `app/tests/` (PR #18): son la red de seguridad del refactor y tienen que seguir pasando.

## Prioridades inmediatas

1. **ERR-4.1:** `.env` ya no se versiona (0df9baa), pero `JWT_SECRET` sigue en el historial: **rotar el secreto** en cada entorno donde se haya usado.
2. **ERR-1.1 / RS-2.1:** `SystemError`, con su `From` genérico, mezcla los errores de negocio con los técnicos. Hay que reemplazarlo por `{Verb}{Noun}Error` por caso de uso y `Fault`/`Failure<E>`.
3. **RS-1 / HEX-1 / HEX-2:** separar cada contexto en crates por capa, empezando por `communities` (HEX-1.1).

Ya resueltas: ERR-4.2 (`dbg!`), ERR-4.3 (`Debug` con el secreto), VER-1.1/RS-4.1 (`tools/architecture` con trinquete sobre `baseline.txt`), VER-1.2 (CI con fmt, clippy y test), RS-2.5 (`[workspace.lints]`) ERR-1.2/ERR-1.3 (los errores ya no implementan `std::error::Error`) HEX-8.4/HEX-8.7 (código muerto del kernel) y RS-1.3/RS-1.4 (herencia del workspace, sin `anyhow`).

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
| RS-1 Crate por capa | 1 | 1 | 2 | 4 |
| RS-2 `Fault` y `panic!` | 1 | 4 | 0 | 5 |
| RS-3 Puertos async | 0 | 1 | 0 | 1 |
| RS-4 Tests de arquitectura | 1 | 0 | 0 | 1 |
| **Total** | **25** | **48** | **21** | **94** |

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
| HEX-2.5 | HEX-2 Núcleo sin framework | contexts/communities/Cargo.toml:7-10 | `uuid`, `axum`, `serde` y `http` (este último, visto en S1) están disponibles en `domain` y `application`. | alta | Separar crates por capa. | abierto |
| HEX-2.6 | HEX-2 Núcleo sin framework | contexts/communities/src/domain/model/community/community_id.rs:2 | El dominio usa `uuid`. | media | `IdGenerator` + value object sobre `String`. | abierto |
| HEX-4.1 | HEX-4 Casos de uso | iam/…/authenticate_account.rs:17; register_account.rs:18; verify_account.rs:14; identify_account.rs:16; communities/…/create_community.rs:21; list_public_communities.rs:14 | Las implementaciones no se llaman `{Verb}{Noun}Handler` (`AuthenticateAccount`, `RegisterAccount`…). | media | Renombrar a `AuthenticateAccountHandler` y demás, en `handler.rs`. | abierto |
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
| HEX-8.1 | HEX-8 Kernel sin dependencias | shared/Cargo.toml:7-8 | El kernel depende de `chrono` y `serde`. | alta | Kernel solo `std`. | abierto: desde S1, `chrono` sale en R21 y `serde` en R25 |
| HEX-8.2 | HEX-8 Marcadores | shared/src/domain/domain_event.rs:5 | El marcador `DomainEvent` exige `DateTime<Utc>` de `chrono`. | media | Sin dependencias; el instante llega de `Clock`. | abierto |
| HEX-8.3 | HEX-8 Kernel | shared/src/error.rs:9 | `ErrorLayout` y `SystemError` (línea 16), un error global por capas, viven en el kernel. | alta | `Fault` + `Failure<E>` (RS-2). | abierto |
| HEX-8.4 | HEX-8 Marcadores de error | shared/src/domain/domain_error.rs:3; shared/src/infrastructure/infrastructure_error.rs:3 | Traits `DomainError` e `InfrastructureError`: Rust no los necesita (y aquí nadie los usa). | media | Eliminarlos. | resuelto (80c00e4) |
| HEX-8.5 | HEX-8 Sin puertos | shared/src/application/ports/outbound/token_verifier.rs:6 | Un puerto de negocio (`TokenVerifier`) y su `TokenError` (error.rs:2) están en el kernel; no es una capacidad de la lista. | alta | Moverlo a `iam` (ver HEX-1.5). | abierto |
| HEX-8.6 | HEX-8 Sin modelos | shared/src/domain/model/verified_identity.rs:2; shared/src/application/auth_context.rs:2 | Tipos de identidad en el kernel. | media | `AuthenticatedAccount` en `edge/http`. | abierto |
| HEX-8.7 | HEX-8 Kernel | shared/src/application/common_application_error.rs:2 | `CommonApplicationError` está muerto y es un error compartido de aplicación. | baja | Eliminarlo. | resuelto (80c00e4) |
| HEX-8.8 | HEX-8 Kernel | shared/src/infrastructure/http/api_error_response.rs:4 | Un tipo de transporte HTTP (con `serde`) en el kernel. | media | `edge/http` con `Problem`. | abierto |
| HEX-8.9 | HEX-8 Marcadores | shared/src/domain/aggregate_root.rs:6-8 | `AggregateRoot` no es un marcador: impone `record_event(&mut self)`, `pull_events` y `version`, una API mutable. | baja | Marcador vacío; los eventos se manejan con `take_events(self)` en el agregado. | abierto |
| VER-1.1 | VER-1 Tests de arquitectura | Cargo.toml:2-5 | No existe `tools/architecture`: ninguna regla de dependencias o de nombres tiene test. | alta | Agregar `tools/architecture` (la plantilla de rust-stack). | resuelto (ad558fc) |
| VER-1.2 | VER-1 CI | .github/workflows/sync-development.yml:22-58 | El único workflow sincroniza ramas; no corre `fmt`, `clippy` ni `test`. | alta | Workflow con el CI mínimo de RS-4. | resuelto (c54f6ae) |

### ERR (error-handling)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| ERR-1.1 | ERR-1 Dos árboles | shared/src/error.rs:16 | `SystemError` mezcla los fallos de negocio con los técnicos (`Infrastructure`) en un tipo. El `impl<T: ErrorLayout> From<T>` (línea 32) hace que `?` los absorba implícitamente. | alta | `Failure<{Verb}{Noun}Error>` y `Fault` separados. | abierto |
| ERR-1.2 | ERR-1 Errores de dominio | iam/src/domain/errors/account.rs:27, account_id.rs:20, account_status.rs:20, code_validation.rs:18, email.rs:18, hashed_password.rs:18, password_policy.rs:20, username.rs:18; communities/src/domain/errors/community_error.rs:38, invalid_community_id.rs:14, invalid_community_name.rs:14, invalid_community_slug.rs:14 | Los errores de dominio implementan `std::error::Error`: un `?` los manda al canal técnico. | alta | Solo `Debug` + `Display`. | resuelto (eb30039) |
| ERR-1.3 | ERR-1 Errores de application | iam/src/application/errors/account_repository.rs:14, authenticate_account.rs:24, token_generator.rs:14; communities/src/application/errors/community_creation.rs:20, community_repository.rs:14 | Las hojas y los errores de puerto implementan `std::error::Error`. | media | Quitar el `impl`. | resuelto (eb30039) |
| ERR-1.4 | ERR-1 Fallo silenciado | contexts/iam/src/domain/model/account/account.rs:127-129 | `let _ = self.transition_status(…).map_err(…)` descarta el `Result` y responde `Ok`, aunque la transición falle. | media | Propagar con `?`, o `expect("invariante: Registered→Active siempre válida")`. | abierto |
| ERR-1.5 | ERR-1 Conversión implícita | contexts/iam/src/domain/model/account/account_factory.rs:39-43 | En `reconstitute`, un dato persistido corrupto se convierte en error de negocio (400) con `?`. | media | En el adaptador, conversión a `Fault` a mano y con contexto. | abierto |
| ERR-1.6 | ERR-1 Sin árbol A | contexts/communities/src/application/ports/inbound/list_public_communities_use_case.rs:11 | El caso de uso no tiene fallos de negocio, pero declara un tipo de error (`SystemError`). | baja | `Result<Response, Fault>`. | abierto |
| ERR-1.7 | ERR-1 Firma del puerto | iam/…/ports/outbound/account_repository.rs:7-9; password_hasher.rs:2-3; communities/…/community_repository.rs:7-10 | `find_*` devuelve `Option` sin `Result`, `hash` devuelve `String` y `verify`, `bool`: un adaptador real no puede reportar una falla técnica y tiene que entrar en pánico. | media | `Result<Option<T>, Fault>`, `Result<String, Fault>`. | abierto |
| ERR-2.1 | ERR-2 Contrato por caso de uso | iam/…/ports/inbound/authenticate_account_use_case.rs:10, identify_account_use_case.rs:7, register_account_use_case.rs:10, verify_account_use_case.rs:5; communities/…/create_community_use_case.rs:11 | Ningún caso de uso tiene su `{Verb}{Noun}Error` cerrado: todos devuelven el global `SystemError`. | alta | `RegisterAccountError { Validation, Conflict }` y demás, junto al handler. | abierto |
| ERR-2.2 | ERR-2 Sin envolver | shared/src/error.rs:17-19 | Los errores se envuelven (`Domain(Box<dyn ErrorLayout>)`), y el error de dominio llega tal cual al contrato (p. ej. `AccountError::EmailAlreadyExists.into()` en register_account.rs:38). | alta | Traducir a una hoja plana (`Conflict { resource }`). | abierto |
| ERR-2.3 | ERR-2 Contrato incompleto | contexts/iam/src/application/errors/authenticate_account.rs:6 | `AuthenticateAccountError` no cubre `NotFound` (viene de `AccountError`) ni la falla de token: hay contratos parciales mezclados. | media | Un enum cerrado con todas las variantes que el caso produce. | abierto: R2 (ad2f129) quitó `NotFound` del caso; falta la falla de token |
| ERR-2.4 | ERR-2 Nombre | contexts/communities/src/application/errors/community_creation.rs:6 | `CommunityCreationError` en vez de `CreateCommunityError`, y fuera del directorio del caso. | baja | `commands/create_community/error.rs`. | abierto |
| ERR-2.5 | ERR-2 Variante correcta | contexts/iam/src/domain/model/account/account_status.rs:89 | Un estado ilegible se reporta como `InvalidTransition`. | baja | Dato corrupto → `Fault` en el adaptador. | abierto |
| ERR-2.6 | ERR-2 Tests por variante | contexts/iam/src/application/commands/register_account/register_account.rs:125 (patrón en todos los tests de handler) | Los tests afirman sobre `err.code()` (un string), no sobre la variante. | baja | `matches!(r, Err(Failure::Expected(…)))`. | abierto |
| ERR-2.7 | ERR-2 Test por variante | contexts/iam/src/application/commands/register_account/register_account.rs:65 | Falta el test del handler para contraseña corta (`fails_when_password_is_too_short`). Falla de repositorio sin test de endpoint (solo el unit test del mapper). | media | Agregar ambos. | abierto |
| ERR-2.8 | ERR-2 Test por variante | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:77 | La falla del generador de tokens no tiene test de handler (`fails_when_token_generation_fails`) ni de endpoint. | media | Doble que falle + test. | abierto |
| ERR-2.9 | ERR-2 Test por variante | contexts/iam/src/application/commands/verify_account/verify_account.rs:41 | Faltan el test de handler para código fuera de rango (`fails_when_code_is_out_of_range`) y los de handler y endpoint para la falla del repositorio (`fails_when_repository_fails`). | media | Agregar los tests. | abierto |
| ERR-2.10 | ERR-2 Test por variante | contexts/communities/src/application/commands/create_community/create_community.rs:62 | El owner con formato inválido o nil no tiene test de handler (`fails_when_owner_id_is_invalid`), solo de endpoint. La falla de repositorio no tiene test de endpoint. | media | Agregar los tests. | abierto |
| ERR-3.1 | ERR-3 Respondió vs no pudo | contexts/iam/src/infrastructure/services/jwt_service/jwt_service.rs:85-92 | Un token inválido o vencido se devuelve como `Err(TokenError)`; es un veredicto, no un fallo. | media | `Ok(AccessTokenVerdict::Invalid)`. | abierto |
| ERR-3.2 | ERR-3 Errores para transporte | iam/src/application/errors/account_repository.rs:16-27, token_generator.rs:16-27; communities/src/application/errors/community_repository.rs:16-27 | Los errores de puerto traen un `code()` y un `message()` pensados para el cliente HTTP. | media | Sin texto de usuario; el borde decide. | abierto |
| ERR-3.3 | ERR-3 Falla técnica = árbol B | iam/src/application/errors/account_repository.rs:6, token_generator.rs:6; communities/src/application/errors/community_repository.rs:6 | Fallas técnicas no declarables (persistencia, codificación JWT) modeladas como errores tipados del contrato. | media | `Fault`, propagado con `?`. | abierto |
| ERR-4.1 | ERR-4 Secretos | .env:1 | `JWT_SECRET` está versionado; `.gitignore:1-2` no excluye `.env`. | alta | Sacar `.env` del índice y rotar el secreto; `.env.example` sin valores. | en curso (0df9baa): fuera del índice; falta rotar el secreto |
| ERR-4.2 | ERR-4 Logging en el núcleo | contexts/iam/src/application/commands/register_account/register_account.rs:55 | `dbg!(&account)` escribe en stderr la cuenta completa (email y hash de la contraseña). | alta | Eliminarlo. | resuelto (57e814e) |
| ERR-4.3 | ERR-4 Secretos | app/src/config/jwt.rs:3; app/src/config/app.rs:3 | `#[derive(Debug)]` sobre una configuración que contiene el secreto. | media | Sin `Debug`. | resuelto (8195600) |
| ERR-4.4 | ERR-4 Códigos en el borde | iam/src/domain/errors/error_codes.rs:1-12; iam/src/application/errors/error_codes.rs:1-4; communities/src/domain/errors/error_codes.rs:1-9; communities/src/application/errors/error_codes.rs:1-2 | Los códigos estables y los mensajes de cliente viven en el núcleo. | media | `{ctx}-api/src/codes.rs`. | abierto |
| ERR-4.5 | ERR-4 `fold` exhaustivo | contexts/iam/src/infrastructure/http/errors/error_mapper.rs:40-60; contexts/communities/src/infrastructure/http/errors/error_mapper.rs:37-46 | El status HTTP se decide comparando strings, con un comodín `_ => 500` (líneas 59 y 45). | media | `match` exhaustivo sobre el enum del contrato. | abierto |
| ERR-4.6 | ERR-4 Problem Details | shared/src/infrastructure/http/api_error_response.rs:4; app/src/middleware/auth.rs:22,28 | Las respuestas de error usan `{code,message}` y el 401 va con cuerpo vacío, no en RFC 9457. | media | `edge_http::Problem` con un helper por hoja. | abierto |
| ERR-4.7 | ERR-4 Manejador global | app/src/server.rs:24-27 | No hay manejador del árbol B, ni `FaultReporter`, ni `CatchPanicLayer`. Los `todo!`/`expect` cortan la conexión y las fallas técnicas no se registran. | media | `CatchPanicLayer` + `unexpected()` que reporta y devuelve un 500 genérico. | abierto: R5 (29890cb) existe el trait `FaultReporter`, y los mappers escriben el `Fault` en stderr como interino; faltan el adaptador, `unexpected()` y `CatchPanicLayer` (R26, R30) |
| ERR-4.8 | ERR-4 Identidad faltante | contexts/communities/src/infrastructure/http/handlers/community_handlers.rs:18 | `Extension<VerifiedIdentity>` obligatoria: en una ruta sin proteger responde 500, no 401. | baja | `Option<Extension<AuthenticatedAccount>>` → 401. | abierto |

### TDD (tdd)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| TDD-1.1 | TDD-1 Test que lo exija | contexts/communities/src/application/queries/list_public_communities/list_public_communities.rs:46-143 | Los tests del handler están comentados (desde `56e43e8`) y apuntan a tipos que ya no existen: el handler no tiene ningún test. | media | Rojo primero con un doble que implemente `get_public_list`, o eliminar el caso. | abierto |
| TDD-1.2 | TDD-1 Test que lo exija | contexts/iam/src/infrastructure/persistence/in_memory/account_repository.rs:14; contexts/communities/src/infrastructure/persistence/in_memory/community_repository.rs:13 | Adaptadores en memoria sin tests propios; `find_by_id` de communities no se ejercita en ningún test. | media | Batería de contrato (TDD-2.1). | abierto |
| TDD-1.3 | TDD-1 Test que lo exija | contexts/iam/src/infrastructure/security/password_hasher/argon2_password_hasher.rs:20; contexts/iam/src/infrastructure/services/jwt_service/jwt_service.rs:52 | Adaptadores cubiertos solo indirectamente (`account_lifecycle.rs`, tests HTTP). | baja | Test del adaptador contra su tecnología. | abierto |
| TDD-1.4 | TDD-1 Test que lo exija | contexts/iam/src/domain/model/account/account_status.rs:74 | Los caminos de error de `from_str` (código no numérico, estado desconocido) no tienen test. | baja | Tests de dominio. | resuelto (4355c9f) |
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
| RS-1.3 | RS-1 Herencia | Cargo.toml:1-11 | Sin `[workspace.package]` ni `[workspace.lints]`; versiones repetidas por crate (`uuid` en contexts/iam/Cargo.toml:7 y contexts/communities/Cargo.toml:7, `serde` en cuatro crates). | baja | Herencia del workspace. | resuelto (991e672) |
| RS-1.4 | RS-1 Dependencias | app/Cargo.toml:18 | `anyhow` está declarado y no se usa: cargo avisa `unused dependency`. Hallado en R2, fuera de la auditoría. | baja | Quitarlo, junto con RS-1.3 (R6). | resuelto (991e672) |
| RS-2.1 | RS-2 Árbol B = `Fault` | shared/src/error.rs:16 | No existen `Fault` ni `Failure<E>`; el árbol B viaja como `SystemError::Infrastructure`. | alta | `kernel::Fault` y `kernel::Failure`. | abierto: R5 (e76c162, 29890cb, 446726b) agrega `Fault` y `Failure<E>` y el puente `SystemError::Unexpected`; falta retirar `SystemError` (R19) |
| RS-2.2 | RS-2 `panic!` solo invariantes | iam/…/account.rs:36,40,44; account_id.rs:13,17; account_status.rs:18,22; events/account_event.rs:10,14,18; communities/…/in_memory/community_repository.rs:27 | 11 `todo!()` en código de producción. El de `get_public_list` haría caer la petición si se cablea. | media | Implementar o eliminar los `impl` de trait vacíos. | abierto |
| RS-2.3 | RS-2 `unwrap` prohibido | iam/…/authenticate_account.rs:55; verify_account.rs:30; identify_account.rs:44; account_status.rs:79 (dos veces), 81; app/src/config/jwt.rs:15; app/src/main.rs:12,14 | `unwrap` en producción. En `account_status.rs` panica ante un dato persistido; en `jwt.rs`, ante un `JWT_EXPIRATION_TIME` no numérico. | media | `let Some(..) else`, `?` o `ConfigError`. | resuelto (0415e3e, 4355c9f, ebcd593, 9315004) |
| RS-2.4 | RS-2 `expect("invariante: …")` | iam/…/argon2_password_hasher.rs:26,32; iam/…/in_memory/account_repository.rs:28,36,44; communities/…/in_memory/community_repository.rs:31,39,47 | Los mensajes no empiezan con "invariante:". Argon2 usa `expect` para una falla técnica (línea 26) y para un hash persistido corrupto (línea 32), que deberían ser `Fault`. | media | `Fault` en Argon2; "invariante: mutex sin envenenar" en los mutex. | abierto |
| RS-2.5 | RS-2 Lints | Cargo.toml:1-11 | Sin `[workspace.lints]` con `unwrap_used = "deny"` ni `unsafe_code`. | media | Agregarlos. | resuelto (9315004) |
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

- ~~**Enumeración de usuarios (ERR-4).** `authenticate` responde 404 a un usuario inexistente y 401 a una contraseña incorrecta, así que se puede saber qué nombres de usuario existen.~~ Confirmada por el usuario y resuelta en R2 (ad2f129, 4149660): 401 `IAM_LOGIN_FAILED` y verificación contra un hash señuelo (D6).
- **El señuelo y los parámetros de Argon2 (R2).** `verify` usa los parámetros del hash guardado. Si `Argon2::default()` cambia al actualizar el crate, el señuelo (generado al arrancar con los parámetros nuevos) y los hashes viejos dejan de costar lo mismo. Hoy no pasa; habría que revisarlo al subir `argon2` o al cambiar sus parámetros.
- **`Account` deriva `Debug` con `HashedPassword` dentro** (account.rs:14). Cualquier log futuro con `{:?}` filtra el hash. No viola nada hoy, quitando el `dbg!` de ERR-4.2.
- **Orden en `RegisterAccount`.** Hashea con Argon2 (register_account.rs:49) antes de validar la política de contraseña dentro de la factory. Es orquestación cuestionable (costo y contraseña en claro pasada al dominio), no claramente un `if` de negocio en el handler.
- **Tests de caracterización en `app/tests/`.** Prueban reglas de negocio (longitudes, formatos) por el endpoint, y con adaptadores reales en vez de `Stub…UseCase`. Es la violación típica de TDD-2, pero están ahí a propósito como red de seguridad para el refactor. No los conté.
- **Verificar dos veces** devuelve `InvalidVerification` (400). Podría ser `Conflict`, pero depende de la semántica del producto (ERR-2).
- **Tratar `shared/` como el kernel.** Según rust-stack, los tests de arquitectura ignorarían `shared` como crate *legacy*. Se auditó contra HEX-8 porque hoy cumple ese papel. Si se decide que no es el kernel, HEX-8.1 a HEX-8.9 pasan a ser hallazgos de HEX-1/HEX-2 (un crate compartido entre contextos).
- **`contexts/iam/Cargo.lock` y `.idea/` versionados.** No violan ningún principio; no se contaron.
- **`HashedPasswordError` como respuesta de cliente (400).** Solo se produce si el hasher devuelve algo raro, así que sería árbol B. Queda dentro de ERR-1.1 y no aparte.

## 5. Plan de rebanadas

Las rebanadas se citan como **R1…R30**. Cada una es un PR que tiene que estar verde en CI. En ese mismo PR se actualiza el inventario (`resuelto (<commit>)`) y, al terminar, se vuelve a auditar con `craft-backend:architecture-reviewer`. El avance de cada rebanada se anota en §5.8.

Datos verificados que condicionan el plan:
- ~~`hex_4_use_case_contracts_do_not_expose_domain_types` no actúa sobre los contextos legacy.~~ Desde S1 (ai-harness `v1.0.0-rc.1`) sí actúa: en legacy, el dominio de application es `crate::domain`. Revisa 17 tipos de contrato y pasa en verde.
- Un contexto legacy no puede depender de un crate `edge/*`. Todas las capas virtuales comparten el manifiesto, y `(Domain, Edge)` está prohibido. En `iam` aparecería una entrada nueva en `contexts/iam/Cargo.toml`. En `communities` seguiría viva la entrada de HEX-1.1.
- `hex_2_core_depends_only_on_std_and_the_kernel` cuenta también las `[dev-dependencies]`. Después de la separación, los tests de application no pueden usar tokio.
- Los tests de caracterización fijan `IAM_INVALID_ACCOUNT_ID(_FORMAT)` en `/communities/create`, los bodies `{code,message}` exactos y el 401 con cuerpo vacío. Además documentan como "CURRENT BEHAVIOR" el duplicado por mayúsculas (HEX-7.1).
- `AccountFactory::reconstitute` y `AccountStatus::from_str` no se usan fuera de los tests: ningún adaptador persiste en texto.
- Decidido por el usuario: `account_lifecycle.rs` admite **adaptación mecánica**.
- `tools/architecture/baseline.txt` tiene 63 líneas: 8 de comentario y **55 entradas**. Coincide con la suma de §5.6. El "56" del encargo original era un error de conteo; ninguna entrada queda sin rebanada. *(En R1. Después de R3 y R4 quedaron 36, y S1 las reescribe como 47 con el formato nuevo: ver §5.10.)*

### 5.1 Reglas transversales

- **Trinquete.** Cuando un PR mueve o renombra un archivo que figura en `baseline.txt`, en ese mismo PR la violación tiene que desaparecer. Si no, el archivo no se toca. Antes de abrir cada PR: `cargo test -p architecture`.
- **Red de seguridad.** `app/tests/` no se modifica, salvo en los cambios de contrato listados en §5.5.
- **`account_lifecycle.rs` (decidido: adaptación mecánica).** Se permite cambiar imports, nombres, `.await`/`block_on` y reemplazar `code()` por un `matches!` sobre la variante equivalente. Ningún escenario ni aserto cambia de significado. El PR enumera cada adaptación.
- **TDD.** El "rojo" de los tests de arquitectura es la entrada de la línea base. Al corregir la violación, el test falla con "Ya no violan: borrar", y el PR borra esa línea. Los tests que hoy afirman sobre `code()` se reemplazan por la versión que afirma sobre la variante: primero se escribe la nueva, en rojo, y después se borra la vieja (ERR-2.6).
- ~~**Verificación manual mientras haya legacy.**~~ Desde S1, el test de contratos (HEX-4) actúa sobre legacy: una filtración de `crate::domain` en `commands/*/{command,response,error}.rs` o en `ports/inbound/` es una violación nueva en la misma rebanada que la introduce. Ya no hace falta el `grep` a mano en R11–R18.
- **Una dependencia, una entrada.** Desde S1, las claves de los tests de dependencias son `ruta/Cargo.toml#sección.paquete`, con `n` = número de capas virtuales que la ven. La rebanada que quita un paquete de un `Cargo.toml` borra sus entradas en el mismo PR, aunque el plan asignara el cierre a otra rebanada.
- **La línea base solo crece en una sincronización con el harness.** Esa sincronización va en un PR propio, sin código de producción, y con el análisis de la línea base vieja frente a la nueva: entradas que se mantienen, las que cambian de clave, las que desaparecen y las nuevas, con ID del inventario y rebanada (ver §5.10). **Nunca en una rebanada.** Si una entrada desaparece sin un cambio de formato que lo explique, el PR se detiene: puede ser un cierre en falso.

### 5.2 Errores y estructura: recomendación

**Migrar los errores antes de separar los crates, en la disposición legacy.** La hipótesis "corregir en el lugar antes de mover" se sostiene, con estos ajustes:

1. **La excepción del HTTP es más chica de lo propuesto.** ERR-4.4 y ERR-4.5 se pueden hacer en el lugar. Cada caso de uso migrado trae su `fold` exhaustivo, en `infrastructure/http/<caso>.rs`, y sus códigos en `infrastructure/http/codes.rs`. Esos archivos no violan nada y se mudan intactos a `{ctx}-api/src`. Lo que sí espera a la separación es lo que necesita `edge/http`: ERR-4.6, ERR-4.7 (la parte de `unexpected()` en los handlers), ERR-4.8, HEX-1.6, HEX-8.6 y HEX-8.8. La razón es la del contexto: un contexto legacy no puede depender de `edge-http` sin agregar una entrada en `contexts/iam/Cargo.toml` o sin bloquear la de `contexts/communities/Cargo.toml`.
2. **Lo estructural por definición.** HEX-1.7, HEX-1.9, HEX-2.1, HEX-2.5 y RS-1.1 se resuelven solo separando crates. Las entradas `hex_2_*` de los `Cargo.toml` de contexto no desaparecen mientras el HTTP comparta manifiesto con el dominio.
3. **Lo que la separación verifica por primera vez.** La separación es la primera verificación real de HEX-4: el test de contratos solo empieza a actuar ahí. Por eso conviene que R27 y R28 muevan código que ya se revisó a mano (§5.1).
4. **Antes de separar.** Hay que sacar del dominio `uuid`, `rand` y `chrono` (R20–R22). Además, los puertos ya tienen que ser async con un `block_on` de std (R23–R24). Si no, los manifiestos nuevos de domain y application nacen con violaciones de `hex_2_core` que la línea base no admite.

Si se hiciera al revés (separar primero), habría que mover las 17 entradas ERR-1 y las 6 HEX-4 a rutas nuevas, todavía violando, y el trinquete lo prohíbe. Las alternativas serían arreglarlas en el PR de la mudanza, que deja de ser chico, o escribir dos veces el mapeo de `SystemError`.

### 5.3 Secuencia

Tamaños: XS (<50 líneas), S (<200), M (<500), L (>500 o muchos archivos).

#### Fase 0: base

**R1. Fijar el nightly.** *Hecha (ver §5.8).*
- **Objetivo:** `rust-toolchain.toml` con `channel = "nightly-AAAA-MM-DD"`. La fecha es la del último CI verde: el `rustc -V` del paso "Install toolchain". Localmente hoy es `2026-03-02`.
- **Resultado:** `channel = "nightly-2026-11-19"`. El último CI verde (run 44647947435, `fix/security-slice-0`, 2026-11-20) usó `rustc 1.102.0-nightly (65ce8611f 2026-11-18)`. rustup publica ese build en el canal del 2026-11-19, un día después de la fecha del commit que muestra `rustc -V`. La toolchain local (`2026-03-02`) era ocho meses más vieja y nunca pasó por CI; se descartó.
- **IDs:** ninguno (higiene de VER-1).
- **Toca:** `rust-toolchain.toml`.
- **Línea base:** sin cambios.
- **Tests:** ninguno nuevo. Se verifica con fmt, clippy y test verdes en CI con la fecha fijada.
- **Riesgo para la caracterización:** nulo.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** XS.

**R2. Sign-in responde 401 a un usuario inexistente (decidido).** *Hecha (ver §5.8).*
- **IDs:** ninguno. Viene de la duda de la §4 confirmada por el usuario.
- **Toca:** `authenticate_account.rs` y sus tests, y el cableado de `AuthenticateAccount` en `app/src/containers/`.
- **Alcance de D6 (decidido: sí, dentro de R2).** Cuando el usuario no existe, el handler igual llama a `verify` del hasher, con la contraseña recibida contra un **hash señuelo**, y descarta el resultado. Así el 401 tarda lo mismo que con una contraseña incorrecta. El composition root genera el señuelo una sola vez al arrancar (`hasher.hash(<valor aleatorio>)`) y se lo pasa al handler, así que tiene los mismos parámetros de Argon2 que los hashes reales. El handler no conoce el formato del hash.
- **Línea base:** sin cambios.
- **Rojo:**
  - test de handler `fails_with_login_failed_when_account_does_not_exist`: hoy da `IAM_ACCOUNT_NOT_FOUND`;
  - test de handler `verifies_against_the_decoy_hash_when_account_does_not_exist` (D6): un doble del hasher que registra sus llamadas tiene que recibir un `verify` con la contraseña y el señuelo. Hoy no recibe ninguno;
  - tests HTTP `authenticate_unknown_username_returns_401_login_failed` y `authenticate_by_email_returns_401`.
- **Contrato HTTP: cambia.** `POST /auth/authenticate` con un usuario inexistente pasa de 404 `IAM_ACCOUNT_NOT_FOUND` a 401 `IAM_LOGIN_FAILED`. Se reemplazan `authenticate_unknown_username_returns_404` y `authenticate_by_email_returns_404`.
- **Fuera de alcance:** un test HTTP que mida tiempos. Sería inestable en CI; el test del handler alcanza para fijar la llamada al señuelo.
- **Tamaño:** S.
- **Resultado:**
  - Commits `ad2f129` (401 + reemplazo de los dos tests HTTP), `4149660` (señuelo) y `8a70093` (`std::hint::black_box` sobre el `verify` descartado, a pedido del revisor).
  - El doble se llama `SpyPasswordHasherPortAdapter` y no `SpyPasswordHashingAdapter`. El test de arquitectura deriva el concepto del puerto actual, `PasswordHasherPort`, y la línea base no admite entradas nuevas. R9 lo renombra junto con el puerto y lo mueve a `application/testing.rs`. Devuelve `true` en `verify` para probar que el handler descarta el resultado.
  - Se borró el test de handler `fails_when_account_not_found`: fijaba el 404 que reemplaza `fails_with_login_failed_when_account_does_not_exist`.
  - `account_lifecycle.rs`: adaptación mecánica (decidida por el usuario en R2). `iam()` genera el señuelo con `hasher.hash("lifecycle-decoy")` y lo pasa a `AuthenticateAccount::new`, más el import de `PasswordHasherPort`. Ningún escenario ni aserto cambia.
  - Los tests nuevos afirman sobre `err.code()`, como el resto del archivo. Pasar a variantes es ERR-2.6, y la migración cubre `fails_with_login_failed_when_account_does_not_exist` y `verifies_against_the_decoy_hash_when_account_does_not_exist`, que afirman el mismo código.
  - Revisión con `craft-backend:architecture-reviewer`: ningún hallazgo bloqueante. Lo que señaló de otras rebanadas ya figura en el inventario y no se corrigió aquí:
    - el sufijo `Port` de `PasswordHasherPort` (HEX-5.1, R9);
    - `FakePasswordHasher` sin estado ni batería (TDD-2.1, TDD-2.3, R9);
    - los `expect` de Argon2, de los que `hash` ahora corre también al arrancar para generar el señuelo (RS-2.4, R9);
    - `if !verify` e `if !can_authenticate` en el handler (HEX-7.2);
    - `SystemError` y los asertos sobre `code()` (ERR-2.1, ERR-2.6, R14).

#### Fase 1: limpieza barata en el lugar

**R3. Los errores dejan de implementar `std::error::Error`.** *Hecha (ver §5.8).*
- **IDs:** ERR-1.2, ERR-1.3.
- **Toca:**
  - `shared/src/error.rs`: `ErrorLayout` pasa a exigir `Debug` en lugar de `Error`;
  - los 17 archivos `*/errors/*.rs`: se quita el `impl`.
- **Línea base: −17.** Son todas las entradas `err_1_*`.
- **Tests:** el trinquete.
- **Riesgo:** nulo.
- **Tamaño:** S.
- **Resultado:**
  - Commit `eb30039`. Ningún tipo usaba `thiserror`, y todos tenían `Display` a mano, así que solo se quitaron los 17 `impl std::error::Error for … {}` vacíos y el `use std::error::Error` de `shared/src/error.rs`.
  - El rojo fue el trinquete: con los `impl` quitados, `err_1_domain_errors_do_not_implement_std_error` (12) y `err_1_application_errors_do_not_implement_std_error` (5) fallaron con "Ya no violan: borrar". Se borraron las 17 líneas de `baseline.txt`.
  - Nada dependía de que fueran `Error`: no hay `?` hacia `anyhow`, ni `Box<dyn Error>`, ni `.source()`. Compiló sin tocar otro archivo.
  - Ningún test cambió, salvo la línea base. `app/tests/` y `account_lifecycle.rs` quedaron intactos.
  - Commit aparte `f552921` (`test(iam)`): se separaron los dos tests de R2. `verifies_against_the_decoy_hash_when_account_does_not_exist` afirma solo la llamada al Spy. `fails_with_login_failed_when_account_does_not_exist` afirma solo la respuesta y ahora usa el Spy, que responde `true` en `verify`, para seguir probando que el handler descarta el veredicto del señuelo (con `FakePasswordHasher` el señuelo da `false` y esa garantía se perdía).
  - Fuera de alcance, sin tocar: `ErrorWithCode` de los tests de los dos `error_mapper.rs` sigue implementando `Error`, que ya no hace falta (se borra con los mappers en R19); `app/src/config/error.rs` (`ConfigError`) implementa `Error` y es del composition root, donde está permitido; `anyhow` en `app/Cargo.toml` no se usa (advertencia `unused_dependencies` de cargo, ya presente en `main`).
  - Revisión con `craft-backend:architecture-reviewer`: sin hallazgos en el diff ni hallazgos nuevos. Lo que señaló de otras rebanadas ya figura en el inventario y no se corrigió aquí:
    - el sufijo `Port` en `SpyPasswordHasherPortAdapter` (HEX-5.1, R9);
    - los asertos sobre `err.code()` (ERR-2.6);
    - la falta de `fails_when_token_generation_fails` (ERR-2.8);
    - `if !account.can_authenticate()` en el handler (HEX-7.2);
    - `ErrorLayout`/`SystemError`, que siguen mezclando los dos árboles (HEX-8.3, ERR-1.1, ERR-2.2; R5 a R19).

**R4. Borrar código muerto del kernel.** *Hecha (ver §5.8).*
- **IDs:** HEX-8.4, HEX-8.7.
- **Toca:** `shared/src/domain/domain_error.rs`, `shared/src/infrastructure/infrastructure_error.rs` y `shared/src/application/common_application_error.rs`.
- **Línea base: −2** (`hex_8_kernel_traits`: `domain_error.rs` e `infrastructure_error.rs`).
- **Tamaño:** XS.
- **Resultado:**
  - Commit `80c00e4`. Se borraron los tres archivos, los `pub mod` de los tres `mod.rs` y los `pub use DomainError` y `pub use InfrastructureError`.
  - Antes de borrar se buscó cada nombre (tipo, trait y módulo) en todo el workspace, tests incluidos. Nadie los usaba: el único rastro era un `use` y un `CommonApplicationError::Infrastructure` **comentados** en `list_public_communities.rs` (líneas 51 y 128), que no compilan y desaparecen con el archivo en R18. No se tocaron.
  - Ningún módulo quedó vacío: `shared/src/domain/` conserva `aggregate_root`, `domain_event`, `entity`, `model` y `value_object`; `shared/src/infrastructure/` conserva `http`; `shared/src/application/` conserva `auth_context` y `ports`. Ninguna otra entrada de la línea base cambió de ruta.
  - El rojo fue el trinquete: `hex_8_kernel_traits_are_model_markers_or_listed_capabilities` falló con "Ya no violan: borrar" para `domain_error.rs` e `infrastructure_error.rs`. Se borraron las 2 líneas de `baseline.txt`; quedan 2 (`token_verifier.rs` y `error.rs`).
  - Ningún test cambió, salvo la línea base. `app/tests/` y `account_lifecycle.rs` quedaron intactos. 188 tests, fmt y clippy en verde.
  - Revisión con `craft-backend:architecture-reviewer`: sin hallazgos en el diff ni hallazgos nuevos. Lo que señaló de otras rebanadas ya figura en el inventario y no se corrigió aquí:
    - las referencias comentadas en `list_public_communities.rs`, dentro del bloque de tests comentado (TDD-1.1, R18);
    - las 2 entradas que quedan de `hex_8_kernel_traits` (`token_verifier.rs` en R10, `error.rs` en R19);
    - `anyhow` sin usar en `app/Cargo.toml` (advertencia de cargo, anotada en R3).

**R5. Primitivas del kernel en `shared`.** *Hecha (ver §5.8).*
- **IDs:** RS-2.1 (parcial).
- **Objetivo:** `shared/src/{fault,failure,id}.rs` copiados de `template/kernel` (`Fault`, `Failure<E>`, `IdGenerator`, `FaultReporter`), con la misma disposición que `kernel/src`, para que R29 sea un `git mv`. Se agrega un puente temporal `SystemError::Unexpected(Fault)`, que los dos mappers responden como 500 con el cuerpo de D7: `{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}`. Se borra en R19.
- **Línea base:** sin cambios. Los traits nuevos son capacidades de la lista.
- **Rojo:** los tests del template `display_never_exposes_the_technical_detail`, `question_mark_converts_technical_errors_into_fault` y `map_expected_translates_only_the_expected_tree` (no compilan). Además, un test del mapper: `unexpected_fault_maps_to_internal_server_error`, que afirma el status y el cuerpo exacto de D7.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** S.
- **Resultado:**
  - Commit `e76c162`: `fault.rs`, `failure.rs` e `id.rs` copiados de ai-harness `97a8f2973efd05642f40df39cd8c0671a4fc658c` (`stacks/rust/template/kernel/src`), con sus tests y la misma disposición plana. La única diferencia con el template es que el `rustfmt.toml` del repo (`imports_granularity = "Crate"`) une los dos `use std::…` de `fault.rs`. `lib.rs` declara los módulos privados y reexporta `Failure`, `Fault`, `FaultReporter` e `IdGenerator`, como el template.
  - `Clock` no se copió: el template no lo define (su README dice "aún sin uso en la plantilla"). Decidido por el usuario: entra en R21, con su primer uso.
  - `shared/Cargo.toml` no cambia. `hex_8_kernel_depends_only_on_std` y `hex_8_kernel_traits_…` siguen con sus entradas (1 y 2); el segundo revisa 8 traits en lugar de 6.
  - Commit `29890cb`: `SystemError::Unexpected(Fault)`. Los dos `error_mapper.rs` lo responden con `unexpected()`, que da 500 con el cuerpo de D7. El detalle del `Fault` va a stderr con `eprintln!` (no hay `tracing` en el proyecto), marcado como interino hasta R26 (ERR-4.7). Ese registro no tiene test.
  - Commit `446726b`: a pedido del revisor, el código de D7 vive en `SystemError::UNEXPECTED_CODE`, que usan `code()` y los dos mappers. Así el test del mapper cubre también la rama de `code()`. Se comprobó que el test falla si cambia el literal.
  - Rojo: con solo los tests del template, `shared` no compila (faltan `Fault` y `Failure`). `unexpected_fault_maps_to_internal_server_error` (iam y communities) no compila sin la variante. Con un brazo provisorio que respondía 500 sin cuerpo, falla por el cuerpo (`left: ""`). El test lee el cuerpo con un `poll` y `Waker::noop()` de std, para no agregar `tokio` a las dev-dependencies de los contextos (`hex_2_core` las cuenta).
  - 194 tests (188 + 4 del template + 2 de los mappers), fmt y clippy en verde. `app/tests/` y `account_lifecycle.rs` quedaron intactos.
  - Revisión con `craft-backend:architecture-reviewer`: aplicado el hallazgo de `code()` (arriba). Lo demás queda para otras rebanadas:
    - el `eprintln!` escribe solo el `Display` del error de primer nivel, sin la cadena de `source()`; `StderrFaultReporterAdapter` debería recorrerla (R26);
    - `unexpected()` está duplicado en los dos mappers (R19 retira los mappers; R26/R30 dan el borde común);
    - el `Debug` de `Fault` imprime el `source` (igual que el template): un `{:?}` sobre `SystemError` expondría el detalle. Hoy nada en producción lo hace.

**R6. Herencia del workspace.** *Hecha (ver §5.8).*
- **IDs:** RS-1.3 y RS-1.4 (quitar `anyhow` de `app`).
- **Toca:** todos los `Cargo.toml`, con `[workspace.package]` y las dependencias externas en `[workspace.dependencies]`. No cambia ninguna ruta.
- **Línea base:** sin cambios.
- **Riesgo:** el `Cargo.lock`; se verifica con `--locked`.
- **Tamaño:** S.
- **Resultado:**
  - Commit `991e672`. `[workspace.package]` con `version` y `edition`; los seis crates los heredan (`publish = false` sigue en `architecture`). Las 17 dependencias externas pasan a `[workspace.dependencies]`, con las features que comparten todos sus usuarios. La única feature propia de un crate es `util` de `tower`, en las `[dev-dependencies]` de `app`.
  - Cuando dos crates pedían requisitos distintos (`serde` `"1"` y `"1.0.228"`, `tower` `"0.5"` y `"0.5.3"`), el workspace toma el más específico. Ya era el que fijaba la resolución, así que la versión resuelta no cambia.
  - Verificado contra `main`: el `Cargo.lock` solo pierde `anyhow`; `cargo tree --workspace -e features,normal,dev,build` es idéntico salvo `anyhow`, y las claves de `[dependencies]`, `[dev-dependencies]` y `[build-dependencies]` de cada manifiesto, que es lo único que leen los tests `hex_1_*`, `hex_2_*` y `hex_8_*` (`Crate::dependencies`), también: solo falta `anyhow`. `dep.workspace = true` conserva la clave, así que ningún test perdió detección.
  - Sin rojo: es un refactor de manifiestos sin cambio de comportamiento, y la línea base no cambia. Desaparece el aviso `unused dependency` de `anyhow`.
  - `contexts/iam/Cargo.lock` (versionado, §4) no se tocó.
  - Revisión con `craft-backend:architecture-reviewer`: sin hallazgos en el diff. Señaló dos huecos de verificación (VER-1), que no son violaciones actuales y quedan fuera de esta rebanada porque cerrarlos exige tests nuevos en `tools/architecture`:
    - `Crate::dependencies` (tests/common/mod.rs:69) lee solo la clave y no resuelve `package =`. Con la herencia, un renombre en la raíz (`foo = { package = "axum" }`) más `foo.workspace = true` en domain o application pasaría `hex_2_*`. Hoy no hay ningún `package =` en el workspace. Corrección propuesta: resolver `workspace = true` contra `[workspace.dependencies]` de la raíz y usar su `package`, o leer `cargo metadata`.
    - Ningún test exige `workspace = true` en las dependencias externas de los miembros, así que un crate nuevo puede volver a fijar una versión propia (RS-1.3) sin que nada falle.
    - `iam` en `contexts/communities/Cargo.toml`, que ya figura como HEX-1.1 (R16).

#### Fase 2: puertos salientes, uno por rebanada

Patrón de cada rebanada:
- se quita el sufijo `Port`;
- la firma pasa a `Result<Option<T>, Fault>` o `Result<T, Fault>` y se borra el error de puerto;
- el doble sale del archivo del puerto a `application/testing.rs` (`#[cfg(any(test, feature = "testing"))]`), con un nombre de TDD-2 y estado real;
- se crea la batería `application/testing/contracts/<puerto>.rs` y corre contra el doble y contra el adaptador;
- el adaptador toma el nombre `{Tecnología}{Concepto}Adapter` en un archivo con ese nombre, que reemplaza al viejo;
- los `expect` de los mutex pasan a `"invariante: mutex sin envenenar"`.

Una falla del puerto llega al handler todavía no migrado como `SystemError::Unexpected` (R5).

**R7. `CommunityRepository`.**
- **IDs:**
  - HEX-5.1 y HEX-5.2 (communities);
  - ERR-1.7, ERR-3.2 y ERR-3.3 (communities);
  - TDD-2.1, TDD-2.2 y TDD-2.3 (communities);
  - TDD-1.2 y RS-2.4 (communities);
  - TDD-2.5 y RS-2.2 (`get_public_list`), ya cerrados por R18 (D3 decidida: eliminar). R7 borra además `find_by_id` del puerto, del doble y del adaptador, porque no lo usa nadie. La batería cubre solo lo que queda: `save` y `find_by_slug`.
- **Precondición:** R18 hecha. Si no, `get_public_list` todavía tiene un llamador y su `todo!()` sigue en el adaptador.
- **Línea base: −3:**
  - `hex_5_outbound` y `hex_5_adapters` de `ports/outbound/community_repository.rs`;
  - `hex_5_adapters` de `in_memory/community_repository.rs`.
- **Rojo:**
  - batería `saves_and_finds_by_slug` contra el doble actual, que no guarda nada;
  - test de handler `fails_with_unexpected_when_repository_fails`.
- **Contrato HTTP: cambia (sin cobertura en caracterización).** El 500 por falla del repositorio deja de llevar `COMMUNITIES_REPOSITORY_ERROR` y pasa al cuerpo interino de D7 (`INTERNAL_ERROR`).
- **Tamaño:** M.

**R8. `AccountRepository`.**
- **IDs:**
  - HEX-5.1 y HEX-5.2 (iam);
  - ERR-1.7, ERR-3.2 y ERR-3.3 (iam);
  - TDD-2.1, TDD-2.2 y TDD-2.3 (iam);
  - TDD-2.4 (el email del doble);
  - TDD-1.2 y RS-2.4 (iam).
- **Línea base: −3:** `hex_5_outbound` y `hex_5_adapters` de `ports/outbound/account_repository.rs`, y `hex_5_adapters` de `in_memory/account_repository.rs`.
- **Rojo:** batería `finds_by_email_what_was_saved`. Hoy falla contra el doble: `save` no guarda y `find_by_email` devuelve otra cuenta.
- **Caracterización:** lifecycle se adapta (nombre del puerto y del adaptador, `find_by_email` con `Result`).
- **Contrato HTTP:** desaparece `IAM_ACCOUNT_REPOSITORY_ERROR`, igual que en R7.
- **Tamaño:** M.

**R9. `PasswordHashingService`.**
- **IDs:** HEX-5.1, HEX-5.2 (Argon2), RS-2.4 (Argon2, a `Fault`), TDD-2.3, TDD-2.4 y TDD-1.3 (Argon2).
- **Cambios:**
  - `hash` pasa a `Result<String, Fault>` y `verify` a `Result<bool, Fault>`;
  - un `FixedPasswordHashingAdapter` coherente, con el hash derivado de la contraseña;
  - el `SpyPasswordHasherPortAdapter` de R2 sale de los tests de `authenticate_account.rs` a `application/testing.rs` como `SpyPasswordHashingAdapter`;
  - el señuelo de R2: con `hash` falible, el composition root propaga el `Fault` al arrancar. Hay que decidir qué hace el handler si el `verify` del señuelo devuelve `Fault`.
- **Línea base: −2:** `hex_5_outbound` y `hex_5_adapters` de `password_hasher.rs`. Y **−1:** `hex_5_adapters` de `argon2_password_hasher.rs`.
- **Rojo:**
  - batería `verify_accepts_the_hashed_password_and_rejects_another`, contra el doble actual (compara texto plano con el hash) y contra Argon2;
  - `corrupt_hash_is_a_fault` en Argon2.
- **Tests de handler:** el test de éxito de authenticate deja de mandar el hash como contraseña. Se corrige un doble infiel, no se ajusta un test para que pase.
- **Tamaño:** M.

**R10. `AccessTokenService` y la query `VerifyAccessToken` en iam.**
- **IDs:**
  - HEX-1.5, HEX-8.5, HEX-5.1 y HEX-5.2 (JWT);
  - ERR-3.1, ERR-3.3 (token);
  - TDD-2.1, TDD-1.3 (JWT) y TDD-2.2.
- **Cambios:**
  - `TokenGeneratorPort` y `TokenVerifier` se fusionan en un puerto `AccessTokenService`: `issue(&str) -> Result<String, Fault>` y `verify(&str) -> Result<AccessTokenVerdict, Fault>`, con `ports/outbound/dtos/access_token_payloads.rs`;
  - se crean `VerifyAccessTokenUseCase` y `VerifyAccessTokenHandler` en `queries/verify_access_token/`;
  - el middleware de `app` depende de `Arc<dyn VerifyAccessTokenUseCase>` (sigue en `app` hasta R28);
  - se borran `shared/src/application/ports/outbound/{token_verifier,error}.rs`;
  - `jwt_service/` pasa a `jwt_access_token.rs` y se quita el `module_inception`.
- **Línea base: −4:**
  - `hex_5_outbound` y `hex_5_adapters` de `token_generator.rs`;
  - `hex_5_adapters` de `jwt_service.rs`;
  - `hex_8_kernel_traits` de `token_verifier.rs`.
- **Rojo:**
  - `expired_token_is_an_invalid_verdict`: hoy es `Err`;
  - tests del handler (`fails_…`, `returns_account_id_for_valid_token`).
- **Caracterización:** sin cambios. Los 401 siguen iguales. Lifecycle se adapta.
- **Contrato HTTP:** desaparece `IAM_TOKEN_GENERATOR_ERROR`.
- **Tamaño:** M/L.

> **Punto de replanificación P1 (después de R10).** Ver §5.9. Termina la fase de puertos: se revisa si el patrón de las fases 2 y 3 sigue sirviendo antes de migrar los contratos.

#### Fase 3: contratos por caso de uso, en el lugar

Patrón de cada rebanada:
- `commands|queries/<caso>/{command,response,error,handler}.rs`, con `{Verb}{Noun}Handler` y sin `module_inception`;
- hojas planas en `application/errors.rs` (`Validation { field }`, `Conflict { resource }`, `NotFound { resource }`…), con `From<Leaf>` hacia el contrato;
- el `UseCase` devuelve `Result<_, Failure<{Verb}{Noun}Error>>`;
- el handler traduce a mano los errores de dominio a hojas (ERR-2.2);
- en el HTTP, `infrastructure/http/<caso>.rs` hace un `match` exhaustivo, sin comodín, con los códigos y mensajes de `infrastructure/http/codes.rs` (byte a byte los de hoy);
- los tests de handler pasan a `matches!` sobre la variante.

En todas, el contrato HTTP se mantiene.

**R11. `RegisterAccount`.**
- **IDs:**
  - HEX-4.1 (register), ERR-2.1 (register), ERR-2.2 (parcial);
  - ERR-2.6 (register), ERR-2.7;
  - ERR-4.4 y ERR-4.5 (register).
- **Cambios:** el `HashedPasswordError` pasa a `Fault` (§4).
- **Línea base: −1** (`hex_4_handlers` de `register_account.rs`).
- **Rojo:**
  - `fails_with_conflict_on_taken_email` y `…_username` (variante);
  - `fails_when_password_is_too_short` (falta hoy);
  - test de endpoint con `StubRegisterAccountUseCase` que devuelve `Unexpected`, y responde 500.
- **Caracterización:** lifecycle se adapta (nombre del handler).
- **Tamaño:** M.

**R12. Unicidad sobre el valor normalizado.**
- **IDs:** HEX-7.1.
- **Cambios:** se construyen `Email` y `Username` antes de consultar al repositorio.
- **Caracterización previa (primer commit del PR, sin tocar producción):** en `app/tests/auth_api.rs`, `register_with_invalid_username_and_taken_email_returns_409`, marcado `CURRENT BEHAVIOR`. Registra `ana`/`ana@example.com` y después manda `ab` (username inválido) con `ana@example.com`. Afirma 409 `IAM_ACCOUNT_EMAIL_ALREADY_EXISTS`, porque hoy `RegisterAccount` busca el conflicto con el texto crudo antes de que la factory valide el formato (register_account.rs:37-51). Tiene que pasar en verde contra el código actual.
- **Rojo:**
  - `fails_with_conflict_when_email_differs_only_in_case`, en el handler y en HTTP;
  - el cambio de producción pone en rojo el test de caracterización anterior. Se reemplaza por `register_rejects_invalid_username_before_checking_taken_email`, que afirma 400 `IAM_INVALID_USERNAME`: primero el formato, después el conflicto.
- **Contrato HTTP: cambia.** Se reemplazan `register_accepts_same_email_with_different_casing` y `register_accepts_same_username_with_surrounding_spaces` por los mismos casos con 409. También cambia "username inválido + email tomado", de 409 a 400 (ver §5.5). Hay que mantener el orden "contraseña, después formato, después conflicto" (`register_validates_password_before_username_and_email` y `register_checks_email_before_username_when_both_are_taken`).
- **Riesgo:** cambia el orden entre validar el formato y detectar el conflicto. Ahora lo fija el test de caracterización de arriba, así que el cambio queda a la vista en el diff de `app/tests/`.
- **Tamaño:** S.

**R13. `VerifyAccount`.**
- **IDs:** HEX-4.1, ERR-2.1, ERR-2.6, ERR-2.9 y ERR-1.4 (`expect("invariante: Registered→Active")`).
- **Contrato:** `VerifyAccountError { NotFound, Validation, InvalidVerification }`. La variante de verificar dos veces depende de D1.
- **Línea base: −1** (`verify_account.rs`).
- **Rojo:** `fails_when_code_is_out_of_range` y `fails_when_repository_fails`. Además, un test de endpoint para la falla del repositorio.
- **Caracterización:** lifecycle se adapta (pasa de `code()` a la variante).
- **Tamaño:** S/M.

**R14. `AuthenticateAccount`.**
- **IDs:** HEX-4.1, HEX-4.4, ERR-2.1, ERR-2.3, ERR-2.6, ERR-2.8 y HEX-7.2.
- **Cambios:**
  - `account.authenticate() -> Result<(), CannotAuthenticate>` en el agregado;
  - `AuthenticateAccountError { LoginFailed, CannotAuthenticate }`, sin `NotFound` gracias a R2;
  - se borra `application/errors/authenticate_account.rs`.
- **Línea base: −1.**
- **Rojo:**
  - dominio: `registered_account_cannot_authenticate`;
  - handler: `fails_when_token_generation_fails`, que da `Unexpected`;
  - un test de endpoint 500.
- **Tamaño:** M.

**R15. `IdentifyAccount`.**
- **IDs:** HEX-4.1, ERR-2.1 y ERR-2.6.
- **Línea base: −1.**
- **Rojo:** `fails_with_not_found_for_unknown_identifier`, sobre la variante.
- **Caracterización:** `identify_with_different_email_casing_returns_404` no cambia (ver D10).
- **Tamaño:** S.

**R16. Cortar `communities → iam`.**
- **IDs:** HEX-1.1, HEX-1.2, HEX-1.3 y HEX-1.4.
- **Cambios:**
  - un `OwnerId` propio en `domain/model/community/owner_id.rs`;
  - los códigos propios de communities pasan a `infrastructure/http/codes.rs`;
  - se quita `iam` de `contexts/communities/Cargo.toml`.
- **Línea base: −1** (`hex_1` de `contexts/communities/Cargo.toml`): **cierra `hex_1_dependencies_point_inward`.**
- **Rojo:** tests de dominio `owner_id_rejects_non_uuid` y `…_nil`, según D2.
- **Contrato HTTP:** con la opción por defecto de D2, ninguno. Se conservan los strings `IAM_INVALID_ACCOUNT_ID(_FORMAT)`.
- **Riesgo:** `create_with_non_uuid_subject_returns_400` y `create_with_nil_uuid_subject_returns_400`.
- **Tamaño:** S/M.

**R17. `CreateCommunity`.**
- **IDs:**
  - HEX-4.1, HEX-4.2 (`owner_id: String` en el comando);
  - ERR-2.1, ERR-2.4, ERR-2.6 y ERR-2.10;
  - HEX-1.8 (los códigos salen de domain);
  - ERR-4.4 y ERR-4.5 (communities).
- **Línea base: −1** (`create_community.rs`).
- **Rojo:**
  - `fails_when_owner_id_is_invalid` (handler);
  - un test de endpoint para la falla del repositorio.
- **Caracterización:** `create_checks_slug_uniqueness_before_subject_format` fija el orden: primero el slug, después el owner.
- **Tamaño:** M.

**R18. Eliminar `ListPublicCommunities` (D3 decidida: eliminar).**
- **IDs:** TDD-1.1, HEX-4.1 (list), HEX-4.3, ERR-1.6, HEX-6.2, TDD-2.5 y RS-2.2 (`get_public_list`).
- **Cambios:** se borran el caso (`queries/list_public_communities/`), su puerto de entrada, `get_public_list` del puerto `CommunityRepositoryPort`, del doble y del adaptador en memoria (con su `todo!()`), y `AuthContext` (parte de HEX-8.6).
- **Orden:** ya no depende de la fase 3. Se adelanta y va **antes de R7**, después de R1 (ver §5.4). Conserva el número para no renumerar las referencias.
- **Línea base: −1** (`hex_4_handlers` de `list_public_communities.rs`). Ya no cierra `hex_4_use_case_implementations_are_handlers`: lo cierra la última de R11, R13 a R15 y R17.
- **Tests:** sin rojo nuevo; el rojo es la línea de la línea base. Se borran los tests comentados (TDD-1.1).
- **Caracterización:** `there_is_no_route_to_list_communities` no cambia.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** S.

**R19. Retirar `SystemError`.**
- **IDs:** ERR-1.1, RS-2.1, ERR-2.2, HEX-8.3 y ERR-4.4 (resto).
- **Cambios:**
  - se borran `shared/src/error.rs` y `ErrorLayout`;
  - los errores de dominio quedan solo con `Debug` y `Display`;
  - se borran los `error_codes.rs` de domain y de application;
  - se borran los dos `error_mapper.rs` y sus tests, que ya quedaron reemplazados por los `fold`.
- **Línea base: −1:** **cierra `hex_8_kernel_traits_are_model_markers_or_listed_capabilities`.**
- **Tests:** refactor puro, sin rojo nuevo.
- **Caracterización:** lifecycle queda sin `SystemError`.
- **Tamaño:** M.

> **Punto de replanificación P2 (después de R19).** Ver §5.9. Ya no queda `SystemError`: se confirma que el árbol de errores quedó como pide ERR antes de tocar el dominio.

#### Fase 4: dominio

**R20. Alta y restauración de `Account`.**
- **IDs:**
  - HEX-7.5, HEX-7.6 (iam), HEX-2.2 y HEX-2.3;
  - ERR-1.5, HEX-7.7 y ERR-2.5 (se borran `reconstitute` y `from_str`, que no tienen uso, junto con sus tests);
  - RS-2.2 (`account_id.rs` y `account_status.rs`).
- **Cambios:**
  - `Account::create`/`restore` con value objects, y `new` privado;
  - `AccountId` sobre `String`, con el id que llega de `IdGenerator` (`UuidIdGeneratorAdapter` en infra);
  - el código de verificación sale de un puerto `VerificationCodeService` (`RandVerificationCodeAdapter`);
  - se quitan los `should_implement_trait` y el `module_inception` de `model/account`.
- **Línea base:** sin cambios.
- **Rojo:**
  - `create_uses_the_given_id_and_code` (dominio);
  - handler con `FixedIdGeneratorAdapter`: `registers_with_the_generated_id`;
  - batería del generador de códigos: `generates_six_digit_codes`.
- **Riesgo:** `register_returns_201_with_uuid…` y `verification_code_is_six_digits`.
- **Tamaño:** M/L.

**R21. Agregado inmutable y eventos.**
- **IDs:**
  - HEX-7.3, HEX-7.4, HEX-8.9, HEX-8.2 y HEX-2.4;
  - *(S1)* HEX-2.1 (parcial: `chrono` en `contexts/iam/Cargo.toml`) y HEX-8.1 (parcial: `chrono` en `shared/Cargo.toml`);
  - RS-2.2 (`account.rs` y `account_event.rs`).
- **Cambios:**
  - `confirm(&self, …) -> Result<Self, …>`;
  - `AggregateRoot` como marcador;
  - `DomainEvent` sin `chrono`;
  - el trait `Clock` en `shared/src/clock.rs`, solo `std`, que R5 dejó fuera porque el template no lo definía. *(S1: ai-harness `v1.0.0-rc.1` ya lo trae en `kernel/src/clock.rs`, `fn now(&self) -> SystemTime`. Se copia tal cual, con el adaptador `SystemClockAdapter` de `infrastructure/system`; ver §5.10.)* Entra aquí con su primer uso y un test que lo exija;
  - eventos implementados o retirados, según D4;
  - *(S1)* `chrono` sale de `contexts/iam/Cargo.toml` y de `shared/Cargo.toml`: después de este cambio, ninguno de los dos lo usa.
- **Línea base (S1): −3:** `hex_2_core` `contexts/iam/Cargo.toml#dependencies.chrono` y `shared/Cargo.toml#dependencies.chrono`, y `hex_8_kernel_depends` `shared/Cargo.toml#dependencies.chrono`.
- **Rojo:**
  - `confirm_returns_the_active_account_and_leaves_the_original_untouched`;
  - con eventos: `create_records_registered`.
- **Tamaño:** M.

**R22. Dominio de `communities`.**
- **IDs:** HEX-2.6, HEX-7.6 (communities), HEX-7.8 y HEX-7.9 (según D4).
- **Cambios:**
  - `CommunityId` sobre `String`, con `IdGenerator`;
  - `MembershipPolicy` pasa a `model/community/`;
  - se quitan los `allow`.
- **Rojo:** handler `creates_with_the_generated_id`.
- **Riesgo:** `create_returns_200_with_id…`, que exige un UUID de 36 caracteres; lo cumple el adaptador UUID.
- **Tamaño:** S/M.

#### Fase 5: puertos async (paralelizables entre sí)

**R23. iam async.** **R24. communities async.**
- **IDs:** RS-3.1.
- **Cambios:**
  - `BoxFuture` en `shared/src/future.rs`;
  - todos los puertos de entrada y de salida del contexto;
  - `block_on` de std en `application/testing.rs` para los tests de application;
  - Argon2 con `spawn_blocking` en el adaptador.
- **Rojo:** batería async del adaptador Argon2 bajo `#[tokio::test]` (infra).
- **Tests:** refactor sin cambio de comportamiento.
- **Caracterización:** lifecycle se adapta (`.await`).
- **Tamaño:** M cada una.

#### Fase 6 y 7: kernel y estructura

**R25. `shared` sin dependencias.**
- **IDs:** HEX-8.1 (el resto: `serde`; `chrono` sale en R21, desde S1). HEX-8.2 ya quedó resuelto en R21.
- **Cambios:** `ApiErrorResponse` se copia temporalmente a cada `infrastructure/http`; R30 lo elimina.
- **Línea base: −2.** *(S1: con las claves por dependencia, son `hex_8_kernel_depends` y `hex_2_core` de `shared/Cargo.toml#dependencies.serde`. Las de `chrono` las cierra R21.)* **Cierra `hex_8_kernel_depends_only_on_std`.**
- **Tamaño:** S.

**R26. `edge/http` e `infrastructure/stderr`.**
- **IDs:** ERR-4.7 (parcial).
- **Cambios:**
  - crates nuevos con `Problem`, `unexpected()`, `AuthenticatedAccount` y `StderrFaultReporterAdapter`, más los tests del template;
  - `CatchPanicLayer` en `app/src/server.rs`.
- **Línea base:** sin cambios. Solo depende de ellos `app`, que es Bootstrap.
- **Rojo:**
  - `serializes_as_problem_json_with_extension_code`;
  - `unexpected_reports_the_fault_and_hides_the_detail`;
  - un test de `app`: `a_panicking_route_returns_500`.
- **Contrato HTTP: cambia (sin cobertura previa).** Un panic responde 500 en lugar de cortar la conexión.
- **Tamaño:** M.

> **Punto de replanificación P3 (después de R26, antes de R27).** Ver §5.9. R26 corre en paralelo desde R5, así que P3 se hace cuando están hechas R22, R24, R25 y R26, las precondiciones de la separación. Es la última revisión antes de las rebanadas L.

**R27. Separar `communities` en crates por capa.**
- **IDs:**
  - RS-1.1 (parcial), HEX-2.5;
  - HEX-1.9 (communities);
  - HEX-6.1 (communities: `CreateCommunityHttpRequest/Response`);
  - ERR-4.8 (`Option<Extension<AuthenticatedAccount>>`);
  - HEX-8.6 (se borra `VerifiedIdentity`; el middleware de `app` inserta `AuthenticatedAccount`).
- **Estructura:** `contexts/communities/{domain,application,infrastructure,api}`, con `CommunitiesApi`.
- **Precondición del trinquete:** de `contexts/communities/src`, en la línea base solo quedan `requests/create.rs` y `responses/created.rs`. Se renombran en este mismo PR.
- **Línea base: −7** *(S1; antes −4)*: las 4 `hex_2_core` de `contexts/communities/Cargo.toml` (`axum`, `http`, `serde`, `uuid`), la `hex_2_frameworks` (`axum`, n = 3), porque el manifiesto desaparece, y los dos `hex_6`.
- **Rojo:** `communities-api/tests/create_community.rs`: `responds_401_without_authenticated_account`.
- **Riesgo:** ~~el test de contratos HEX-4 se activa en este PR~~. Desde S1 ya actúa en legacy. El riesgo que queda es que un crate nuevo (`communities-application`) declare más dependencias de las que pide su capa.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** L (sobre todo, archivos movidos).

**R28. Separar `iam` y crear `IamApi::protect`.**
- **IDs:**
  - RS-1.1, HEX-2.1;
  - HEX-1.6, HEX-1.7, HEX-1.9;
  - HEX-6.1 (iam).
- **Cambios:**
  - el middleware sale de `app` y va a `iam-api`;
  - `app` compone `iam.protect(communities.protected())`;
  - lifecycle se muda a `contexts/iam/infrastructure/tests/`.
- **Precondición:** de `contexts/iam/src`, en la línea base solo quedan los 7 `hex_6`.
- **Línea base: −14** *(S1; antes −9)*: las 6 `hex_2_core` de `contexts/iam/Cargo.toml` (`argon2`, `axum`, `jsonwebtoken`, `rand`, `serde`, `uuid`; `chrono` sale en R21), la `hex_2_frameworks` (`axum`, n = 3) y los 7 `hex_6`. **Cierra `hex_2_core…`, `hex_2_server_frameworks…` y `hex_6_transport…`.**
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

**R30. Problem Details.**
- **IDs:** ERR-4.6, ERR-4.7 (resto: los `fold` usan `unexpected(fault, reporter)`) y HEX-8.8.
- **Cambios:** todas las respuestas de error y los 401 pasan a `application/problem+json`, con `code` como miembro de extensión. Se borra el `ApiErrorResponse` interino.
- **Rojo:** tests HTTP con el formato nuevo.
- **Contrato HTTP: cambia todo el cuerpo de error.** Se modifican `assert_error` (`app/tests/common/mod.rs`) y las aserciones `raw_body == ""` de los 401 (ver D5).
- **Tamaño:** M.

### 5.4 Dependencias y paralelismo

```
R1 ─┬─ R2 ─────────────────────────────┐
    ├─ R3, R4, R6, R18   (paralelas)   │
    └─ R5 ─┬─ R7 (+R18) ──────┬─ R17 ──┤
           │        R16 ──────┘        │
           ├─ R8 ─ R9 ─ R10 ─[P1]─ R11 ─ R12
           │             └─ R13, R14(+R2), R15 ┴─ R19 ─[P2]─┬─ R20 ─ R21 ─ R23 ─┐
           │                                                └─ R22 ─ R24 ───────┤
           └─ R26 (desde R5, paralela a las fases 3 a 5)                        │
                                     R25 (tras R19, R21) ───────────────────────┤
                                           [P3] ─ R27 (R22, R24, R25, R26) ─ R28 (R10, R23) ─ R29, R30
```

- R8, R9 y R10 se pueden hacer en paralelo, pero tocan los mismos handlers de iam. Conviene ir en serie para no tener conflictos.
- R18 (eliminar `ListPublicCommunities`, D3) no depende de nada más que R1 y va antes de R7.
- R7 y R16 son paralelas a toda la rama iam.
- `[P1]`, `[P2]` y `[P3]` son los puntos de replanificación de §5.9. No se arranca la rebanada siguiente sin cerrar el punto.
- R13, R14 y R15 son paralelas entre sí después de R11. R14 necesita R2, R9 y R10.
- R23 y R24 son paralelas. R26 se puede hacer desde R5 en adelante.

### 5.5 Cambios de contrato HTTP (todos explícitos)

| Rebanada | Cambio | Tests de caracterización que cambian |
|---|---|---|
| R2 (hecho) | Sign-in de un usuario inexistente: de 404 a 401 `IAM_LOGIN_FAILED` | `authenticate_unknown_username_returns_404` → `authenticate_unknown_username_returns_401_login_failed`; `authenticate_by_email_returns_404` → `authenticate_by_email_returns_401` |
| R5, R7, R8, R10 | 500 de infraestructura con `{"code":"INTERNAL_ERROR",…}` (D7), sin `*_REPOSITORY_ERROR` ni `IAM_TOKEN_GENERATOR_ERROR` | ninguno (solo los tests del mapper) |
| R12 | Duplicado por mayúsculas o espacios: de 201 a 409 | `register_accepts_same_email_with_different_casing`, `register_accepts_same_username_with_surrounding_spaces` |
| R12 | Username inválido + email tomado: de 409 `IAM_ACCOUNT_EMAIL_ALREADY_EXISTS` a 400 `IAM_INVALID_USERNAME` | `register_with_invalid_username_and_taken_email_returns_409` (lo agrega R12 en su primer commit) pasa a `register_rejects_invalid_username_before_checking_taken_email` |
| R16 | Solo si D2 elige códigos propios | `create_with_non_uuid_subject_returns_400`, `create_with_nil_uuid_subject_returns_400` |
| R26 | Panic: 500 en lugar de conexión cortada | ninguno |
| R28 | Solo si cambia el fallback: ruta desconocida sin token, de 401 a 404 | `unknown_route_without_token_returns_401` |
| R30 | Problem Details en todos los errores y en los 401 | `assert_error`, los `raw_body == ""` |

### 5.6 Cierre de cada test de arquitectura

Desde S1 la columna cuenta entradas con el formato nuevo (una por dependencia en los `Cargo.toml`). Entre paréntesis va el total de violaciones que admiten, sumando las `n`.

| Test | Entradas hoy (S1) | Justo antes de S1 | Queda en cero en |
|---|---|---|---|
| `err_1_domain_errors_do_not_implement_std_error` | 0 | 0 | R3: en cero (eb30039) |
| `err_1_application_errors_do_not_implement_std_error` | 0 | 0 | R3: en cero (eb30039) |
| `hex_1_dependencies_point_inward` | 1 (4) | 1 | R16 |
| `hex_5_outbound_ports_are_named_by_role` | 4 | 4 | R10 (R7 a R10) |
| `hex_5_adapters_are_named_by_technology_and_concept` | 8 | 8 | R10 (R7 a R10) |
| `hex_4_use_case_implementations_are_handlers` | 6 | 6 | R17 (R18, R11, R13 a R15, R17) |
| `hex_8_kernel_traits_are_model_markers_or_listed_capabilities` | 2 | 2 | R19 (R4, R10, R19); R4: −2 (80c00e4), quedan 2 |
| `hex_8_kernel_depends_only_on_std` | 2 | 1 | R25 (R21 −1 `chrono`, R25 −1 `serde`) |
| `hex_2_core_depends_only_on_std_and_the_kernel` | 13 (24) | 3 | R28 (R21 −2 `chrono`, R25 −1, R27 −4, R28 −6) |
| `hex_2_server_frameworks_only_at_the_edge` | 2 (6) | 2 | R28 (R27, R28) |
| `hex_6_transport_types_carry_the_transport_name` | 9 | 9 | R28 (R27, R28) |
| `hex_4_use_case_contracts_do_not_expose_domain_types` | 0 | 0 | Desde S1 actúa también en legacy (17 tipos de contrato) |

En total quedan 47 entradas (65 violaciones), que desaparecen entre R7 y R28. Después de R29, `baseline.txt` queda solo con la cabecera.

Suma por rebanada desde S1: R7 −3, R8 −3, R9 −3, R10 −4, R11 −1, R13 −1, R14 −1, R15 −1, R16 −1, R17 −1, R18 −1, R19 −1, **R21 −3**, R25 −2, **R27 −7** y **R28 −14**: 47. Las 36 de antes de S1 se repartían entre las mismas rebanadas, con R25 −2, R27 −4 y R28 −9, y sin R21.

**Conciliación (R1).** El encargo hablaba de 56 entradas, pero `tools/architecture/baseline.txt` tiene 55 (63 líneas, de las que 8 son comentario). La columna "Entradas hoy" coincide test por test con `grep -v '^#' baseline.txt | cut -d' ' -f1 | sort | uniq -c`. No falta ninguna entrada: se corrige el conteo, no se asigna ninguna. La suma por rebanada también da 55: R3 −17, R4 −2, R7 −3, R8 −3, R9 −3, R10 −4, R11 −1, R13 −1, R14 −1, R15 −1, R16 −1, R17 −1, R18 −1, R19 −1, R25 −2, R27 −4 y R28 −9.

Los hallazgos que no son de código no tienen rebanada:
- ERR-4.1: rotar el secreto, una acción del usuario.
- TDD-1.5, TDD-1.6 y TDD-1.7: son históricos. Ver D8.

### 5.7 Decisiones del usuario

Decididas (2026-11-20, en el PR de R1):

- **D3. `ListPublicCommunities` (R18, R7): decidido, eliminar.** Se borran el caso, su puerto de entrada, `get_public_list`, `find_by_id` y `AuthContext`. No se agrega ninguna ruta. R18 se adelanta y va antes de R7.
- **D6. Tiempo de respuesta del sign-in (R2): decidido, sí, dentro de R2.** Cuando el usuario no existe, se verifica Argon2 contra un hash señuelo generado al arrancar, así el 401 no permite enumerar usuarios por tiempo. Test rojo: `verifies_against_the_decoy_hash_when_account_does_not_exist`.
- **D7. Cuerpo interino del 500 por `Fault` (R5 a R30): decidido, el cuerpo que propone el plan.** `{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}`. Rige hasta que R30 lo reemplaza por Problem Details (D5).

Abiertas:

- **D1. Verificar dos veces (R13).** Seguir con 400 `IAM_ACCOUNT_INVALID_VERIFICATION` o pasar a 409 `Conflict` ("ya verificada").
- **D2. Identidad del dueño en communities (R16).**
  - **a)** Validar el formato UUID y el nil, conservando los códigos `IAM_INVALID_ACCOUNT_ID(_FORMAT)`. No cambia el contrato y es la opción por defecto del plan.
  - **b)** Lo mismo, con códigos propios `COMMUNITIES_INVALID_OWNER_ID…`. Cambia el contrato.
  - **c)** No validar: el `sub` llega firmado por iam y `OwnerId` es opaco, como en el template. Cambia el contrato: un `sub` no UUID crea la comunidad.
- **D4. Eventos (R21, R22).** Implementar `AccountEvent::Registered` con `take_events` y el comportamiento de `MembershipPolicy`, o retirar el andamiaje muerto (HEX-7.4, HEX-7.9).
- **D5. Forma de Problem Details (R30).** Si se conserva `code` con los mismos valores, qué texto va en `detail`, y si los 401 llevan cuerpo.
- **D8. Hallazgos históricos (TDD-1.5 a TDD-1.7).** Marcarlos como `descartado (histórico; TDD-1 rige desde R1)`.
- **D9. Mover `app/` a `apps/server`, como en el template.** Fuera del inventario; opcional después de R29.
- **D10. `identify` no normaliza el email.** Lo fija `identify_with_different_email_casing_returns_404`. No es un hallazgo; se mantiene salvo que se decida lo contrario.

### 5.8 Registro de rebanadas

Una fila por rebanada fusionada. **Tokens** es el consumo de la sesión que hizo la rebanada (lo que informa `/cost` al cerrarla). **Revisor** dice si se corrió `craft-backend:architecture-reviewer` y por qué no, si no se corrió.

| Rebanada | PR | Commit | Toolchain | Tokens | Revisor | Notas |
|---|---|---|---|---|---|---|
| R1 | #24 | ver PR | `nightly-2026-11-19` | pendiente: dato de `/cost` | no: no toca código de producción | Apilado sobre #23, que lleva #21 y #22 a `main`. fmt, clippy y 187 tests en verde localmente. #24 se fusionó en `ci/rust-checks`, no en `main`; lo lleva a `main` #25. |
| R2 | #26 | `ad2f129`, `4149660`, `8a70093` | `nightly-2026-11-19` | $2.67 (opus-5-5: 3.7k input, 39.0k output, 4.2m cache read, 143.1k cache write; 8m 19s API, 26m 9s wall) | sí (ver PR) | Apilado sobre #25. 188 tests, fmt y clippy en verde localmente. Sin test de tiempos: el Spy fija la llamada al señuelo. |
| R3 | #27 | `eb30039`, `f552921` | `nightly-2026-11-19` | $1.48 (opus-5-5: 1.8k input, 18.8k output, 1.9m cache read, 106.8k cache write; 4m 27s API, 4m 54s wall) | sí (ver PR) | Línea base −17 (todas las `err_1_*`). 188 tests, fmt y clippy en verde localmente. |
| R4 | #28 | `80c00e4` | `nightly-2026-11-19` | $0.80 (opus-5-5: 1.3k input, 10.5k output, 778.9k cache read, 59.8k cache write; 2m 34s API, 2m 40s wall) | sí (ver PR) | Línea base −2 (`hex_8_kernel_traits`: `domain_error.rs`, `infrastructure_error.rs`). 188 tests, fmt y clippy en verde localmente. |
| R5 | #30 | `e76c162`, `29890cb`, `446726b` | `nightly-2026-11-19` | $3.06 (opus-5-5: 4.6k input, 27.6k output, 4.1m cache read, 220.0k cache write; 6m 32s API, 12h 21m wall, casi todo inactividad entre turnos) | sí (ver PR) | Línea base sin cambios. Copia de ai-harness `97a8f29`; `Clock` pasa a R21. 194 tests, fmt y clippy en verde localmente. |
| R6 | #29 | `991e672` | `nightly-2026-11-19` | $1.37 (opus-5-5: 447 input, 16.6k output, 1.6m cache read, 98.0k cache write; 3m 17s API, 4m 26s wall) | sí (ver PR) | Línea base sin cambios. `Cargo.lock` solo pierde `anyhow`; features efectivas idénticas. 188 tests, fmt y clippy en verde localmente. Commit aparte `45fe532` con `/slice`. |
| S1 | #31 | `79185ff`, `c8dbed3` | `nightly-2026-11-19` | $3.81 (opus-5-5: 5.6k input, 56.1k output, 6.1m cache read, 207.1k cache write; 10m 3s API, 23m 29s wall) | sí (ver PR) | Fuera del plan: `tools/architecture` desde ai-harness `v1.0.0-rc.1`. Re-línea base 36 → 47 entradas, solo por cambio de formato (§5.10). Sin código de producción. |

### 5.9 Puntos de replanificación

Hay tres: **P1** después de R10, **P2** después de R19 y **P3** después de R26 (antes de R27). En cada uno, antes de abrir la rebanada siguiente:

1. Auditoría completa con `craft-backend:architecture-reviewer` y actualización de la cabecera del documento (fecha, commit, conteos).
2. `cargo test -p architecture`, y comparar las entradas que quedan en `baseline.txt` con la columna "Entradas hoy" de §5.6 menos lo ya cerrado. Si no coinciden, explicar la diferencia antes de seguir.
3. Revisar las decisiones abiertas de §5.7 que bloquean las rebanadas siguientes: D1 (R13) y D2 (R16, si todavía no se hizo) en P1; D4 (R21, R22) en P2, y D5 (R30) en P3.
4. Comparar tamaño y tokens reales de §5.8 con lo estimado, y partir las rebanadas que vienen si alguna quedó más grande de lo previsto.
5. Reescribir en este documento lo que cambie del plan (secuencia, IDs, línea base) en un PR de solo documentación, antes de seguir.

### 5.10 S1: re-línea base

Rebanada fuera del plan. Sincroniza `tools/architecture` con `stacks/rust/template/tools/architecture` de ai-harness `v1.0.0-rc.1` (`3b5dc80`), sin tocar código de producción. Los archivos se copiaron tal cual: `cargo fmt` con el `rustfmt.toml` del proyecto solo reagrupa los `use` de 5 archivos. Única divergencia de contenido: en `tests/legacy.rs:45`, `contains(&"axum".to_string())` pasa a `iter().any(|d| d == "axum")`, porque el clippy de `nightly-2026-11-19` lo rechaza (`unnecessary_to_owned`) con `-D warnings`. Hay que reportarlo al template. `tests/common/mod.rs` es el del template, sin adaptación local. Su modo legacy oficial ya coincide con este repo: `LEGACY_KERNEL_DIR = "shared"`, y `iam` y `communities` pasan a legacy porque tienen `Cargo.toml` propio. Fuera de `tools/`, solo cambia el `Cargo.toml` raíz, que gana `publish = false` en `[workspace.package]`. Lo exige `publish.workspace = true` del template.

Línea base vieja: 36 entradas. Nueva: 47 entradas (65 violaciones con las `n`). La nueva se llenó con `CRAFT_BACKEND_BASELINE_INIT=1` a partir del informe de los tests, con la línea base vaciada a la cabecera. La cabecera del template dice que la línea base "se llena una sola vez". Este segundo llenado es una excepción del proyecto, que §5.1 acota a las sincronizaciones con el harness: cambia el formato de las claves, no las violaciones que se toleran.

**a) Se mantienen (29).** Todas las entradas de ruta de archivo, con la misma clave:
- `hex_4_use_case_implementations_are_handlers`: 6;
- `hex_5_adapters_are_named_by_technology_and_concept`: 8;
- `hex_5_outbound_ports_are_named_by_role`: 4;
- `hex_6_transport_types_carry_the_transport_name`: 9;
- `hex_8_kernel_traits_are_model_markers_or_listed_capabilities`: 2 (`token_verifier.rs`, `error.rs`).

**b) Cambian de clave por el formato nuevo (7 → 18).** La clave pasa de `ruta/Cargo.toml` a `ruta/Cargo.toml#sección.paquete`, y `n` cuenta las capas virtuales del contexto legacy que ven la dependencia: domain y application en `hex_2_core` (2); domain, application e infrastructure en `hex_2_frameworks` (3, porque Api puede usar axum), y las cuatro capas de communities en `hex_1` (4).

| Test | Clave vieja | Claves nuevas | ID |
|---|---|---|---|
| `hex_1_dependencies_point_inward` | `contexts/communities/Cargo.toml` | `…#dependencies.iam` 4 | HEX-1.1 |
| `hex_2_core_depends_only_on_std_and_the_kernel` | `contexts/communities/Cargo.toml` | `…#dependencies.axum` 2, `.http` 2, `.serde` 2, `.uuid` 2 | HEX-2.5 |
| `hex_2_core_depends_only_on_std_and_the_kernel` | `contexts/iam/Cargo.toml` | `…#dependencies.argon2` 2, `.axum` 2, `.chrono` 2, `.jsonwebtoken` 2, `.rand` 2, `.serde` 2, `.uuid` 2 | HEX-2.1 |
| `hex_2_core_depends_only_on_std_and_the_kernel` | `shared/Cargo.toml` | `…#dependencies.chrono`, `.serde` | HEX-8.1 |
| `hex_2_server_frameworks_only_at_the_edge` | `contexts/communities/Cargo.toml` | `…#dependencies.axum` 3 | HEX-2.5 |
| `hex_2_server_frameworks_only_at_the_edge` | `contexts/iam/Cargo.toml` | `…#dependencies.axum` 3 | HEX-2.1 |
| `hex_8_kernel_depends_only_on_std` | `shared/Cargo.toml` | `…#dependencies.chrono`, `.serde` | HEX-8.1 |

Cada paquete nuevo ya figuraba en el hallazgo de la columna ID. La única excepción es `http` en communities: HEX-2.5 nombra `uuid`, `axum` y `serde`, y `http` (que usa `infrastructure/http`) es el mismo hallazgo, con un paquete más. Se agrega a su fila.

**c) Desaparecen sin un cambio de formato que lo explique: ninguna.** Las 7 claves viejas que ya no están corresponden, una por una, a las claves nuevas de b).

**d) Entradas nuevas: ninguna.** Ninguna regla nueva del template encuentra violaciones en este repo. Los tests nuevos (`legacy.rs`, `ratchet.rs`, `hex_3_sees_the_legacy_kernel`, `hex_4_contracts_are_checked_in_legacy_contexts`) y los que ahora actúan en legacy (`hex_4_use_case_contracts_do_not_expose_domain_types`, con 17 tipos de contrato) pasan en verde.

**Rebanadas que cambian:**
- **R21:** copia `Clock` del template en lugar de escribirlo, y quita `chrono` de `contexts/iam/Cargo.toml` y de `shared/Cargo.toml`, que dejan de usarlo. Línea base: de 0 a −3.
- **R25:** −2, pero ahora son las dos entradas de `serde` de `shared/Cargo.toml`.
- **R27:** de −4 a −7. Deja de tener el riesgo de "HEX-4 se activa en este PR".
- **R28:** de −9 a −14.
- **R29:** borra todo lo marcado `legacy:` en `tools/architecture/tests`, más `tests/legacy.rs` y `tests/fixtures/legacy/`.
- **R11–R18:** ya no necesitan el `grep "crate::domain"` a mano (§5.1).
- Cualquier rebanada que quite un paquete de un `Cargo.toml` borra sus entradas en el mismo PR (§5.1).

**Kernel del template desde `97a8f29` (no se copia aquí).** Commits `ec3d9ea` y `e1cfa7d`:
- `kernel/src/clock.rs` (`trait Clock: Send + Sync { fn now(&self) -> SystemTime; }`) y `pub use clock::Clock` en `kernel/src/lib.rs` → **R21**, como ya preveía el plan.
- Crate nuevo `infrastructure/system` con `SystemClockAdapter` y su test → **R21**, junto con `Clock`, si R21 cablea el reloj real. Si no, en R26, junto con `infrastructure/stderr`.
- `infrastructure/stderr/src/fault_reporter.rs`: el reporte recorre la cadena `source()` del `Fault` (+64 líneas) → **R26**, que crea ese crate. Hay que copiarlo desde `v1.0.0-rc.1`, no desde `97a8f29`.
- `failure.rs`, `fault.rs` e `id.rs` del template coinciden con los de `shared/src` (solo cambia la agrupación de `use` por `cargo fmt`). `future.rs` y `model.rs` siguen asignados a R23/R24 y R21.

## Método

- Se leyó entero cada archivo `.rs` y `Cargo.toml` versionado, y cada `archivo:línea` se verificó contra su contenido en `d0b0eeb`.
- No se ejecutaron `cargo test` ni `clippy` (auditoría en solo lectura). El repo no tiene tests de arquitectura que correr.
- El orden de los commits se revisó con `git log --reverse`.
