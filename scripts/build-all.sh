#!/usr/bin/env bash

set -e

usage() {
  exitcode=0
  if [[ -n "$1" ]]; then
    exitcode=1
    echo "Error: $*"
  fi
  cat <<EOF
usage: $0 [+<cargo version>] [--debug] <install directory>
EOF
  exit $exitcode
}

# Set build flags
maybeRustVersion=
installDir=
buildVariant=release
maybeReleaseFlag=--release
while [[ -n $1 ]]; do
  if [[ ${1:0:1} = - ]]; then
    if [[ $1 = --debug ]]; then
      maybeReleaseFlag=
      buildVariant=debug
      shift
    else
      usage "Unknown option: $1"
    fi
  elif [[ ${1:0:1} = \+ ]]; then
    maybeRustVersion=$1
    shift
  else
    installDir=$1
    shift
  fi
done

# Get the output filetype
if [[ $OSTYPE == darwin* ]]; then
  libExt=dylib 
elif [[ $OSTYPE == linux* ]]; then
  libExt=so
else 
  echo OS unsupported
  exit 1
fi

# Check the install directory is provided
if [[ -z "$installDir" ]]; then
  usage "Install directory not specified"
  exit 1
fi

# Create the install directory
installDir="$(mkdir -p "$installDir"; cd "$installDir"; pwd)"
mkdir -p "$installDir/lib"
mkdir -p "$installDir/bin"
echo "Install location: $installDir ($buildVariant)"
cd "$(dirname "$0")"/..
SECONDS=0

# Enumerate the bins
BINS=(
  clockwork
)

# Create bin args
binArgs=()
for bin in "${BINS[@]}"; do 
  binArgs+=(--bin "$bin")
done

# Build the repo
(
  set -x
  cargo $maybeRustVersion build $maybeReleaseFlag "${binArgs[@]}" --lib
  anchor build
)

# Create new Geyser plugin config 
touch "$installDir"/lib/geyser-plugin-config.json
echo "{
  \"libpath\": \"$installDir/lib/libclockwork_plugin.$libExt\",
  \"keypath\": \"$installDir/lib/clockwork-worker-keypair.json\",
  \"slot_timeout_threshold\": 150,
  \"worker_threads\": 10
}" > "$installDir"/lib/geyser-plugin-config.json

# Create a worker keypair
echo
solana-keygen new -f -s --no-bip39-passphrase -o $installDir/lib/clockwork-worker-keypair.json
echo

# Copy bins
for bin in "${BINS}"; do
  cp -fv "target/$buildVariant/$bin" "$installDir"/bin
done

# Copy program binaries into lib folder
cp -fv "target/deploy/clockwork_health.so" "$installDir"/lib
cp -fv "target/deploy/clockwork_http.so" "$installDir"/lib
cp -fv "target/deploy/clockwork_network.so" "$installDir"/lib
cp -fv "target/deploy/clockwork_pool.so" "$installDir"/lib
cp -fv "target/deploy/clockwork_scheduler.so" "$installDir"/lib

# Copy plugin
cp -fv "target/$buildVariant/libclockwork_plugin.$libExt" "$installDir"/lib

# Success message
echo "Done after $SECONDS seconds"
echo 
echo "To use these binaries:"
echo "  export PATH=\"$installDir\"/bin:\"\$PATH\""
