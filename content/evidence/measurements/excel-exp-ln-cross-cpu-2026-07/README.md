# EXP and LN on an Intel and an AMD machine (July 2026)

The measurement behind "The Last Bit" (`content/lastbit/the-last-bit-2026-07.pdf`): the same 106
`=EXP`/`=LN` probes, injected through `Range.Value2` and read back as raw binary64 bits by
`Collect-ExcelExpData.ps1`, on two machines.

| File | CPU | OS | Excel | Collected (UTC) |
|---|---|---|---|---|
| `ExcelExpData_AuthenticAMD_20260703-223049.json` | AMD Ryzen 7 4800H | Windows 11 Pro 10.0.26200 | 16.0 build 20131, 64-bit | 2026-07-03 22:30 |
| `ExcelExpData_GenuineIntel_20260705-191433.json` | Intel Core i7-9750H | Windows 10 Home 10.0.19045 | 16.0 build 18429, 64-bit | 2026-07-05 17:14 |

The two runs used different Excel builds (20131 and 18429), as each file records, although the
write-up says "the same build". A difference here therefore differs in CPU, Excel build and OS
together. Of the 106 probes 83 agree and 23 differ by one unit in the last place, all in the
`microcode` category; the per-function records are `content/evidence/environments/FUNC.EXP.json`
and `FUNC.LN.json`.

Copied on 2026-10-30 from `C:\Temp\ExcelExpFunction` (the AMD file there is
`research/data/baseline_AMD_Ryzen4800H.json`). The only change is `environment.computer_name`,
replaced by "(removed for publication)". SHA-256 of the originals: AMD
`3cb521213a8493063d820ddfb798b4c3b9591c3199790c020977b89752250764`, Intel
`730ec8038939b0f4fc139098576ca30309b1dd240ce8b4614aa686e30ac1d504`; the script is unchanged
(`cf9c394d7d5ef65593c95bc866ab3e523f82ac4b199c594cb0cd88592a612c25`).
