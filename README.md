# 📦 tick-id-rs

Session unique **Tick ID** that specifies a specific tick in a deterministic simulation.

A tick represents any positive integer time period, excluding zero, with typical durations being 16 ms or 32 ms.

TickId is represented as a u32. With each tick equivalent to 16 ms, the maximum duration is approximately 68,719,476 seconds, which translates to about 2.18 years.

> “2.18 years should be enough for everyone!”

## Installation

```toml
[dependencies]
tick-id = "0.0.8"
```
