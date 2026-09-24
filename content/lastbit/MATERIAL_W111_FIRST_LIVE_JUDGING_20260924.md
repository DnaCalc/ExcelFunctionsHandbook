# Material: the first honest look (W111, 2026-09-24)

Raw notes for the Last Bit series. Not edited for publication.

## "We've hardly started"

For months the working belief was: most functions match Excel bit for bit, a few holdouts
(ERFC, GAMMALN, PMT's expm1, IRR/RATE) are brutally hard. A stock take on 22 September pulled
every claim into one table and the belief did not survive. Most "exact" functions had been
checked on a handful of rows, or on the same rows their fix was fitted to, or only in a July
fuzz sweep that never recorded a count. Once "any known differing row means the function is
not exact" became the rule, the picture flipped: a small set of well-evidenced functions, a
large grey middle, and no evidence at all for dozens.

The first thing the new parity ledger did was admit that.

## Excel had moved underneath us

Every stored judgement was from Excel builds 20026 to 20326. The machine's Excel had quietly
updated to 16.0.20430. The oracle cache, 797 MB of banked answers, was pinned to build 20131 in
a format the cache now refuses. Nothing was wrong with any of it; it simply described a
different program. "Characterized on build B" is the only honest sentence, and B moves.

## Twenty-four functions, one afternoon

The first live judging pass took 24 functions nobody had ever properly checked, generated a
fresh corpus for each (month-ends, the 31st, every basis, invalid arguments, negative zero,
subnormals), and asked Excel about 33,000 questions through the Value2 plumbing. Two and a
half minutes of Excel time.

Five came back clean on every row. Nineteen did not, and several of those were not last-bit
curiosities but plain rule differences: DOLLARDE(3.69, 10) is 3.69 in Excel and 9.9 in OxFunc;
TBILLEQ switches formula for bills longer than half a year and OxFunc never did; BAHTTEXT
refuses negative numbers that Excel happily spells out with a leading "minus".

## COUPDAYSNC and the non-additive calendar

The one fix that landed the same day is a small lesson about 30/360. COUPDAYSNC is "days from
settlement to the next coupon". The natural implementation counts those days directly. Excel
does not: on the US 30/360 basis it publishes COUPDAYS minus COUPDAYBS, the whole period minus
the part already accrued.

Those are the same number, unless the calendar is not additive. US 30/360 is not: whether the
31st becomes the 30th depends on what the *start* date was. Settle on 31 July 2020 with the
next coupon on 1 October: counted directly, 31 July becomes 30 July and the count is 61.
Counted as the period (90) minus the accrued part from 1 July to 31 July (30, because the start
was the 1st, so the 31st stays the 31st), it is 60. Excel says 60.

Three rows out of 1,060 in an old battery showed it; 65 fresh rows on the new build confirmed
it. The other bases are additive, so they never noticed.
