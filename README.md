# FHIR Consent exchange — Rust

Small library and CLI that sanity-check **FHIR Consent** before exchange and Patient Access flows: **active** status, **scope**, **patient**, and **provision** period.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 23** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [FHIR Consent](http://hl7.org/fhir/R4/consent.html)

## What it checks

- `resourceType` **Consent**
- **status** `active`
- **patient.reference**
- **scope** present
- Warning if **provision.period** missing

| Problem | What breaks |
|--------|-------------|
| **Draft consent** | Apps cannot treat consent as granted |
| **No scope** | Downstream cannot filter permitted data |
| **No patient** | Consent not bound to a person |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-consent -- examples/valid-consent.json
cargo run --bin validate-consent -- examples/invalid-consent.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or input.

## Sample output

```bash
cargo run --bin validate-consent -- examples/valid-consent.json
```

**Pass** — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-consent.json`) — exit code `1`:

```json
[
  {
    "severity": "error",
    "path": "status",
    "rule_id": "..."
  }
]
```

## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken fixtures for tests and demos

## GitHub topics

Paste in repository **About** (also: `fhir`, `hl7-fhir`, `healthcare-interoperability`, `rust`):

`fhir-consent, patient-access, smart-on-fhir, privacy, fhir, hl7-fhir, healthcare-interoperability, rust, fhir-r4`

## Inferno vs this repo

Patient Access suites test **authorized** reads on servers. This repo is the **Consent resource** your portal stores before tokens are issued.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
