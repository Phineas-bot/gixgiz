# ADR 0005: Windows application identity and packaging direction

- **Status:** Accepted
- **Date:** 2026-08-03
- **Owners:** GixGiz project owner
- **Decision scope:** Product identity, installation scope and packaging direction; required before Tasks 02–05
- **Related:** ADR 0001, ADR 0002, ADR 0004, Product Vision, GitHub issue #1

## Context

Windows application identity affects executable names, package identity, application-data paths, upgrades, uninstall behavior, signing, shortcuts, process discovery and future client compatibility. Changing these identifiers after public releases can orphan databases, credentials, models, update channels and uninstall records.

GixGiz is still in the foundation stage, so this is the correct point to freeze stable names while keeping packaging implementation deliberately small. The product is licensed under MIT, but code licensing does not grant others ownership of the GixGiz name or official branding.

## Decision

### 1. Canonical product identity

The canonical identity is:

```text
Product display name: GixGiz
Repository/source name: gixgiz
Desktop executable: GixGiz.exe
Core executable: gixgiz-core.exe
Windows package/application identifier: ai.gixgiz.desktop
Default per-user data root: %LOCALAPPDATA%\GixGiz
Default per-user cache/staging root: %LOCALAPPDATA%\GixGiz\cache
Rust crate prefix: gixgiz-
Dart package root name: gixgiz_desktop
```

The package identifier is a stable technical identity and must not be changed casually after distribution begins.

### 2. Installation scope

The initial supported installation is **per-user**.

- Normal installation and operation should not require administrator rights.
- Application binaries install in a per-user application location selected by the packaging technology.
- User data lives outside the installation directory.
- System-wide installation is deferred until enterprise or multi-user requirements justify it.
- Privileged runtime installation, when later required, uses a separate explicitly approved helper and does not elevate the full desktop/core application.

### 3. Packaging direction

The project will package the Flutter desktop shell and Rust core sidecar as one coordinated Windows product.

For prototype and early internal builds:

- a reproducible unpackaged Windows build is acceptable;
- the desktop and compatible core are distributed together;
- the desktop resolves the bundled core by a controlled installation-relative path;
- version compatibility is still checked at runtime.

Before external alpha, the project will select and document one signed installer/package format after validating:

- Flutter desktop support;
- sidecar bundling;
- per-user installation;
- upgrades and rollback;
- SmartScreen and code-signing behavior;
- runtime/provider integration;
- custom/user-selected model storage;
- uninstall retention choices.

No ADR claim is made yet that MSIX, MSI or a custom bootstrapper is the final format.

### 4. Release artifact contents

A GixGiz Windows release may include:

- `GixGiz.exe` and Flutter runtime assets;
- `gixgiz-core.exe`;
- required native runtime libraries;
- default configuration and signed catalogue/rule assets;
- licences and third-party notices;
- optional short-lived updater/installer helper in later releases.

It must not include downloaded user models, user databases, credentials or personal diagnostics.

### 5. Data and storage layout

The per-user root contains clearly separated domains, for example:

```text
%LOCALAPPDATA%\GixGiz\
├── data\          # SQLite and structured platform state
├── config\        # non-secret user/device configuration
├── logs\          # bounded, rotated and redacted logs
├── cache\         # disposable caches
├── staging\       # resumable downloads/install staging
├── content\       # managed metadata/content not stored in SQLite
└── backups\       # migration/recovery snapshots
```

Large model storage may use a user-approved internal or external location. The database records validated references and ownership metadata.

Secrets use an OS-backed secret store when introduced and are not placed in these plaintext directories.

### 6. Upgrade compatibility

- Desktop and core ship as a tested compatible pair.
- Startup performs the protocol handshake in ADR 0003.
- An incompatible core does not run silently; the UI reports a repair/update requirement.
- Updates are staged and verified before replacing active binaries.
- Binary replacement and database migration are separate concerns: the updater must not remove the previous recovery path before the new application and migrations validate successfully.
- Downgrade is not assumed safe when the database schema is newer.

### 7. Signing and release trust

- Internal development builds may be unsigned and must be labelled as such.
- External alpha and later releases should be code-signed when practical.
- Signing keys and certificates are never available to untrusted pull-request workflows.
- Release artifacts include checksums and provenance/SBOM information as release maturity increases.
- Catalogue, rules, update metadata and future Packs require their own integrity policies; signing the desktop binary alone is insufficient.

### 8. Uninstall and repair behavior

Uninstall removes application binaries and registered application integration, but it must not silently delete user data, conversations, managed model libraries or externally installed Ollama.

The uninstaller or desktop removal flow must present explicit choices such as:

- keep all user data and models;
- remove temporary/cache/staging data only;
- remove GixGiz-managed data after showing estimated impact;
- leave externally managed runtimes and models untouched.

Repair reinstalls or verifies application binaries without treating user data as disposable.

### 9. Branding and licence

- Source code is available under the repository’s MIT License.
- Official product naming uses `GixGiz` consistently.
- Forks must follow the MIT notice requirements, but the licence does not automatically make them official GixGiz releases.
- A separate trademark/branding policy may be added before a public extension ecosystem or commercial distribution.

## Alternatives considered

### Per-machine installation from the beginning

Deferred. It may suit managed enterprise deployment, but it adds elevation, multi-user data ownership, services and update complexity before the consumer foundation is proven.

### MSIX selected immediately

Not accepted yet. MSIX offers identity and update benefits, but sandboxing, sidecar behavior, external runtimes and user-selected storage must be tested before commitment.

### MSI or custom EXE selected immediately

Not accepted yet. These provide flexibility but require installer authoring, elevation discipline, rollback and signing decisions that should be based on an actual build.

### Store database and models beside the executable

Rejected. Installation directories may be read-only, replaced during updates or removed during uninstall. User data requires an independent lifecycle.

### Fixed data path on another drive

Rejected as the default. `%LOCALAPPDATA%` is appropriate for per-user structured state. Large model storage may be configurable after validation.

### Package identifier based on GitHub username

Rejected. Product identity should remain independent of repository ownership and hosting platform.

## Consequences

### Positive

- Stable names across Flutter, Rust, Windows and documentation.
- Per-user scope reduces privilege requirements and multi-user conflicts.
- User data survives normal application upgrades and uninstall by default.
- Packaging remains flexible until a real sidecar build can be evaluated.
- Desktop/core compatibility and recovery are treated as release requirements.

### Negative

- The final installer technology remains an open implementation decision before external alpha.
- Two coordinated executables increase packaging and signing work.
- Per-user installation does not immediately solve enterprise deployment.
- Data-retention choices add UX and test requirements to uninstall/repair flows.

## Security and privacy impact

- Least-privilege per-user installation is the default.
- User data and secrets are separated from replaceable binaries.
- Signing material is isolated from normal CI and pull requests.
- Repair/update operations must validate package identity and integrity.
- Uninstall cannot silently remove private user content or externally managed software.
- Application paths, data roots and selected external storage are canonicalized before use.

## Implementation constraints

- Task 02 uses the product display name and Dart identity defined here.
- Task 03 uses `gixgiz-core` and `gixgiz-contracts` naming.
- Task 04 discovers and supervises the core through controlled package-relative/bootstrap behavior.
- Task 05 uses the `%LOCALAPPDATA%\GixGiz` data root and keeps SQLite outside the installation directory.
- Tasks 02–05 do not need a production installer, updater, signing certificate or privileged helper.
- No code may hard-code a final MSIX/MSI decision before packaging evaluation.

## Follow-up work

- Compare MSIX, MSI and signed bootstrapper options after the first packaged desktop/core handshake build.
- Add `THIRD_PARTY_NOTICES` and dependency-licence reporting before external distribution.
- Define application update and rollback architecture before automatic updates.
- Define the privileged helper before Task 09 installs or modifies Ollama.
- Add a branding/trademark policy before accepting third-party Packs or distributing official logos.