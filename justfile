set shell := ["bash", "-uc"]

default:
  @just --choose --justfile {{justfile()}}

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo clean
  rm *.lock

sort-d:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo sort-derives
  
web:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve -p app