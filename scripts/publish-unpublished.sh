#!/usr/bin/env bash

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
readonly REGISTRY="crates-io"
readonly CRATES_IO_API="https://crates.io/api/v1/crates"
readonly USER_AGENT="ohos-native-bindings-publish-script/1.0"

REMOTE="origin"
DRY_RUN=false
REQUESTED_PACKAGES=()
REQUESTED_PACKAGE_COUNT=0
PUBLISH_TIMEOUT_SECONDS="${PUBLISH_TIMEOUT_SECONDS:-1800}"
POLL_INTERVAL_SECONDS="${POLL_INTERVAL_SECONDS:-10}"

usage() {
  cat <<'EOF'
Publish workspace crates that are not yet available on crates.io.

The script:
  - only considers packages under crates/ and sys/;
  - skips package versions that already exist on crates.io;
  - publishes missing packages in workspace dependency order;
  - creates and pushes <package>-v<version> annotated tags;
  - never creates a GitHub Release and does not require a GitHub token.

Usage:
  scripts/publish-unpublished.sh [options]

Options:
  -p, --package NAME  Only process this package. May be specified more than once.
      --remote NAME   Git remote used to read and push tags (default: origin).
      --dry-run       Show what would be published and tagged without changing state.
  -h, --help          Show this help.

Environment:
  PUBLISH_TIMEOUT_SECONDS  Maximum wait for a published version to appear (default: 1800).
  POLL_INTERVAL_SECONDS    Delay between registry checks (default: 10).

Examples:
  scripts/publish-unpublished.sh --dry-run
  scripts/publish-unpublished.sh \
    -p ohos-native-huks-sys \
    -p ohos-huks-binding
EOF
}

log() {
  printf '[publish] %s\n' "$*"
}

warn() {
  printf '[publish] warning: %s\n' "$*" >&2
}

die() {
  printf '[publish] error: %s\n' "$*" >&2
  exit 1
}

require_command() {
  command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"
}

is_positive_integer() {
  case "$1" in
    ''|*[!0-9]*|0) return 1 ;;
    *) return 0 ;;
  esac
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -p|--package)
      [[ $# -ge 2 ]] || die "$1 requires a package name"
      REQUESTED_PACKAGES[${REQUESTED_PACKAGE_COUNT}]="$2"
      REQUESTED_PACKAGE_COUNT=$((REQUESTED_PACKAGE_COUNT + 1))
      shift 2
      ;;
    --remote)
      [[ $# -ge 2 ]] || die "$1 requires a git remote name"
      REMOTE="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      die "unknown option: $1"
      ;;
  esac
done

require_command cargo
require_command curl
require_command git
require_command jq
require_command awk
require_command mktemp

is_positive_integer "${PUBLISH_TIMEOUT_SECONDS}" ||
  die "PUBLISH_TIMEOUT_SECONDS must be a positive integer"
is_positive_integer "${POLL_INTERVAL_SECONDS}" ||
  die "POLL_INTERVAL_SECONDS must be a positive integer"

cd "${REPO_ROOT}"

git rev-parse --is-inside-work-tree >/dev/null 2>&1 ||
  die "repository root is not a git worktree: ${REPO_ROOT}"
git remote get-url "${REMOTE}" >/dev/null 2>&1 ||
  die "git remote does not exist: ${REMOTE}"

if [[ "${DRY_RUN}" == false ]] &&
  [[ -n "$(git status --porcelain=v1 --untracked-files=all -- Cargo.toml Cargo.lock crates sys)" ]]; then
  die "Cargo.toml, Cargo.lock, crates/, and sys/ must be clean before publishing and tagging"
fi

readonly HEAD_COMMIT="$(git rev-parse HEAD)"
readonly TEMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/ohos-publish.XXXXXX")"
readonly METADATA_FILE="${TEMP_DIR}/metadata.json"
readonly ALL_PACKAGES_FILE="${TEMP_DIR}/all-packages.tsv"
readonly SELECTED_PACKAGES_FILE="${TEMP_DIR}/selected-packages.tsv"
readonly REMAINING_PACKAGES_FILE="${TEMP_DIR}/remaining-packages.tsv"
readonly NEXT_PACKAGES_FILE="${TEMP_DIR}/next-packages.tsv"
readonly PROCESSED_PACKAGES_FILE="${TEMP_DIR}/processed-packages.txt"
readonly REMOTE_TAGS_FILE="${TEMP_DIR}/remote-tags.txt"

cleanup() {
  rm -rf "${TEMP_DIR}"
}
trap cleanup EXIT

cargo metadata --format-version 1 --no-deps >"${METADATA_FILE}"

jq -r --arg root "${REPO_ROOT}" '
  .packages
  | map(
      select(
        (.manifest_path | startswith($root + "/crates/"))
        or (.manifest_path | startswith($root + "/sys/"))
      )
    )
  | map(select(.publish != []))
  | map({
      name,
      version,
      manifest_path,
      dependencies: (
        [.dependencies[] | select(.path != null) | .name]
        | unique
        | sort
        | join(",")
      )
    })
  | sort_by(.name)
  | .[]
  | [.name, .version, .manifest_path, .dependencies]
  | @tsv
' "${METADATA_FILE}" >"${ALL_PACKAGES_FILE}"

: >"${SELECTED_PACKAGES_FILE}"

package_was_requested() {
  local package_name="$1"
  local requested
  local requested_index=0

  if [[ ${REQUESTED_PACKAGE_COUNT} -eq 0 ]]; then
    return 0
  fi

  while [[ ${requested_index} -lt ${REQUESTED_PACKAGE_COUNT} ]]; do
    requested="${REQUESTED_PACKAGES[${requested_index}]}"
    if [[ "${requested}" == "${package_name}" ]]; then
      return 0
    fi
    requested_index=$((requested_index + 1))
  done

  return 1
}

while IFS=$'\t' read -r package_name package_version manifest_path dependencies; do
  if package_was_requested "${package_name}"; then
    printf '%s\t%s\t%s\t%s\n' \
      "${package_name}" \
      "${package_version}" \
      "${manifest_path}" \
      "${dependencies}" >>"${SELECTED_PACKAGES_FILE}"
  fi
done <"${ALL_PACKAGES_FILE}"

requested_index=0
while [[ ${requested_index} -lt ${REQUESTED_PACKAGE_COUNT} ]]; do
  requested_package="${REQUESTED_PACKAGES[${requested_index}]}"
  if ! awk -F '\t' -v package="${requested_package}" '
    $1 == package { found = 1 }
    END { exit(found ? 0 : 1) }
  ' "${SELECTED_PACKAGES_FILE}"; then
    die "package is not a publishable crates/* or sys/* member: ${requested_package}"
  fi
  requested_index=$((requested_index + 1))
done

if [[ ! -s "${SELECTED_PACKAGES_FILE}" ]]; then
  log "no publishable packages selected"
  exit 0
fi

log "reading tags from git remote ${REMOTE}"
git ls-remote --tags "${REMOTE}" >"${REMOTE_TAGS_FILE}"

registry_version_status() {
  local package_name="$1"
  local package_version="$2"
  local response_file="${TEMP_DIR}/registry-response.json"
  local http_code
  local curl_exit
  local attempt=1

  while [[ ${attempt} -le 5 ]]; do
    set +e
    http_code="$(
      curl \
        --silent \
        --show-error \
        --location \
        --output "${response_file}" \
        --write-out '%{http_code}' \
        --header "User-Agent: ${USER_AGENT}" \
        "${CRATES_IO_API}/${package_name}/${package_version}"
    )"
    curl_exit=$?
    set -e

    if [[ ${curl_exit} -ne 0 ]]; then
      if [[ ${attempt} -eq 5 ]]; then
        die "failed to query crates.io for ${package_name}@${package_version}"
      fi
    else
      case "${http_code}" in
        200)
          printf 'published\n'
          return
          ;;
        404)
          printf 'missing\n'
          return
          ;;
        429|500|502|503|504)
          if [[ ${attempt} -eq 5 ]]; then
            die "crates.io returned HTTP ${http_code} for ${package_name}@${package_version}"
          fi
          ;;
        *)
          die "crates.io returned HTTP ${http_code} for ${package_name}@${package_version}"
          ;;
      esac
    fi

    sleep "$((attempt * 2))"
    attempt=$((attempt + 1))
  done
}

remote_tag_commit() {
  local tag_name="$1"

  awk \
    -v direct_ref="refs/tags/${tag_name}" \
    -v peeled_ref="refs/tags/${tag_name}^{}" '
      $2 == direct_ref { direct = $1 }
      $2 == peeled_ref { peeled = $1 }
      END {
        if (peeled != "") {
          print peeled
        } else if (direct != "") {
          print direct
        }
      }
    ' "${REMOTE_TAGS_FILE}"
}

record_remote_tag() {
  local tag_name="$1"
  local tag_object

  tag_object="$(git rev-parse "refs/tags/${tag_name}")"
  printf '%s\trefs/tags/%s\n' "${tag_object}" "${tag_name}" >>"${REMOTE_TAGS_FILE}"
  printf '%s\trefs/tags/%s^{}\n' "${HEAD_COMMIT}" "${tag_name}" >>"${REMOTE_TAGS_FILE}"
}

prepare_local_tag() {
  local package_name="$1"
  local package_version="$2"
  local tag_name="$3"
  local local_commit
  local remote_commit

  remote_commit="$(remote_tag_commit "${tag_name}")"
  if [[ -n "${remote_commit}" ]] && [[ "${remote_commit}" != "${HEAD_COMMIT}" ]]; then
    die "remote tag ${tag_name} points to ${remote_commit}, expected ${HEAD_COMMIT}"
  fi

  if git rev-parse -q --verify "refs/tags/${tag_name}" >/dev/null; then
    local_commit="$(git rev-list -n 1 "${tag_name}")"
    if [[ "${local_commit}" != "${HEAD_COMMIT}" ]]; then
      die "local tag ${tag_name} points to ${local_commit}, expected ${HEAD_COMMIT}"
    fi
    return
  fi

  if [[ -n "${remote_commit}" ]]; then
    return
  fi

  if [[ "${DRY_RUN}" == true ]]; then
    log "[dry-run] would create tag ${tag_name} at ${HEAD_COMMIT}"
    return
  fi

  git tag \
    --annotate "${tag_name}" \
    --message "chore: Release package ${package_name} version ${package_version}" \
    "${HEAD_COMMIT}"
  log "created local tag ${tag_name}"
}

push_tag() {
  local tag_name="$1"
  local remote_commit

  remote_commit="$(remote_tag_commit "${tag_name}")"
  if [[ -n "${remote_commit}" ]]; then
    log "tag ${tag_name} already exists on ${REMOTE}"
    return
  fi

  if [[ "${DRY_RUN}" == true ]]; then
    log "[dry-run] would push tag ${tag_name} to ${REMOTE}"
    return
  fi

  git push "${REMOTE}" "refs/tags/${tag_name}"
  record_remote_tag "${tag_name}"
  log "pushed tag ${tag_name} to ${REMOTE}"
}

sync_tag_for_published_package() {
  local tag_name="$1"
  local remote_commit

  remote_commit="$(remote_tag_commit "${tag_name}")"
  if [[ -n "${remote_commit}" ]]; then
    return
  fi

  if git rev-parse -q --verify "refs/tags/${tag_name}" >/dev/null; then
    if [[ "${DRY_RUN}" == true ]]; then
      log "[dry-run] would push existing local tag ${tag_name} to ${REMOTE}"
    else
      git push "${REMOTE}" "refs/tags/${tag_name}"
      record_remote_tag "${tag_name}"
      log "pushed existing local tag ${tag_name} to ${REMOTE}"
    fi
  else
    warn "${tag_name} is published but has no local or remote tag; leaving it unchanged"
  fi
}

wait_until_published() {
  local package_name="$1"
  local package_version="$2"
  local started_at
  local now

  started_at="$(date +%s)"
  while true; do
    if [[ "$(registry_version_status "${package_name}" "${package_version}")" == "published" ]]; then
      return
    fi

    now="$(date +%s)"
    if [[ $((now - started_at)) -ge ${PUBLISH_TIMEOUT_SECONDS} ]]; then
      die "timed out waiting for ${package_name}@${package_version} to appear on crates.io"
    fi

    log "waiting for ${package_name}@${package_version} to appear on crates.io"
    sleep "${POLL_INTERVAL_SECONDS}"
  done
}

process_package() {
  local package_name="$1"
  local package_version="$2"
  local tag_name="${package_name}-v${package_version}"
  local status

  status="$(registry_version_status "${package_name}" "${package_version}")"
  if [[ "${status}" == "published" ]]; then
    log "skip ${package_name}@${package_version}: already published"
    sync_tag_for_published_package "${tag_name}"
    return
  fi

  log "missing ${package_name}@${package_version}"
  prepare_local_tag "${package_name}" "${package_version}" "${tag_name}"

  if [[ "${DRY_RUN}" == true ]]; then
    log "[dry-run] would publish ${package_name}@${package_version}"
    push_tag "${tag_name}"
    return
  fi

  cargo publish \
    --registry "${REGISTRY}" \
    --package "${package_name}@${package_version}"

  wait_until_published "${package_name}" "${package_version}"
  log "published ${package_name}@${package_version}"
  push_tag "${tag_name}"
}

package_is_selected() {
  local package_name="$1"

  awk -F '\t' -v package="${package_name}" '
    $1 == package { found = 1 }
    END { exit(found ? 0 : 1) }
  ' "${SELECTED_PACKAGES_FILE}"
}

package_is_processed() {
  local package_name="$1"

  awk -v package="${package_name}" '
    $0 == package { found = 1 }
    END { exit(found ? 0 : 1) }
  ' "${PROCESSED_PACKAGES_FILE}"
}

package_is_ready() {
  local dependencies="$1"
  local dependency
  local old_ifs="${IFS}"

  IFS=','
  for dependency in ${dependencies}; do
    if package_is_selected "${dependency}" && ! package_is_processed "${dependency}"; then
      IFS="${old_ifs}"
      return 1
    fi
  done
  IFS="${old_ifs}"

  return 0
}

cp "${SELECTED_PACKAGES_FILE}" "${REMAINING_PACKAGES_FILE}"
: >"${PROCESSED_PACKAGES_FILE}"

while [[ -s "${REMAINING_PACKAGES_FILE}" ]]; do
  progress=false
  : >"${NEXT_PACKAGES_FILE}"

  while IFS=$'\t' read -r package_name package_version manifest_path dependencies; do
    if package_is_ready "${dependencies}"; then
      process_package "${package_name}" "${package_version}"
      printf '%s\n' "${package_name}" >>"${PROCESSED_PACKAGES_FILE}"
      progress=true
    else
      printf '%s\t%s\t%s\t%s\n' \
        "${package_name}" \
        "${package_version}" \
        "${manifest_path}" \
        "${dependencies}" >>"${NEXT_PACKAGES_FILE}"
    fi
  done <"${REMAINING_PACKAGES_FILE}"

  if [[ "${progress}" == false ]]; then
    die "workspace package dependency cycle detected among selected packages"
  fi

  cp "${NEXT_PACKAGES_FILE}" "${REMAINING_PACKAGES_FILE}"
done

if [[ "${DRY_RUN}" == true ]]; then
  log "dry-run completed"
else
  log "all selected packages are published"
fi
