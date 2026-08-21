# Security Policy

## Supported versions

Security fixes are applied to the latest release line and, when practical, the immediately previous minor release.

## Reporting a vulnerability

Please report suspected vulnerabilities privately to `sanskarin@outlook.in` with the subject `SortSmith security report`. Include affected version, reproduction conditions, and impact. Do not include real private files or secrets.

Please allow maintainers a reasonable remediation window before public disclosure. Security fixes should include regression tests when feasible.

## Security model

SortSmith is local-first and intentionally uses no authentication or cloud backend. Its highest-risk boundary is filesystem access. The engine rejects parent-directory destination traversal, disables link following by default, canonicalizes selected roots in the desktop host, blocks destination redirection through symlinks outside the chosen root, and records reversible journals. Users remain responsible for filesystem permissions and backups of irreplaceable data.
