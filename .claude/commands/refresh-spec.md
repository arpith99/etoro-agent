---
description: Refresh docs/spec/ from the eToro API Docs MCP server
---

# Refresh the OpenAPI snapshot

Re-retrieve the upstream eToro OpenAPI document into `docs/spec/`. This is the
one pipeline stage that cannot be a plain script: the document is only served
through the `etoro-api-docs` MCP server. Probing `api-portal.etoro.com` for a
public copy returns 403 (bot protection) for a default user agent and 404 with
a browser user agent at `/openapi/api-reference/openapi.json`,
`/api-reference/openapi.json`, `/openapi.json`, `/docs.json`, and `/mint.json`.
Do not add a scraper that works around the bot protection.

Everything downstream of this command is deterministic and offline.

## Preconditions

- The `etoro-api-docs` MCP server is connected. If it is not, stop and say so —
  do not fabricate spec content.
- Work from the repository root.

## Procedure

All `jq` commands run against the MCP tool
`mcp__etoro-api-docs__query_docs_filesystem_e_toro_api_docs`, targeting
`/openapi/api-reference/openapi.json`.

**1. Read the header and counts.**

```
jq '{openapi, info, servers, counts: {schemas: (.components.schemas|length), operations: ([.paths[][]]|length)}}' /openapi/api-reference/openapi.json
```

Write the result to `docs/spec/.staging/meta.json`, adding a `source` field:
`"eToro API Docs MCP: /openapi/api-reference/openapi.json"`.

**2. Dump the component schemas in chunks of 30.**

For each offset N in 0, 30, 60, ... up to the schema count:

```
jq '.components.schemas | to_entries | .[N:N+30] | from_entries' /openapi/api-reference/openapi.json
```

Write each result verbatim to `docs/spec/.staging/schemas-NN.json`, numbered
`schemas-00.json`, `schemas-01.json`, ... Do not reformat, summarise, or
"clean up" the JSON — this is the codegen input and must stay byte-faithful to
upstream.

**3. Dump the operations index in chunks of 60.**

For each offset N in 0, 60, 120, ...:

```
jq '[.paths | to_entries[] | .key as $p | (.value | to_entries[]) |
     {path: $p, method: .key, operationId: .value.operationId, tags: .value.tags,
      summary: .value.summary,
      scopes: [.value.security[]?.oauth2[]?],
      ratelimit: .value["x-ratelimit"],
      requestSchemas: [.value.requestBody.content[]?.schema."$ref"?],
      responseSchemas: [.value.responses[]?.content[]?.schema."$ref"?]}]
   | .[N:N+60]' /openapi/api-reference/openapi.json
```

Write each to `docs/spec/.staging/operations-NN.json`.

The `requestSchemas` / `responseSchemas` fields are what
`scripts/build_typify_input.py` uses to seed each tag's schema closure, so they
must not be dropped.

**4. Fingerprint upstream, then verify the transcription.**

The chunk contents pass through the agent's context on the way to disk, so they
must be proven byte-faithful rather than assumed. **Do not use `sha256sum` on
the MCP side for this.** That sandbox's text tools are character-oriented: its
`wc -c` reports a character count, and its `sha256sum` does not digest the
UTF-8 byte stream. A correct transcription of any chunk containing non-ASCII
characters (the spec is full of em-dashes) still yields a different digest
there, while pure-ASCII chunks agree — which makes the trap easy to trust by
accident.

Fingerprint over code points instead, which both sides can compute identically:

```
jq -r '.components.schemas | to_entries[] |
       "\(.key) \(.value|tostring|length) \(.value|tostring|explode|reduce .[] as $c (0; (.*131 + $c) % 1000000007))"' \
   /openapi/api-reference/openapi.json
```

Save that output to `docs/spec/.staging/fingerprints.txt`, then:

```sh
python3 scripts/verify_chunks.py docs/spec/.staging/fingerprints.txt
```

It reports missing, extra, and mismatched schemas by name. Fix any mismatch by
re-fetching just that schema before continuing.

**5. Assemble and validate.**

```sh
python3 scripts/fetch_spec.py
```

It merges the chunks, rejects duplicate or missing ranges, fails on any
dangling `$ref`, and writes `docs/spec/{schemas,operations,_meta}.json`.

**6. Clean up and review.**

```sh
rm -rf docs/spec/.staging
python3 scripts/fetch_spec.py --check
jj diff --stat docs/spec
```

Report the API version change (old → new) and the schema/operation count delta.

## After a refresh

A refresh only updates the snapshot. To propagate it into Rust:

```sh
./scripts/regenerate-types.sh
cargo test --all-targets
```

Expect compilation breakage when upstream adds required fields or renames
types — that is the pipeline working as intended, surfacing drift instead of
hiding it. See `scripts/README.md`.
