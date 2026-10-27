# Raw maintenance v1 — validation, 27 October 2026

Implementation in PR #473, reviewed with Claude Code in read-only mode before deployment.
The review is scoped to raw maintenance as requested: no pickup/fall/anatomical protection gates.

## Corrections made after review

| Finding | Correction / regression |
|---|---|
| Stop/release could queue behind ordinary work | Separate priority slot for stop/release/cancel; drain cancelled ordinary operations; test with a full 16-entry queue. |
| Refresh could saturate admission and scan during walking | Atomic pre-enqueue global rate limit; background scan only in maintenance or normal idle/Limp; tests for rate and no normal-motion scanning. |
| Failed read preserved old registers under a new timestamp | Replace failed snapshot with unknown fields, preserve startup preimage and record the error; fault-injection test ensures no PID/goal confirmation from stale data. |
| Exit could enable torque before clearing old raw goals | Keep admissions closed through fresh-pose and normal-gain restoration with torque off; stay write-inhibited if restoration fails. Integration counters verify no normal writes under ownership and explicit normal restoration. |
| Stale telemetry targets across off/session boundaries | Remove confirmed-off joint targets, clear per-session command state on acquisition. |
| Lost release response was not replayable | Bounded terminal release outcome; replay cannot close a newly acquired session. |
| Expiry repeatedly retried a dead servo every tick | Clear the expiry after the first off attempt; unresolved off remains inhibited for explicit cancel/relax recovery. |
| Missing per-motor admission errors and acquire metadata | Named unknown/on outcomes; acquire includes lease and readback snapshot; committed outcomes include monotonic completion time/control tick. Caller credentials are journaled. |
| Native register decoder lacked coverage | Extract pure XL330 decoder and exercise byte offsets, signed homing, goal/count conversion and truncated blocks across all 15 motor IDs. |
| Route coverage did not include the new methods | Add all 12 diagnostic methods to the shared exhaustive test inventory; BLE/WebRTC rejection tests now cover them. API history records v45. |

Other regressions cover every motor's selective torque/PID/goal path, seeding and enable readback
failure, partial gains, stale heartbeat, lease/disconnect cleanup, independent inhibit recovery,
request conflict/eviction, superseded target deadlines and unrelated-motor holds.

A targeted follow-up found a concurrent acquire could hide a completed-session edge. The
controller now retains an exit-owed barrier until the integration acknowledges fresh goal/PID
restoration; new acquisitions remain excluded throughout. Regression tests race acquisition
against release and check the barrier across ticks. A non-owner cannot replace a pending owner
stop/release, and terminal release retries remain allowed for the prior owner.

Unconfirmed stop is deliberately not retried every tick: status retains the inhibit and an
operator cancel/relax retries it. This is the agreed raw-mode failure contract, not a watchdog.

## Checks

Local full-workspace tests pass on macOS excluding `nfc`: the unchanged nfcd binary imports
Linux-only Spidev on this host. Linux CI includes the entire workspace and the board build.
Focused regression tests were rerun after the review fixes. Final CI and deployment revision
are recorded in the PR rather than assuming a moving branch tag is immutable.

No movement or torque-on test on the physical robot is implied by fake/IPC tests. Deployment
verification reads all 15 native snapshots before any future motor activation. Actual serial
latency, servo-side behaviour after host process failure and mechanical clearance require bench
measurement. There is no host-crash watchdog guarantee in v1. The native snapshot read is one
148-byte control-table transaction for one ID; it must be verified on the installed hardware.
