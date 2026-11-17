# TLAPS on Windows: isolated portability experiment

## Result

TLAPM 1.4.5 runs on this Windows machine without WSL, a global Cygwin
installation, or executing the TLAPS installer. Folder-local Cygwin binaries
and DLLs were added to the extracted legacy package. This is a tested legacy
command-line setup, not a current TLAPM language-server distribution.

| Check | Observed result |
|---|---|
| `tlapm.exe --version` | `1.4.5 (build 33809)`, exit 0 |
| Z3 version | `4.8.6 - 32 bit`, exit 0 |
| Zenon version | `0.8.4 [a268] 2017-11-14`, exit 0 |
| LS4 startup/help | Exit 0; subsequently checked two temporal obligations in Euclid |
| Valid conjunction proof using Zenon | Exit 0, backend dispatched, generated `PROOF-FOUND` proof |
| Valid integer-arithmetic proof using SMT/Z3 | Exit 0, backend dispatched |
| Deliberately false theorem using Zenon | Rejected with an unproved obligation, exit 3 |
| Deliberately false arithmetic theorem using SMT/Z3 | Rejected with an unproved obligation, exit 3 |
| Relocated minimal copy in `relocation check` | SMT proof succeeded through `tlapm.cmd`, exit 0 |
| Unchanged upstream `Euclid.tla` | 37 unique obligations checked, exit 0, 1.431 seconds |

## Run from PowerShell

```powershell
Set-Location C:\SANDBOX\APP\tlaplus\tlaps
.\tlapm.cmd --version
.\tlapm.cmd --nofp .\checks\ValidSMT.tla
```

This version of TLAPM can be silent on successful proof checking. Inspect
`$LASTEXITCODE`, which is 0 for success and 3 for the deliberately false test
cases. Generated proof files are written relative to the current working
directory, so run experiments from this folder or another disposable directory.

`tlapm.cmd` uses `setlocal`. It sets PATH, home, and temporary directories only
for its child processes. It does not change permanent environment variables.
The wrapper intentionally limits PATH to the bundle, its local runtime, and
standard Windows directories, rather than using tools from Git Bash or another
Cygwin installation.

To expose the wrapper in only the current PowerShell session:

```powershell
$env:PATH = "C:\SANDBOX\APP\tlaplus\tlaps;$env:PATH"
tlapm --version
```

## Repeat the checks

Task and Python are only needed for the experiment scripts, not the TLAPM
wrapper itself. Downloads and extracted files are already present.

```powershell
task                  # List commands
task run              # Bounded startup checks
task proof            # Two valid and two invalid proof checks
task euclid           # Check the unchanged upstream Euclid proof
task relocation       # Check the separate copy in a path containing spaces
task test             # Startup and proof checks
task runtime          # Verify cached runtime packages and extract selected files
task clean            # Remove generated scratch files only
```

`task extract` refuses to overwrite an existing `bundle` directory. It is an
extraction tool, not a command to rerun against this populated experiment.
The relocation check preserves its first copied bundle; it is not an automatic
synchronizer after source files change.

## Upstream Euclid example

The exact file requested at
https://github.com/tlaplus/tlapm/blob/main/examples/Euclid.tla
was downloaded unchanged to `checks/upstream/Euclid.tla`. Its SHA-256 is
`e91d47ecd63881b3f9009dd9740b44017314e48c2e720055d16c41babf374268`.
It is byte-for-byte identical to the example in the legacy package.

Run `task euclid` from this directory. It executes the Windows TLAPM with
fingerprint reuse disabled, one worker, and a 180-second process-tree timeout.
Generated proof files stay under `scratch/upstream-euclid/Euclid.tlaps`.

The observed run exited 0 in 1.431 seconds, processing 37 unique obligations:
24 trivial discharges, 11 Z3 launches, and two LS4 launches through `ptl_to_trp`.
No Zenon or Isabelle proof process was used. Full output is in
`logs/Euclid.tla.log`, and the command/result record is in
`logs/Euclid.result.json`. Missing optional provers listed by verbose
configuration output did not prevent this proof from checking successfully.

This checks the proof, not a numeric execution of the algorithm. As written,
the example assumes `GCDProperty1`, `GCDProperty2`, and `GCDProperty3` as axioms;
it does not prove those number-theoretic facts. Its final theorem is a safety
property conditional on the variables becoming equal, not a termination proof.

## What was downloaded and inspected

The official GitHub API listed this asset and the direct download returned
HTTP 200:

https://github.com/tlaplus/tlapm/releases/download/v1.4.5/tlaps-1.4.5-i686-cygwin-inst.exe

- Download size: 172,306,157 bytes.
- SHA-256: `d413a2cdce7f9aa1cc190ec0f5bc4734d0cffb752f1085fb449876eb837cebe8`.
- 7-Zip identified an embedded gzip stream at byte offset 689166.
- Its tar payload has 8102 entries. 7161 regular files were extracted.
- Fifteen Unix symbolic links were inventoried but not created.
- The archive contains no `tlapm_lsp` or `tlapm-lsp` executable.
- Its TLAPM executable is 32-bit PE and imports `cygwin1.dll` and
  `cyggcc_s-1.dll`, which the installer payload does not include.

The installer executable was never run. Extraction was performed with Python
from the verified payload, validating destination paths and excluding links.

Additional packages came from the archived x86 Cygwin mirror:

https://mirrors.kernel.org/sourceware/cygwin-archive/20221123/

Package sizes and SHA-512 hashes were checked against its downloaded
`x86/setup.xz` package index before extraction. That index was fetched over
HTTPS; its detached signature was not independently verified.

The 17 selected runtime files include the Cygwin 3.3.6 engine, GCC runtimes,
Bash and its DLLs, `ps`, `cygpath`, and four core utilities. Exact package
versions, download URLs, and digests are retained in
`logs/runtime-downloads.json`. No package installer, service, or registry editor
was run.

## What failed before the working setup

1. The extracted TLAPM, Zenon, and LS4 initially failed with Windows status
   `0xC0000135`, indicating missing DLL dependencies. Z3 already started.
2. Adding just the DLLs let TLAPM load, but it failed with `End_of_file`.
3. Upstream startup source showed a module-level `ps -l` probe before argument
   handling. Adding folder-local `/bin/sh` and `ps` resolved that startup failure.
4. Initial proof attempts exposed missing `mkdir`, `ls`, `tail`, and `rm`.
   These were added from the archived coreutils package, with their DLLs.

No Linux backend was used. Cygwin supplies a Windows DLL-based POSIX runtime;
it is not WSL or a virtual machine.

## Limits

- This is TLAPS 1.4.5, not the current server/toolchain.
- There is no `tlapm_lsp`, so this does not unblock the proposed modern Zed
  language-server integration.
- Isabelle certification with `-C` was not tested or enabled. The fifteen skipped
  symbolic links include Isabelle launchers. Treat the working result as Zenon
  and SMT backend proof checking, not validation of the full Isabelle stack.
- LS4 has checked Euclid's two temporal proof obligations, but broader LS4
  coverage was not tested. Other optional provers are absent.
- Z3 imports the x86 Microsoft Visual C++ runtime. Those dependencies were
  available on this machine; the experiment did not install them or prove that
  this folder runs on a clean Windows machine.
- Only a small set of proofs and a relocation path containing spaces were tested.
  No general Windows compatibility or security guarantee is implied.
- No Zed extension files or unrelated project files were modified by this test.

## Evidence

Older logs retain previous folder locations as historical evidence. New runs
use this dedicated `C:\SANDBOX\APP\tlaplus\tlaps` folder.

- `downloads/releases.json`: upstream release metadata.
- `logs/archive-list.txt` and `logs/tar-inventory.json`: artifact inventory.
- `logs/extraction.json`: payload digest, extraction count, and skipped links.
- `logs/pe-imports.txt`: PE architecture and imported DLLs.
- `logs/probe-bare.json`: missing-runtime startup failures.
- `logs/probe-with-dlls-only.json`: intermediate startup result.
- `logs/probe-with-runtime.json`: working version/startup probes.
- `logs/proof-smoke.json` and `logs/*.tla.log`: proof results and full output.
- `logs/initial-proof-probes/`: initial proof attempt evidence.
- `logs/relocation.json` and `logs/relocation.log`: wrapper relocation check.

Relevant startup/relocation source:

- https://github.com/tlaplus/tlapm/blob/v1.4.5/src/system.ml
- https://github.com/tlaplus/tlapm/blob/v1.4.5/src/params.ml
