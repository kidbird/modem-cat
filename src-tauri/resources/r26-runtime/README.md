Place the x86 `vcruntime140.dll` for the `r26-cli` firmware sidecar in this directory before building installers.

`build.ps1` and `scripts/build-helper.ps1` will stage it automatically from the local Windows / VC++ runtime when available.
