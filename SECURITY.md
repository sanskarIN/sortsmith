# Security Policy

## Supported versions

Security fixes are applied to the latest release line and, when practical, the immediately previous minor release.

## Reporting a vulnerability

Please report suspected vulnerabilities privately to `sanskarin@outlook.in` with the subject `SortSmith security report`. Include the affected version, reproduction conditions, expected/observed behavior, and security impact. Do not attach real private files, credentials, tokens, or other sensitive data; use synthetic fixtures whenever possible.

Please allow maintainers a reasonable remediation window before public disclosure. Security fixes should include regression tests when feasible.

## Security model

SortSmith is local-first and intentionally uses no authentication or cloud backend. Its highest-risk boundary is filesystem access.

Current controls include:

- selected roots are canonicalized before privileged filesystem work;
- normal scans do not follow symlinks;
- parent-directory traversal and unsafe destination components are rejected;
- existing destination-parent symlinks are resolved and blocked if they escape the selected root;
- rendered rename filenames reject unsafe cross-platform characters, Windows-reserved device names, trailing spaces/periods, and excessive filename lengths;
- collision handling never intentionally overwrites an existing destination;
- undo journals are reversible, separate from settings, and loaded from regular local journal files;
- settings/import data is schema- and rule-validated, bounded to 16 MiB, and persisted through flushed/synced temporary files plus replacement rename;
- imported settings and saved local state reject symlink-backed files at the trust boundary;
- structured operation logs exclude file paths/content, rotate at 5 MiB, and refuse symlink/non-file targets;
- duplicate detection hashes locally and never deletes duplicate candidates automatically;
- Tauri's Content Security Policy restricts application content to local sources and the IPC endpoint required by the desktop runtime;
- CI includes format/lint/tests/build checks and CodeQL analysis for TypeScript and Rust.

## Threat-model limitations

SortSmith cannot protect files from an already-compromised operating-system account, malicious kernel/administrator software, or external programs racing to replace files after validation. Users remain responsible for operating-system permissions and independent backups of irreplaceable data. Native background execution is not enabled in version 0.1, reducing unattended permission/startup risk.

## Secrets

The public repository must not contain real API keys, signing keys, certificates, passwords, tokens, personal production data, or notarization credentials. Release signing material belongs only in protected CI secrets or trusted platform key stores.
