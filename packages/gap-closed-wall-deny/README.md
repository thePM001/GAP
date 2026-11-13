# gap-closed-wall-deny 0.1.0

## What it is

This package is the closed wall Deny of GAP. It judges the envelope of a sealed capsule, names every closed wall with a reason and a repair, seals a capsule with a digest and reseals it on a retry. It teaches the capsule side of Admit in a form that runs without a Base Node.

## Place in GAP

The package is the capsule side of Layer 3 in section 03 of the root README, Write. Check. Admit. The profile package judges the instruction and this package judges the capsule that carries the action.

## What ships

| Wall id | Closes when | Prescribed repair |
| --- | --- | --- |
| `gap:unbound:scene` | The scene is not bound | Bind the scene on the sealed capsule and reseal |
| `gap:unbound:dock` | The dock is not bound | Bind the dock on the sealed capsule and reseal |
| `gap:unbound:time` | The timestamp is not bound | Bind the timestamp at seal and reseal |
| `gap:unbound:sequence` | The sequence is not bound | Bind the sequence on the sealed capsule and reseal |
| `gap:writing` | The writing is empty or carries a forbidden dash character | Repair the writing and reseal a new capsule |

- All five walls are judged in one pass. When two walls close both are named, in sorted order.
- A repair never carries a grant list, so a close does not tell an agent which permission it would need.
- The digest mode is sha256-structure: a SHA-256 over the sealed bytes, written as 64 hex characters.
- The same sealed bytes are a replay. A retry reseals with a new nonce, so its digest differs from the first one.
- The field `enabled` is read at load time and an attractor is a forensic record. Neither lets an action omit Admit.
- Tests prove each point above, with the SHA-256 test vectors of NIST for the digest.

## Interfaces

The binary `gap-closed-wall-deny` reads lines of the form key=value from standard input:

| Key | Meaning |
| --- | --- |
| `scene`, `dock`, `timestamp` and `sequence` | The four bind fields of the envelope |
| `writing` | The text that the writing wall reads |
| `enabled` | The load time flag, true unless the value is false |
| `input` | Optional bytes to seal. When present, the binary also prints the digest lines |

It prints `allow=true` or `allow=false` and then one line per closed wall with the wall id, the reason and the repair, each separated by a bar. An envelope without a dock and with an input prints:

```
allow=false
closed=gap:unbound:dock|unbound dock closes Admit|bind dock on the sealed capsule then reseal
digest=00f96dd3cc53934ea22f269fbc56dae1ef26f92af94fc8220a9307b81ab8c7f4
retry_digest=c3f2078f0e73b6ca715acab6f10316c41eb6b5da92d1e4f726a75e4d848eeab6
replay=true
resealed=true
```

The library exposes `live_admit` for the whole pass, one function per wall, `collect_all` for the collect-all rule and `seal_capsule`, `is_replay`, `reseal_on_retry` and `structure_digest` for the capsule.

## Limits

The package is a teaching subset of the live path. It reads an envelope of six fields and not the action and snapshot of a live Base Node. It does not freeze a clock, does not wait for the kernel pulse and does not apply an action. It does not read a GAP instruction, which is the work of the profile package.

## Run and test

From the folder `packages`:

```
cargo build --locked -p gap-closed-wall-deny
cargo test --locked -p gap-closed-wall-deny
```

To judge an envelope by hand, write the keys into a file with one key per line. Then pass the file to `target/debug/gap-closed-wall-deny` on standard input.

## Related documents

- [The root README](../../README.md), section 03 for the three layers.
- [docs/LIVE-EVALUATION.md](../../docs/LIVE-EVALUATION.md) for the path of one action, the kernel pulse and the trust bundle.
- [The profile package](../gap-schema-profile-v13/README.md) for the walls that read an instruction.
