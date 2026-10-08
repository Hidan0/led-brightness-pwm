#!/usr/bin/env bash
# Local build prerequisites for ESP-IDF v4.4.4 on a modern host, installed
# under .embuild/ (git-ignored). Requires `uv`. Idempotent.
#
# - Python 3.11: ESP-IDF 4.4's installer does not support newer Pythons
# - libclang 15: bindgen 0.63 generates empty structs with libclang >= 16
#
# Build with:  PATH="$PWD/.embuild/bin:$PATH" cargo build
set -euo pipefail
cd "$(dirname "$0")"

uv python install 3.11
py=$(uv python find 3.11)
uv pip install --python "$py" --system --break-system-packages virtualenv

mkdir -p .embuild/bin
ln -sf "$py" .embuild/bin/python3
ln -sf "$py" .embuild/bin/python

uv venv --allow-existing -p 3.11 .embuild/libclang-env
VIRTUAL_ENV=.embuild/libclang-env uv pip install "libclang==15.0.6.1"
ln -sfn libclang-env/lib/python3.11/site-packages/clang/native .embuild/libclang

# ESP-IDF 4.4 still imports pkg_resources, removed in setuptools 81. Its
# Python env only exists after the first build: rerun this script if that
# build fails on "Some Python dependencies must be installed".
idf_py=~/.espressif/python_env/idf4.4_py3.11_env/bin/python
if [ -x "$idf_py" ]; then
    "$idf_py" -m pip install -q "setuptools<70"
fi
