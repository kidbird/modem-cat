# Modem Band Specifications

Supported models and their hardware band specifications. This is the reference source for
`spec_bands_for_model()` in `modem-hal/src/types.rs`.

## High-level Summary

| Model | Platform | LTE Bands | NR Bands |
|---|---|---|---|
| RM520N-GL / RM500Q-GL | Qualcomm | B1/2/3/4/5/7/8/12/13/14/17/18/19/20/25/26/28/29/30/32/34/38/39/40/41/42/43/48/66/71 | n1/2/3/5/7/8/12/13/14/18/20/25/26/28/29/30/38/40/41/48/66/70/71/75/76/77/78/79 |
| RM520N-CN / RM500Q-CN | Qualcomm | B1/3/5/8/34/38/39/40/41 | n1/3/5/8/28/41/78/79 |
| RM520N-EU | Qualcomm | B1/3/5/7/8/20/28/32/38/40/41/42/43/71 | n1/3/5/7/8/20/28/38/40/41/71/75/76/77/78 |
| RM500U-CNV / RG200U-CN | UniSoc (展锐) | B1/3/5/8/34/38/39/40/41 | n1/3/5/8/28/41/77/78/79 |
| RM500U-EA | UniSoc (展锐) | B1/2/3/4/5/7/8/20/28/38/40/41/66 | n1/3/5/7/8/20/28/38/40/41/66/77/78 |

## Qualcomm Models

### RM520N-GL / RM500Q-GL

- **LTE**: B1/ 2/ 3/ 4/ 5/ 7/ 8/ 12/ 13/ 14/ 17/ 18/ 19/ 20/ 25/ 26/ 28/ 29/ 30/ 32/ 66/ 71/ B34/ 38/ 39/ 40/ 41/ 42/ 43/ 48
- **NR**: n1/ 2/ 3/ 5/ 7/ 8/ 12/ 13/ 14/ 18/ 20/ 25/ 26/ 28/ 29/ 30/ 38/ 40/ 41/ 48/ 66/ 70/ 71/ 75/ 76/ 77/ 78/ 79

> RM500Q-GL shares the same spec as RM520N-GL.

### RM520N-CN / RM500Q-CN

- **LTE**: B1/ 3/ 5/ 8/ B34/ 38/ 39/ 40/ 41
- **NR**: n1/ 3/ 5/ 8/ 28/ 41/ 78/ 79

> RM500Q-CN shares the same spec as RM520N-CN.

### RM520N-EU

- **LTE**: B1/ 3/ 5/ 7/ 8/ 20/ 28/ 32/ 71/ B38/ 40/ 41/ 42/ 43
- **NR**: n1/ 3/ 5/ 7/ 8/ 20/ 28/ 38/ 40/ 41/ 71/ 75/ 76/ 77/ 78

## UniSoc (展锐) Models

### RM500U-CNV / RG200U-CN

- **LTE**: B1/ 3/ 5/ 8/ B34/ 38/ 39/ 40/ 41
- **NR**: n1/ 3/ 5/ 8/ 28/ 41/ 77/ 78/ 79

> RG200U-CN shares the same spec as RM500U-CNV.

### RM500U-EA

- **LTE**: B1/ 2/ 3/ 4/ 5/ 7/ 8/ 20/ 28/ 66/ B38/ 40/ 41
- **NR**: n1/ 3/ 5/ 7/ 8/ 20/ 28/ 38/ 40/ 41/ 66/ 77/ 78

## Notes

- Band numbers without prefix letter follow the previous prefix (e.g. `B34/ 38/ 39` means B34, B38, B39).
- `spec_bands_for_model()` in `modem-hal/src/types.rs` is the authoritative implementation. Keep this doc in sync when adding new models.
- Models not listed here return empty spec bands — the app will still function but won't show hardware spec info.
