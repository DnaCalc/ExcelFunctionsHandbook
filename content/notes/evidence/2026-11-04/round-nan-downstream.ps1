<#
How Excel treats the NaN bit pattern ROUND returns near DBL_MAX, downstream of the cell.
Same rules as codex-runs\spot-check\excel_runner.ps1: its own Excel instance (PID checked),
a new blank workbook, inputs as doubles through Value2, quits only that instance. The workbook
is saved once as .xlsx into this script's folder so the stored <v> can be read.
#>
param([Parameter(Mandatory)] [string] $Out)
$ErrorActionPreference = 'Stop'
Add-Type -Namespace NanProbe -Name Win32 -MemberDefinition @'
[DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(System.IntPtr hWnd, out uint pid);
'@
function ToBits([double] $d) { '0x{0:x16}' -f [BitConverter]::DoubleToInt64Bits($d) }
function Encode($v) {
    if ($null -eq $v) { return 'empty' }
    if ($v -is [double]) { return ToBits $v }
    if ($v -is [int]) { return "error:$v" }
    if ($v -is [bool]) { return "logical:$v" }
    return "text:$v"
}
$before = @(Get-Process EXCEL -ErrorAction SilentlyContinue | ForEach-Object Id)
$xl = New-Object -ComObject Excel.Application
[uint32] $xlPid = 0
[void][NanProbe.Win32]::GetWindowThreadProcessId([IntPtr]$xl.Hwnd, [ref]$xlPid)
if ($xlPid -eq 0 -or $before -contains $xlPid) {
    [void][Runtime.InteropServices.Marshal]::ReleaseComObject($xl)
    throw "COM returned Excel PID $xlPid, which this script did not start"
}
Write-Host "nan-probe: started Excel PID $xlPid"
$result = [ordered]@{ pid = $xlPid; build = $null; cells = [ordered]@{} }
$xlsx = Join-Path (Split-Path $Out) 'nan-downstream.xlsx'
try {
    $xl.DisplayAlerts = $false
    $result.build = $xl.Build
    $wb = $xl.Workbooks.Add()
    $ws = $wb.Worksheets.Item(1)
    $ws.Range('E1').Value2 = [double]1.7976931348623157E308
    $ws.Range('E2').Value2 = [double]-308
    $ws.Range('E3').Value2 = [double]-307
    $ws.Range('E4').Value2 = [double]0
    $formulas = [ordered]@{
        A1 = '=ROUND(E1,E2)'; A2 = '=ROUND(E1,E3)'; A3 = '=ROUND(E1,E4)'; A4 = '=ROUND(-E1,E2)'
        B1 = '=ISNUMBER(A1)'; B2 = '=ISERROR(A1)'; B3 = '=A1+0'; B4 = '=A1*1'; B5 = '=A1=A1'
        B6 = '=TEXT(A1,"0")'; B7 = '=ROUND(A1,0)'; B8 = '=ERROR.TYPE(A1)'; B9 = '=A1'
        B10 = '=SUM(A1)'; B11 = '=SUM(A1:A2)'; B12 = '=ISNUMBER(A2)'; B13 = '=A2+0'; B14 = '=TYPE(A1)'
        B15 = '=MAX(A1)'; B16 = '=IFERROR(A1,"err")'; B17 = '=A1>0'; B18 = '=ROUND(A3,0)'; B19 = '=N(A1)'
        B20 = '=A1-A1'; B21 = '=ABS(A1)'; B22 = '=-A1'; B23 = '=A4'; B24 = '=ISNUMBER(A3)'
    }
    foreach ($k in $formulas.Keys) { $ws.Range($k).Formula = $formulas[$k] }
    $xl.CalculateFull()
    foreach ($k in $formulas.Keys) {
        $c = $ws.Range($k)
        $result.cells[$k] = [ordered]@{ formula = $formulas[$k]; value2 = Encode $c.Value2; text = [string]$c.Text }
    }
    $wb.SaveAs($xlsx, 51)
    $wb.Close($false)
    $wb2 = $xl.Workbooks.Open($xlsx)
    $ws2 = $wb2.Worksheets.Item(1)
    $result.reopened = [ordered]@{}
    foreach ($k in @('A1','A2','A3','A4','B1','B3','B9')) { $result.reopened[$k] = Encode $ws2.Range($k).Value2 }
    $xl.CalculateFull()
    $result.recalculated = [ordered]@{}
    foreach ($k in @('A1','A2','A3','A4','B1','B3','B9')) { $result.recalculated[$k] = Encode $ws2.Range($k).Value2 }
    $wb2.Close($false)
}
finally {
    try { $xl.Quit() } catch { Write-Warning "Quit failed: $_" }
    try { [void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($xl) } catch {}
    $xl = $null
    [GC]::Collect(); [GC]::WaitForPendingFinalizers(); [GC]::Collect(); [GC]::WaitForPendingFinalizers()
    $proc = Get-Process -Id $xlPid -ErrorAction SilentlyContinue
    if ($proc) { $proc.WaitForExit(30000) | Out-Null }
    if (Get-Process -Id $xlPid -ErrorAction SilentlyContinue) { Stop-Process -Id $xlPid -Force }
    Write-Host "nan-probe: Excel PID $xlPid ended"
}
$result | ConvertTo-Json -Depth 6 | Set-Content -Encoding utf8 $Out
# The stored values: sheet1.xml of the saved package.
Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip = [IO.Compression.ZipFile]::OpenRead($xlsx)
try {
    $entry = $zip.GetEntry('xl/worksheets/sheet1.xml')
    $reader = New-Object IO.StreamReader($entry.Open())
    $reader.ReadToEnd() | Set-Content -Encoding utf8 (Join-Path (Split-Path $Out) 'nan-downstream-sheet1.xml')
    $reader.Close()
} finally { $zip.Dispose() }
Write-Host "nan-probe: wrote $Out"
