# DEC-072 - Publish Atlas Explorer on GitHub Pages

## Status

Accepted on 2026-07-15 (`pages-A`).

## Context

E-M1 produces a deterministic static bundle under `build/web`. The artifact has
no application server, account, analytics service or runtime network
dependency. All browser resources use relative paths and therefore remain valid
under the repository Pages path `/atlas/`.

DEC-071 required a separate decision before external publication. The owner now
selects the dedicated GitHub Pages workflow proposed for E-M3 rather than
coupling publication to the complete MIR and RV64 CI gate.

## Decision

- Publish the current Atlas Explorer at
  `https://cyrilmhansen.github.io/atlas/` through GitHub Pages.
- Build and verify the artifact with `scripts/check-web.sh` from every pushed
  `main` commit and from an explicit manual dispatch.
- Upload only `build/web` as the Pages artifact.
- Keep build and deployment in a dedicated workflow. Grant `pages: write` and
  `id-token: write` only to its deployment job.
- Use the GitHub-managed `github-pages` environment and report the deployed URL
  through the deployment output.
- Do not configure a custom domain, analytics, remote execution, service worker
  or application backend in this milestone.

## Authority and support boundary

Publication does not stabilize `atlas-web-private-v0`, the generated visual
program, the WASM facade, URL query parameters or the layout. Git and the YAML
registry remain authoritative; the deployed files are replaceable derived
artifacts associated with a source commit.

The repository Pages URL is the supported public entry point for Phase 3. Deep
links are useful demonstrations but remain private product routes until a later
decision defines a public URL policy.

## Consequences

- Explorer changes merged to `main` can become public independently of MIR and
  RV64 probe availability, but only after the complete Web acceptance slice
  passes in the Pages build job.
- The existing CI remains unchanged and retains the broader project gates.
- Deployment is reversible by disabling Pages or the workflow; no generated
  site files are committed to a publication branch.
- E-M3's distribution choice is resolved. It does not close Phase 3 or replace
  the E-M2 comprehension experiment.

## Alternatives considered

- Dedicated Pages workflow (`pages-A`): accepted for its narrow permissions,
  direct reuse of the Web gate and low coupling.
- Deployment after the complete CI workflow: rejected for now because MIR and
  RV64 availability should not control an otherwise verified static artifact.
- Committed `docs/` or `gh-pages` build output: rejected because it duplicates
  derived files in Git authority and creates avoidable synchronization work.
