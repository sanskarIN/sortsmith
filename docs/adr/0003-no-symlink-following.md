# ADR 0003: Do not follow filesystem links by default

**Status:** Accepted — 2026-08-21

Following symlinks/junctions can unexpectedly traverse outside a selected root or create cycles. SortSmith therefore sets `follow_links=false` for user scans and automation. The desktop boundary additionally validates planned paths and rejects destination parent components that resolve outside the selected root. A future advanced link-following option would require explicit risk messaging and cycle/root-containment defenses.
