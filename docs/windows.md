# Windows: creating symlinks

`dot` links files with symlinks, and Windows guards that operation. This page is
the single source of truth for the Windows story; other documents link here.

## Allowed when either holds

| | How |
|---|---|
| Developer Mode | Settings -> System -> For developers -> Developer Mode (Windows 10 1703+) |
| Privilege | `SeCreateSymbolicLinkPrivilege`: Local Policies \ User Rights Assignment \ Create symbolic links |

Developer Mode state lives in this registry value (DWORD `1` = on):

```
HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock
    AllowDevelopmentWithoutDevLicense
```

Elevation is not the requirement. An Administrator token that does not hold the
privilege still fails with error 1314, so "run as administrator" is not the fix.
`dot` does not ask for elevation: it passes
`SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE` (through `std`), which is what
makes Developer Mode sufficient.

## Symptom

```
cannot create symbolic links in <dir>: A required privilege is not held by the
client. (os error 1314)
```

`dot link` probes the capability once, in the target directory, before touching
any file (`src/handlers/utils/symlink_support.rs`). The message appears once, not
once per file.

## Hard links and junctions are not substitutes

- **Hard link** — the same data, not a pointer. `dot restore` (symlink to regular
  copy) loses its meaning, and "edit in the repo, live at once" stops holding,
  because editing the linked file edits the repository. Same volume only.
- **Junction** — directories only. `dot` links regular files only
  (README § Technical remarks), so a junction buys nothing.

## CI

`windows-latest` is excluded from `ci.yml` because the test suite needs
Developer Mode, which those runners do not enable. The runners themselves are
elevated, so this is a Developer Mode gap, not a privilege gap. Symlink tests
also skip themselves where symlinks are unavailable.
