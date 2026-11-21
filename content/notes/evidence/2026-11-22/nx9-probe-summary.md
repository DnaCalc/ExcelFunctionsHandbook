# Lane nx9 live-Excel `expm1` proof summary (`dc-j9k.67`)

Measured on Excel `16.0.20527.20196` (Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics)
against DnaCalc commits `339ef7baf6501230364262ee047e062b26f672f5` (before) and
`0707a18dea2de59b5cb7eeb3778fb9b292a40071` (after):

- `nx9_probe1` (SHA-256 `016793ae`):
  - `A_rexp_3way_gt015`: Kahan `excel_expm1_internal(z)` matched 21 of 40 rows (total error
    32 units in the last place) vs 7 of 40 (69 units) for `gratio_rexp` and 6 of 40 (70 units)
    for correctly rounded `expm1`.
  - `B_rexp_poly_vs_expm1_le015`: `excel_expm1_internal(z)` matched 18 of 32 rows (15 units) vs
    9 of 32 (26 units) for `gratio_rexp`.
  - `C_a0875_zln2_consec`: 16 of 16 zero-variance flat -2-unit offset across consecutive
    inputs (`sum_ulp 32` vs `46`).
- Combined 6 live-Excel suites (1,478 rows): `464` (prior master) -> `525` (`cb11 iop=1` alone)
  -> `582` (`nx9` with `excel_expm1_internal`).
- Real-Rust full witness banks (12 gamma/Poisson/chi functions, 177,966 rows, zero regressions):
  - `GAMMADIST`: 3,019 -> 3,034 / 3,139 (+19/-4, net +15, >=5u 20->18, worst 9->8)
  - `GAMMA.DIST`: 78,209 -> 78,865 / 83,312 (+656/-0, net +656, >=5u 723->716)
  - `GAMMA.INV`: 4,336 -> 4,903 / 10,860 (+586/-19, net +567, >=5u 5,049->4,633)
  - `GAMMAINV`: 4,106 -> 4,255 / 6,163 (+161/-12, net +149, >=5u 1,438->1,356)
  - Total across the 12 functions: 164,044 -> 165,431 (+1,422/-35, net +1,387).
- Pinned witness banks (8 functions): 724 / 732 unchanged.
