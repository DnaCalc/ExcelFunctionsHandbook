#requires -Version 5.1
<#
================================================================================
 Collect-ExcelExpData.ps1
================================================================================
 PURPOSE
   Measures exactly what your copy of Microsoft Excel (64-bit, Windows desktop)
   returns for =EXP(x) and =LN(x) on a fixed list of ~106 carefully chosen
   numbers, at the raw bit level, and saves them to a JSON file.  We compare
   these against a reference AMD machine to test whether Excel's results differ
   across CPUs (AMD vs Intel vs ARM).

 IS IT SAFE?  Yes.
   - It opens a NEW, hidden, temporary Excel instance and never touches any
     workbook you have open.  It saves nothing to your workbooks.
   - It only reads: your CPU model, OS/Excel version, and the numeric results
     of EXP/LN on the built-in test numbers.  It does NOT read your files or
     any personal content.  The single JSON output is plain text you can open
     and inspect before sending it back.
   - It cleans up the temporary Excel instance it created (and only that one).

 HOW TO RUN
   1. Save this file as Collect-ExcelExpData.ps1
   2. Open PowerShell, cd to where you saved it, and run:
          powershell -ExecutionPolicy Bypass -File .\Collect-ExcelExpData.ps1
      (or right-click -> Run with PowerShell).  Takes a few seconds.
   3. Send back the JSON file it reports at the end (saved to your Desktop by
      default; override with  -OutDir C:\some\folder ).

 REQUIREMENTS: Windows + desktop Excel (2016/2019/365).  Excel Online and Excel
   for Mac cannot be driven this way -- for Mac, ask the sender for the VBA
   version instead.
================================================================================
#>
[CmdletBinding()]
param([string]$OutDir)

$ErrorActionPreference = 'Stop'
$SchemaVersion = 'excel-exp-collect/1'
$ReferenceCpu  = 'AMD Ryzen 7 4800H (Zen2) | Excel 16.0 x64 | Windows 11 -- ref_amd == live Excel here (validated 294/294)'

# --- embedded test vectors (generated on the reference AMD machine) ----------
$VectorsJson = @'
{
 "schema": "excel-exp-vectors/1",
 "note": "ref_amd = live Excel on the reference AMD machine; ref_correct = correctly-rounded true value; ref_ideal = portable idealized x87 model. cat: microcode = amd!=ideal, bias = amd!=correct==ideal, control = all equal.",
 "reference_cpu": "AMD Ryzen 7 4800H (Zen2) | Excel 16.0 x64 | Windows 11 -- ref_amd == live Excel here (validated 294/294)",
 "rows": [
  {
   "fn": "LN",
   "x": "0d45fbbe4b0e7d55",
   "cat": "microcode",
   "k": null,
   "amd": "c0818e98ea6090b2",
   "correct": "c0818e98ea6090b3",
   "ideal": "c0818e98ea6090b3"
  },
  {
   "fn": "LN",
   "x": "0dbe4e3a44304ad6",
   "cat": "microcode",
   "k": null,
   "amd": "c081653674e5b8d8",
   "correct": "c081653674e5b8d9",
   "ideal": "c081653674e5b8d9"
  },
  {
   "fn": "LN",
   "x": "11f544676cf1bf2f",
   "cat": "microcode",
   "k": null,
   "amd": "c07fddf2ad7e124d",
   "correct": "c07fddf2ad7e124d",
   "ideal": "c07fddf2ad7e124c"
  },
  {
   "fn": "LN",
   "x": "1eb393c13e56f40a",
   "cat": "microcode",
   "k": null,
   "amd": "c07708d724444078",
   "correct": "c07708d724444077",
   "ideal": "c07708d724444077"
  },
  {
   "fn": "LN",
   "x": "23a4108745e19fab",
   "cat": "microcode",
   "k": null,
   "amd": "c0739c4f140e7517",
   "correct": "c0739c4f140e7517",
   "ideal": "c0739c4f140e7518"
  },
  {
   "fn": "LN",
   "x": "2d152de2df4ff1e8",
   "cat": "microcode",
   "k": null,
   "amd": "c06a2199ba6c182a",
   "correct": "c06a2199ba6c182b",
   "ideal": "c06a2199ba6c182b"
  },
  {
   "fn": "LN",
   "x": "2d20badbb5556728",
   "cat": "microcode",
   "k": null,
   "amd": "c06a12f75473f581",
   "correct": "c06a12f75473f581",
   "ideal": "c06a12f75473f580"
  },
  {
   "fn": "LN",
   "x": "2d3693905c941f14",
   "cat": "microcode",
   "k": null,
   "amd": "c069f331dcfe7f08",
   "correct": "c069f331dcfe7f07",
   "ideal": "c069f331dcfe7f07"
  },
  {
   "fn": "LN",
   "x": "2e68258fb8991289",
   "cat": "microcode",
   "k": null,
   "amd": "c0684b9c09fc3ba6",
   "correct": "c0684b9c09fc3ba5",
   "ideal": "c0684b9c09fc3ba5"
  },
  {
   "fn": "LN",
   "x": "420be30a078fed72",
   "cat": "microcode",
   "k": null,
   "amd": "40376deebfb7f46e",
   "correct": "40376deebfb7f46d",
   "ideal": "40376deebfb7f46d"
  },
  {
   "fn": "LN",
   "x": "4574658f6970c8b6",
   "cat": "microcode",
   "k": null,
   "amd": "404e9eafb5cc42c6",
   "correct": "404e9eafb5cc42c5",
   "ideal": "404e9eafb5cc42c5"
  },
  {
   "fn": "LN",
   "x": "461d6906690fdadb",
   "cat": "microcode",
   "k": null,
   "amd": "40512261215c415e",
   "correct": "40512261215c415d",
   "ideal": "40512261215c415d"
  },
  {
   "fn": "LN",
   "x": "4d7d2f67e7585ef8",
   "cat": "microcode",
   "k": null,
   "amd": "4062ca44831b0c3a",
   "correct": "4062ca44831b0c39",
   "ideal": "4062ca44831b0c39"
  },
  {
   "fn": "LN",
   "x": "4fb4742155e8cc52",
   "cat": "microcode",
   "k": null,
   "amd": "4065dd65ac726cee",
   "correct": "4065dd65ac726ced",
   "ideal": "4065dd65ac726ced"
  },
  {
   "fn": "LN",
   "x": "501baf60a3904e63",
   "cat": "microcode",
   "k": null,
   "amd": "40666c2b3ad75615",
   "correct": "40666c2b3ad75615",
   "ideal": "40666c2b3ad75614"
  },
  {
   "fn": "LN",
   "x": "559d2eada7837182",
   "cat": "microcode",
   "k": null,
   "amd": "406e0dc1bc5d2a56",
   "correct": "406e0dc1bc5d2a55",
   "ideal": "406e0dc1bc5d2a55"
  },
  {
   "fn": "LN",
   "x": "603a2ee36db86f49",
   "cat": "microcode",
   "k": null,
   "amd": "40766280e9a44885",
   "correct": "40766280e9a44885",
   "ideal": "40766280e9a44886"
  },
  {
   "fn": "LN",
   "x": "6fffb3b90594a629",
   "cat": "microcode",
   "k": null,
   "amd": "4080a82aa75e37f7",
   "correct": "4080a82aa75e37f7",
   "ideal": "4080a82aa75e37f6"
  },
  {
   "fn": "EXP",
   "x": "c0679449ecbea54d",
   "cat": "microcode",
   "k": -272,
   "amd": "2eed0355d8d43d6f",
   "correct": "2eed0355d8d43d6f",
   "ideal": "2eed0355d8d43d6e"
  },
  {
   "fn": "EXP",
   "x": "3ffd0ea1eed55587",
   "cat": "microcode",
   "k": 3,
   "amd": "401897359943e077",
   "correct": "401897359943e076",
   "ideal": "401897359943e076"
  },
  {
   "fn": "EXP",
   "x": "3fcf0d047067aa41",
   "cat": "microcode",
   "k": 0,
   "amd": "3ff464834a8f9e6e",
   "correct": "3ff464834a8f9e6d",
   "ideal": "3ff464834a8f9e6d"
  },
  {
   "fn": "EXP",
   "x": "c0269699e76adf2b",
   "cat": "microcode",
   "k": -16,
   "amd": "3eea19af09c7214d",
   "correct": "3eea19af09c7214e",
   "ideal": "3eea19af09c7214e"
  },
  {
   "fn": "EXP",
   "x": "bfebeaf135acdf8d",
   "cat": "microcode",
   "k": -1,
   "amd": "3fdabf723fe34b62",
   "correct": "3fdabf723fe34b61",
   "ideal": "3fdabf723fe34b61"
  },
  {
   "fn": "EXP",
   "x": "c05ef32ff789928c",
   "cat": "microcode",
   "k": -179,
   "amd": "34c508a816adb8c2",
   "correct": "34c508a816adb8c3",
   "ideal": "34c508a816adb8c3"
  },
  {
   "fn": "EXP",
   "x": "bfd7733f1a98fc89",
   "cat": "microcode",
   "k": -1,
   "amd": "3fe62ed9f0381bae",
   "correct": "3fe62ed9f0381baf",
   "ideal": "3fe62ed9f0381baf"
  },
  {
   "fn": "EXP",
   "x": "bf9c3df9172646c1",
   "cat": "microcode",
   "k": 0,
   "amd": "3fef21268c727439",
   "correct": "3fef21268c727439",
   "ideal": "3fef21268c727438"
  },
  {
   "fn": "EXP",
   "x": "404a9ab4981feb6c",
   "cat": "microcode",
   "k": 77,
   "amd": "44bb2b01230ce58c",
   "correct": "44bb2b01230ce58c",
   "ideal": "44bb2b01230ce58b"
  },
  {
   "fn": "EXP",
   "x": "4018253d6552d2ce",
   "cat": "microcode",
   "k": 9,
   "amd": "407a25ece8a4620c",
   "correct": "407a25ece8a4620b",
   "ideal": "407a25ece8a4620b"
  },
  {
   "fn": "LN",
   "x": "3e8745b9995cbfbe",
   "cat": "microcode",
   "k": null,
   "amd": "c02f22aa897c0721",
   "correct": "c02f22aa897c0721",
   "ideal": "c02f22aa897c0720"
  },
  {
   "fn": "LN",
   "x": "4143f3fcbbb376b5",
   "cat": "microcode",
   "k": null,
   "amd": "402d8dc3ea722c2a",
   "correct": "402d8dc3ea722c29",
   "ideal": "402d8dc3ea722c29"
  },
  {
   "fn": "LN",
   "x": "196742f4a04e17c5",
   "cat": "microcode",
   "k": null,
   "amd": "c07ab4c2f5a65df6",
   "correct": "c07ab4c2f5a65df7",
   "ideal": "c07ab4c2f5a65df7"
  },
  {
   "fn": "LN",
   "x": "29e190bf0a3bd3c8",
   "cat": "microcode",
   "k": null,
   "amd": "c06e92cdb6ba751a",
   "correct": "c06e92cdb6ba7519",
   "ideal": "c06e92cdb6ba7519"
  },
  {
   "fn": "LN",
   "x": "3f5216a310c42173",
   "cat": "microcode",
   "k": null,
   "amd": "c01b3c32fd4a9268",
   "correct": "c01b3c32fd4a9269",
   "ideal": "c01b3c32fd4a9269"
  },
  {
   "fn": "LN",
   "x": "2c58290e4b16f55a",
   "cat": "microcode",
   "k": null,
   "amd": "c06b278e0bac5120",
   "correct": "c06b278e0bac511f",
   "ideal": "c06b278e0bac511f"
  },
  {
   "fn": "EXP",
   "x": "3fc173bb16e7be7c",
   "cat": "bias",
   "k": 0,
   "amd": "3ff2565441d3e7a6",
   "correct": "3ff2565441d3e7a7",
   "ideal": "3ff2565441d3e7a6"
  },
  {
   "fn": "EXP",
   "x": "bfacee00e2b2a2c5",
   "cat": "bias",
   "k": 0,
   "amd": "3fee3df57bc67f54",
   "correct": "3fee3df57bc67f55",
   "ideal": "3fee3df57bc67f54"
  },
  {
   "fn": "EXP",
   "x": "3fdd263047b0c16c",
   "cat": "bias",
   "k": 1,
   "amd": "3ff93af325edf4b0",
   "correct": "3ff93af325edf4af",
   "ideal": "3ff93af325edf4b0"
  },
  {
   "fn": "EXP",
   "x": "bfde9c004b6357a9",
   "cat": "bias",
   "k": -1,
   "amd": "3fe3d5d7bdbd765a",
   "correct": "3fe3d5d7bdbd7659",
   "ideal": "3fe3d5d7bdbd765a"
  },
  {
   "fn": "EXP",
   "x": "3ff065466245de48",
   "cat": "bias",
   "k": 1,
   "amd": "40064a67654ed174",
   "correct": "40064a67654ed175",
   "ideal": "40064a67654ed174"
  },
  {
   "fn": "EXP",
   "x": "3ff99e646b9e83f2",
   "cat": "bias",
   "k": 2,
   "amd": "4013d5d7bdbd7a04",
   "correct": "4013d5d7bdbd7a05",
   "ideal": "4013d5d7bdbd7a04"
  },
  {
   "fn": "EXP",
   "x": "bff2be2192563c64",
   "cat": "bias",
   "k": -2,
   "amd": "3fd3d5d7bdbd1b38",
   "correct": "3fd3d5d7bdbd1b39",
   "ideal": "3fd3d5d7bdbd1b38"
  },
  {
   "fn": "EXP",
   "x": "c00cd0576cec1cc0",
   "cat": "bias",
   "k": -5,
   "amd": "3f9bee5fd944d2d6",
   "correct": "3f9bee5fd944d2d5",
   "ideal": "3f9bee5fd944d2d6"
  },
  {
   "fn": "EXP",
   "x": "400d71e4750b228e",
   "cat": "bias",
   "k": 5,
   "amd": "4043d5d7bdbdfdc0",
   "correct": "4043d5d7bdbdfdc1",
   "ideal": "4043d5d7bdbdfdc0"
  },
  {
   "fn": "EXP",
   "x": "4010480a4a9cf1a6",
   "cat": "bias",
   "k": 6,
   "amd": "404d49ed9f2dc4bb",
   "correct": "404d49ed9f2dc4ba",
   "ideal": "404d49ed9f2dc4bb"
  },
  {
   "fn": "EXP",
   "x": "401bb28c695e2dfc",
   "cat": "bias",
   "k": 10,
   "amd": "408fc5fa2fa2c4d4",
   "correct": "408fc5fa2fa2c4d3",
   "ideal": "408fc5fa2fa2c4d4"
  },
  {
   "fn": "EXP",
   "x": "401c95dc19e21f17",
   "cat": "bias",
   "k": 10,
   "amd": "4093d5d7bdbecc0f",
   "correct": "4093d5d7bdbecc0e",
   "ideal": "4093d5d7bdbecc0f"
  },
  {
   "fn": "EXP",
   "x": "c01addcb638fdc4a",
   "cat": "bias",
   "k": -10,
   "amd": "3f53d5d7bdbcbbbb",
   "correct": "3f53d5d7bdbcbbba",
   "ideal": "3f53d5d7bdbcbbbb"
  },
  {
   "fn": "EXP",
   "x": "c020fe9920d782b6",
   "cat": "bias",
   "k": -12,
   "amd": "3f2abdfbea140b2e",
   "correct": "3f2abdfbea140b2f",
   "ideal": "3f2abdfbea140b2e"
  },
  {
   "fn": "EXP",
   "x": "c021d0d6574e60b2",
   "cat": "bias",
   "k": -13,
   "amd": "3f21bc8218f66473",
   "correct": "3f21bc8218f66474",
   "ideal": "3f21bc8218f66473"
  },
  {
   "fn": "EXP",
   "x": "c0239a74d0d0d928",
   "cat": "bias",
   "k": -14,
   "amd": "3f0d062686a59f97",
   "correct": "3f0d062686a59f98",
   "ideal": "3f0d062686a59f97"
  },
  {
   "fn": "EXP",
   "x": "402a49858f97b520",
   "cat": "bias",
   "k": 19,
   "amd": "411f2c298d7fa741",
   "correct": "411f2c298d7fa740",
   "ideal": "411f2c298d7fa741"
  },
  {
   "fn": "EXP",
   "x": "402e11ef696a1f3c",
   "cat": "bias",
   "k": 22,
   "amd": "4149d46babb6823e",
   "correct": "4149d46babb6823d",
   "ideal": "4149d46babb6823e"
  },
  {
   "fn": "EXP",
   "x": "40306f177be5e788",
   "cat": "bias",
   "k": 24,
   "amd": "416a2873b102b747",
   "correct": "416a2873b102b746",
   "ideal": "416a2873b102b747"
  },
  {
   "fn": "EXP",
   "x": "40318b266dfdb55c",
   "cat": "bias",
   "k": 25,
   "amd": "4183d5d7bdbc8187",
   "correct": "4183d5d7bdbc8188",
   "ideal": "4183d5d7bdbc8187"
  },
  {
   "fn": "EXP",
   "x": "c0311d224069385f",
   "cat": "bias",
   "k": -25,
   "amd": "3e63d5d7bdbd7f43",
   "correct": "3e63d5d7bdbd7f44",
   "ideal": "3e63d5d7bdbd7f43"
  },
  {
   "fn": "EXP",
   "x": "403279a48fa8e770",
   "cat": "bias",
   "k": 27,
   "amd": "41992d4d3c76b666",
   "correct": "41992d4d3c76b665",
   "ideal": "41992d4d3c76b666"
  },
  {
   "fn": "EXP",
   "x": "c033d4d480cb8ab0",
   "cat": "bias",
   "k": -29,
   "amd": "3e24f514e31f45fd",
   "correct": "3e24f514e31f45fe",
   "ideal": "3e24f514e31f45fd"
  },
  {
   "fn": "EXP",
   "x": "40490f8c6a3eb1b6",
   "cat": "bias",
   "k": 72,
   "amd": "4473d5d7bdbfae36",
   "correct": "4473d5d7bdbfae35",
   "ideal": "4473d5d7bdbfae36"
  },
  {
   "fn": "EXP",
   "x": "c048d88a5374610f",
   "cat": "bias",
   "k": -72,
   "amd": "3b73d5d7bdbd22ee",
   "correct": "3b73d5d7bdbd22ef",
   "ideal": "3b73d5d7bdbd22ee"
  },
  {
   "fn": "EXP",
   "x": "405901cbe48c19c9",
   "cat": "bias",
   "k": 144,
   "amd": "48f3d5d7bdbfc950",
   "correct": "48f3d5d7bdbfc94f",
   "ideal": "48f3d5d7bdbfc950"
  },
  {
   "fn": "EXP",
   "x": "406a0516c5a6848f",
   "cat": "bias",
   "k": 300,
   "amd": "52b3d5d7bdbd8810",
   "correct": "52b3d5d7bdbd880f",
   "ideal": "52b3d5d7bdbd8810"
  },
  {
   "fn": "EXP",
   "x": "c069f7563ff3ef6a",
   "cat": "bias",
   "k": -300,
   "amd": "2d33d5d7bdbfe4d1",
   "correct": "2d33d5d7bdbfe4d2",
   "ideal": "2d33d5d7bdbfe4d1"
  },
  {
   "fn": "EXP",
   "x": "c0706c76635ad94d",
   "cat": "bias",
   "k": -379,
   "amd": "283da78d83db5ba1",
   "correct": "283da78d83db5ba2",
   "ideal": "283da78d83db5ba1"
  },
  {
   "fn": "EXP",
   "x": "4075ac9d8e6cfcbb",
   "cat": "bias",
   "k": 500,
   "amd": "5f33d5d7bdb859a5",
   "correct": "5f33d5d7bdb859a6",
   "ideal": "5f33d5d7bdb859a5"
  },
  {
   "fn": "EXP",
   "x": "c076b708d72d4baa",
   "cat": "bias",
   "k": -524,
   "amd": "1f2969555313862e",
   "correct": "1f2969555313862f",
   "ideal": "1f2969555313862e"
  },
  {
   "fn": "EXP",
   "x": "407e56afba06bfda",
   "cat": "bias",
   "k": 700,
   "amd": "6bb3d5d7bdbdeac4",
   "correct": "6bb3d5d7bdbdeac3",
   "ideal": "6bb3d5d7bdbdeac4"
  },
  {
   "fn": "EXP",
   "x": "c07e4fcf772d7ab8",
   "cat": "bias",
   "k": -700,
   "amd": "1433d5d7bdb8c3d3",
   "correct": "1433d5d7bdb8c3d2",
   "ideal": "1433d5d7bdb8c3d3"
  },
  {
   "fn": "EXP",
   "x": "4085aae57db6b1fb",
   "cat": "bias",
   "k": 1000,
   "amd": "7e73d5d7bdc58ed3",
   "correct": "7e73d5d7bdc58ed2",
   "ideal": "7e73d5d7bdc58ed3"
  },
  {
   "fn": "LN",
   "x": "3feb575c5dbcc5fc",
   "cat": "bias",
   "k": null,
   "amd": "bfc423b12e82d254",
   "correct": "bfc423b12e82d253",
   "ideal": "bfc423b12e82d254"
  },
  {
   "fn": "LN",
   "x": "3ff4ed22766962c0",
   "cat": "bias",
   "k": null,
   "amd": "3fd12dc431cd9536",
   "correct": "3fd12dc431cd9537",
   "ideal": "3fd12dc431cd9536"
  },
  {
   "fn": "LN",
   "x": "3ff76dd708082eb6",
   "cat": "bias",
   "k": null,
   "amd": "3fd868ab5b84186a",
   "correct": "3fd868ab5b841869",
   "ideal": "3fd868ab5b84186a"
  },
  {
   "fn": "LN",
   "x": "3ff894270f14b346",
   "cat": "bias",
   "k": null,
   "amd": "3fdb79869aa8114e",
   "correct": "3fdb79869aa8114d",
   "ideal": "3fdb79869aa8114e"
  },
  {
   "fn": "LN",
   "x": "09d4dd4d0d12aa46",
   "cat": "bias",
   "k": null,
   "amd": "c082c00000000008",
   "correct": "c082c00000000009",
   "ideal": "c082c00000000008"
  },
  {
   "fn": "LN",
   "x": "09d4dd4d0d12b219",
   "cat": "bias",
   "k": null,
   "amd": "c082c00000000006",
   "correct": "c082c00000000005",
   "ideal": "c082c00000000006"
  },
  {
   "fn": "LN",
   "x": "2060a44a100c725f",
   "cat": "bias",
   "k": null,
   "amd": "c075e00000000038",
   "correct": "c075e00000000037",
   "ideal": "c075e00000000038"
  },
  {
   "fn": "LN",
   "x": "2060a44a100c81f9",
   "cat": "bias",
   "k": null,
   "amd": "c075e00000000028",
   "correct": "c075e00000000029",
   "ideal": "c075e00000000028"
  },
  {
   "fn": "LN",
   "x": "36ea8c1f14e2a746",
   "cat": "bias",
   "k": null,
   "amd": "c059000000000014",
   "correct": "c059000000000013",
   "ideal": "c059000000000014"
  },
  {
   "fn": "LN",
   "x": "36ea8c1f14e2de07",
   "cat": "bias",
   "k": null,
   "amd": "c058ffffffffff90",
   "correct": "c058ffffffffff8f",
   "ideal": "c058ffffffffff90"
  },
  {
   "fn": "LN",
   "x": "3e21b48655f3545a",
   "cat": "bias",
   "k": null,
   "amd": "c0340000000001b2",
   "correct": "c0340000000001b3",
   "ideal": "c0340000000001b2"
  },
  {
   "fn": "LN",
   "x": "3e21b48655f365b6",
   "cat": "bias",
   "k": null,
   "amd": "c0340000000000b8",
   "correct": "c0340000000000b7",
   "ideal": "c0340000000000b8"
  },
  {
   "fn": "LN",
   "x": "3fb50385c094d684",
   "cat": "bias",
   "k": null,
   "amd": "c004000000000b48",
   "correct": "c004000000000b47",
   "ideal": "c004000000000b48"
  },
  {
   "fn": "LN",
   "x": "3fb50385c0951bde",
   "cat": "bias",
   "k": null,
   "amd": "c003fffffffff0e0",
   "correct": "c003fffffffff0e1",
   "ideal": "c003fffffffff0e0"
  },
  {
   "fn": "LN",
   "x": "3fdfc80db9dd4377",
   "cat": "bias",
   "k": null,
   "amd": "bfe6666666667850",
   "correct": "bfe6666666667851",
   "ideal": "bfe6666666667850"
  },
  {
   "fn": "LN",
   "x": "3fdfc80db9dd80d1",
   "cat": "bias",
   "k": null,
   "amd": "bfe6666666663a8a",
   "correct": "bfe6666666663a8b",
   "ideal": "bfe6666666663a8a"
  },
  {
   "fn": "LN",
   "x": "3fed8a2b4ac439fa",
   "cat": "bias",
   "k": null,
   "amd": "bfb47ae147ae8130",
   "correct": "bfb47ae147ae8131",
   "ideal": "bfb47ae147ae8130"
  },
  {
   "fn": "LN",
   "x": "3fed8a2b4ac475c8",
   "cat": "bias",
   "k": null,
   "amd": "bfb47ae147ac7ae8",
   "correct": "bfb47ae147ac7ae7",
   "ideal": "bfb47ae147ac7ae8"
  },
  {
   "fn": "LN",
   "x": "3ff15524d1fd7781",
   "cat": "bias",
   "k": null,
   "amd": "3fb47ae147ad9ae2",
   "correct": "3fb47ae147ad9ae1",
   "ideal": "3fb47ae147ad9ae2"
  },
  {
   "fn": "LN",
   "x": "3ff15524d1fd9490",
   "cat": "bias",
   "k": null,
   "amd": "3fb47ae147af4812",
   "correct": "3fb47ae147af4813",
   "ideal": "3fb47ae147af4812"
  },
  {
   "fn": "EXP",
   "x": "3fed0e85d843098c",
   "cat": "control",
   "k": 1,
   "amd": "4003d5d7bdbd9ef4",
   "correct": "4003d5d7bdbd9ef4",
   "ideal": "4003d5d7bdbd9ef4"
  },
  {
   "fn": "EXP",
   "x": "3ff99e646b9e1216",
   "cat": "control",
   "k": 2,
   "amd": "4013d5d7bdbcecde",
   "correct": "4013d5d7bdbcecde",
   "ideal": "4013d5d7bdbcecde"
  },
  {
   "fn": "EXP",
   "x": "400d71e4750afc9a",
   "cat": "control",
   "k": 5,
   "amd": "4043d5d7bdbd9fa7",
   "correct": "4043d5d7bdbd9fa7",
   "ideal": "4043d5d7bdbd9fa7"
  },
  {
   "fn": "EXP",
   "x": "40318b266dfd98e5",
   "cat": "control",
   "k": 25,
   "amd": "4183d5d7bdba4cec",
   "correct": "4183d5d7bdba4cec",
   "ideal": "4183d5d7bdba4cec"
  },
  {
   "fn": "EXP",
   "x": "bfde9c004b63837f",
   "cat": "control",
   "k": -1,
   "amd": "3fe3d5d7bdbd68c4",
   "correct": "3fe3d5d7bdbd68c4",
   "ideal": "3fe3d5d7bdbd68c4"
  },
  {
   "fn": "EXP",
   "x": "bff2be2192561670",
   "cat": "control",
   "k": -2,
   "amd": "3fd3d5d7bdbd4a46",
   "correct": "3fd3d5d7bdbd4a46",
   "ideal": "3fd3d5d7bdbd4a46"
  },
  {
   "fn": "EXP",
   "x": "c01addcb638fc950",
   "cat": "control",
   "k": -10,
   "amd": "3f53d5d7bdbd19d4",
   "correct": "3f53d5d7bdbd19d4",
   "ideal": "3f53d5d7bdbd19d4"
  },
  {
   "fn": "EXP",
   "x": "3fe19aeb55d57a5b",
   "cat": "control",
   "k": 1,
   "amd": "3ffbbc8c3afaa23a",
   "correct": "3ffbbc8c3afaa23a",
   "ideal": "3ffbbc8c3afaa23a"
  },
  {
   "fn": "EXP",
   "x": "3fe4a631cec5f8da",
   "cat": "control",
   "k": 1,
   "amd": "3ffe812ac57fe9e1",
   "correct": "3ffe812ac57fe9e1",
   "ideal": "3ffe812ac57fe9e1"
  },
  {
   "fn": "EXP",
   "x": "3ff1f4e8c801c7ce",
   "cat": "control",
   "k": 2,
   "amd": "40089339f0f4de59",
   "correct": "40089339f0f4de59",
   "ideal": "40089339f0f4de59"
  },
  {
   "fn": "LN",
   "x": "2060a44a100cac9e",
   "cat": "control",
   "k": null,
   "amd": "c075e00000000000",
   "correct": "c075e00000000000",
   "ideal": "c075e00000000000"
  },
  {
   "fn": "LN",
   "x": "36ea8c1f14e2ed60",
   "cat": "control",
   "k": null,
   "amd": "c058ffffffffff6a",
   "correct": "c058ffffffffff6a",
   "ideal": "c058ffffffffff6a"
  },
  {
   "fn": "LN",
   "x": "3e21b48655f38dd3",
   "cat": "control",
   "k": null,
   "amd": "c033fffffffffe74",
   "correct": "c033fffffffffe74",
   "ideal": "c033fffffffffe74"
  },
  {
   "fn": "LN",
   "x": "3fb50385c094eda2",
   "cat": "control",
   "k": null,
   "amd": "c00400000000027a",
   "correct": "c00400000000027a",
   "ideal": "c00400000000027a"
  },
  {
   "fn": "LN",
   "x": "3fed8a2b4ac40535",
   "cat": "control",
   "k": null,
   "amd": "bfb47ae147b04a82",
   "correct": "bfb47ae147b04a82",
   "ideal": "bfb47ae147b04a82"
  },
  {
   "fn": "LN",
   "x": "3ff15524d1fd5a72",
   "cat": "control",
   "k": null,
   "amd": "3fb47ae147abedb0",
   "correct": "3fb47ae147abedb0",
   "ideal": "3fb47ae147abedb0"
  },
  {
   "fn": "LN",
   "x": "40285d6fd931ad76",
   "cat": "control",
   "k": null,
   "amd": "4003ffffffffef2a",
   "correct": "4003ffffffffef2a",
   "ideal": "4003ffffffffef2a"
  },
  {
   "fn": "LN",
   "x": "41bceb088b691218",
   "cat": "control",
   "k": null,
   "amd": "4034000000000174",
   "correct": "4034000000000174",
   "ideal": "4034000000000174"
  }
 ]
}
'@

function Convert-BitsToDouble {
    param([string]$Hex)
    $u = [System.UInt64]::Parse($Hex, [System.Globalization.NumberStyles]::HexNumber)
    $l = [System.BitConverter]::ToInt64([System.BitConverter]::GetBytes($u), 0)
    return [System.BitConverter]::Int64BitsToDouble($l)
}

function Convert-DoubleToBits {
    param([double]$D)
    return ('{0:x16}' -f [System.BitConverter]::DoubleToInt64Bits($D))
}

function Get-Environment {
    $info = [ordered]@{}
    try {
        $cpu = Get-CimInstance Win32_Processor -ErrorAction Stop | Select-Object -First 1
        $info.cpu_name              = $cpu.Name
        $info.cpu_manufacturer      = $cpu.Manufacturer
        $info.cpu_architecture_code = [int]$cpu.Architecture   # 0=x86 9=x64 12=ARM64
    } catch { $info.cpu_error = "$_" }
    try {
        $os = Get-CimInstance Win32_OperatingSystem -ErrorAction Stop
        $info.os_caption      = $os.Caption
        $info.os_version      = $os.Version
        $info.os_architecture = $os.OSArchitecture
    } catch {}
    $info.env_processor_architecture   = $env:PROCESSOR_ARCHITECTURE
    $info.env_processor_architew6432   = $env:PROCESSOR_ARCHITEW6432   # set when emulated
    $info.powershell_pointer_bits      = [System.IntPtr]::Size * 8
    $info.computer_name                = $env:COMPUTERNAME
    return $info
}

$rows = ($VectorsJson | ConvertFrom-Json).rows
Write-Host ("Loaded {0} test vectors." -f $rows.Count)

if (-not $OutDir) {
    try { $OutDir = [Environment]::GetFolderPath('Desktop') } catch { $OutDir = $null }
    if (-not $OutDir) { $OutDir = (Get-Location).Path }
}

$environment = Get-Environment
Write-Host ("CPU : {0}  [{1}]" -f $environment.cpu_name, $environment.cpu_manufacturer)
Write-Host ("OS  : {0} ({1})" -f $environment.os_caption, $environment.os_architecture)

$preExisting = @()
try { $preExisting = @(Get-Process -Name EXCEL -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Id) } catch {}

$xl = $null; $wb = $null; $ws = $null; $newPid = $null
try {
    try {
        $xl = New-Object -ComObject Excel.Application
    } catch {
        throw ("Could not start desktop Excel via COM. This script needs Excel " +
               "for Windows (desktop) -- not Excel Online or Excel for Mac. Error: $_")
    }
    $xl.Visible = $false
    $xl.DisplayAlerts = $false
    try { $xl.ScreenUpdating = $false } catch {}

    $excelInfo = [ordered]@{ version = "$($xl.Version)"; build = "$($xl.Build)" }
    try {
        $ids = @(Get-Process -Name EXCEL -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Id)
        $newPid = ($ids | Where-Object { $preExisting -notcontains $_ } | Select-Object -First 1)
        if ($newPid) {
            $p = Get-Process -Id $newPid -ErrorAction SilentlyContinue
            if ($p) {
                $excelInfo.process_path = $p.Path
                $excelInfo.bitness = if ($p.Path -like '*Program Files (x86)*') { '32-bit' } else { '64-bit' }
            }
        }
    } catch {}
    Write-Host ("Excel: version {0} build {1} ({2})" -f $excelInfo.version, $excelInfo.build,
                ($(if ($excelInfo.bitness) { $excelInfo.bitness } else { 'bitness?' })))

    $wb = $xl.Workbooks.Add()
    $ws = $wb.Worksheets.Item(1)

    # Write each exact double via Value2 (binary64, no text parsing -> culture
    # independent) and a formula that references the input cell.
    for ($i = 0; $i -lt $rows.Count; $i++) {
        $r = $i + 1
        $ws.Cells.Item($r, 1).Value2 = (Convert-BitsToDouble $rows[$i].x)
        if ($rows[$i].fn -eq 'EXP') { $ws.Cells.Item($r, 2).Formula = "=EXP(A$r)" }
        else                        { $ws.Cells.Item($r, 2).Formula = "=LN(A$r)" }
    }
    $xl.CalculateFull()

    $out = New-Object System.Collections.ArrayList
    $mAmd = 0; $mCor = 0; $mIdeal = 0; $rtOk = 0; $errs = 0
    for ($i = 0; $i -lt $rows.Count; $i++) {
        $r = $i + 1
        $inBack = Convert-DoubleToBits ([double]$ws.Cells.Item($r, 1).Value2)
        $rt = ($inBack -eq $rows[$i].x)
        if ($rt) { $rtOk++ }

        $cell = $ws.Cells.Item($r, 2).Value2
        $isErr = -not ($cell -is [double])
        $excelOut = $null; $errText = $null
        if ($isErr) {
            $errs++; $errText = "$cell"
        } else {
            $excelOut = Convert-DoubleToBits ([double]$cell)
            if ($excelOut -eq $rows[$i].amd)     { $mAmd++ }
            if ($excelOut -eq $rows[$i].correct) { $mCor++ }
            if ($excelOut -eq $rows[$i].ideal)   { $mIdeal++ }
        }

        [void]$out.Add([ordered]@{
            fn                  = $rows[$i].fn
            x                   = $rows[$i].x
            cat                 = $rows[$i].cat
            k                   = $rows[$i].k
            excel_out           = $excelOut
            input_roundtrip_ok  = $rt
            is_error            = $isErr
            error_text          = $errText
            ref_amd             = $rows[$i].amd
            ref_correct         = $rows[$i].correct
            ref_ideal           = $rows[$i].ideal
        })
    }

    $summary = [ordered]@{
        rows                       = $rows.Count
        input_roundtrip_exact      = $rtOk
        formula_errors             = $errs
        excel_eq_amd_reference     = $mAmd
        excel_eq_correctly_rounded = $mCor
        excel_eq_ideal_x87_model   = $mIdeal
    }

    $result = [ordered]@{
        schema        = $SchemaVersion
        reference_cpu = $ReferenceCpu
        collected_utc = (Get-Date).ToUniversalTime().ToString('o')
        environment   = $environment
        excel         = $excelInfo
        summary       = $summary
        rows          = $out
    }

    $tag = ($environment.cpu_manufacturer -replace '[^A-Za-z0-9]', '')
    if (-not $tag) { $tag = 'CPU' }
    $stamp   = (Get-Date).ToString('yyyyMMdd-HHmmss')
    $outfile = Join-Path $OutDir ("ExcelExpData_{0}_{1}.json" -f $tag, $stamp)
    $json    = $result | ConvertTo-Json -Depth 8
    [System.IO.File]::WriteAllText($outfile, $json, (New-Object System.Text.UTF8Encoding($false)))

    Write-Host ""
    Write-Host "==================== Summary ===================="
    Write-Host ("  test vectors                : {0}" -f $summary.rows)
    Write-Host ("  input round-trip exact      : {0}/{1}" -f $summary.input_roundtrip_exact, $summary.rows)
    Write-Host ("  Excel == AMD reference      : {0}/{1}" -f $summary.excel_eq_amd_reference, $summary.rows)
    Write-Host ("  Excel == correctly-rounded  : {0}/{1}" -f $summary.excel_eq_correctly_rounded, $summary.rows)
    Write-Host ("  Excel == ideal x87 model    : {0}/{1}" -f $summary.excel_eq_ideal_x87_model, $summary.rows)
    if ($summary.formula_errors) { Write-Host ("  formula errors (unexpected) : {0}" -f $summary.formula_errors) }
    Write-Host "================================================="
    Write-Host ""
    if ($summary.input_roundtrip_exact -ne $summary.rows) {
        Write-Host "WARNING: some inputs did not round-trip exactly -- please mention this." -ForegroundColor Yellow
    }
    Write-Host ("SAVED: {0}" -f $outfile) -ForegroundColor Green
    Write-Host "Please send that JSON file back. Thank you!"
}
finally {
    try { if ($wb) { $wb.Close($false) } } catch {}
    try { if ($xl) { $xl.Quit() } } catch {}
    foreach ($obj in @($ws, $wb, $xl)) {
        try { if ($obj) { [void][System.Runtime.InteropServices.Marshal]::ReleaseComObject($obj) } } catch {}
    }
    [System.GC]::Collect(); [System.GC]::WaitForPendingFinalizers()
    if ($newPid -and ($preExisting -notcontains $newPid)) {
        try {
            $still = Get-Process -Id $newPid -ErrorAction SilentlyContinue
            if ($still) { Stop-Process -Id $newPid -Force -ErrorAction SilentlyContinue }
        } catch {}
    }
}
