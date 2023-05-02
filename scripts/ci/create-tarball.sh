#!/usr/bin/env bash
set -e

usage() {
  echo "Usage: $0 [--target <target triple>]"
  exit 1
}

TARGET=$(cargo -vV | awk '/host:/ {print $2}')
while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      TARGET=$2
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done

RELEASE_BASENAME="${RELEASE_BASENAME:=clockwork-geyser-plugin-release}"
TARBALL_BASENAME="${TARBALL_BASENAME:="$RELEASE_BASENAME"}"

echo --- Creating release tarball
(
  var=$(pwd)
  echo "The current working directory $var"

  set -x
  rm -rf "${RELEASE_BASENAME:?}"/
  mkdir "${RELEASE_BASENAME}"/

  COMMIT="$(git rev-parse HEAD)"
  (
    echo "channel: $CI_TAG"
    echo "commit: $COMMIT"
    echo "target: $TARGET"
  ) > "${RELEASE_BASENAME}"/version.yml

  var=$(pwd)
  echo "The current working directory $var"

  source ./scripts/ci/rust-version.sh stable
  ./scripts/build-all.sh +"${rust_stable:?}" --release --target "$TARGET" "${RELEASE_BASENAME}"

  RELEASE_NAME="${TARBALL_BASENAME}-${TARGET}"

  tar cvf "$RELEASE_NAME".tar "${RELEASE_BASENAME}"
  bzip2 -f "$RELEASE_NAME".tar
  cp -fv "${RELEASE_BASENAME}"/version.yml "$RELEASE_NAME".yml
)

echo --- ok
