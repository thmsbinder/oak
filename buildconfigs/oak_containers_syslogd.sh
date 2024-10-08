#!/bin/bash
#
# Build configuration for oak_containers_syslogd.
#
export PACKAGE_NAME=oak_containers_syslogd

export BUILD_COMMAND=(
  nix
  develop
  .#systemImageProvenance
  --command
  just
  oak_containers_syslogd
)

export SUBJECT_PATHS=(
  oak_containers/syslogd/target/oak_containers_syslogd_patched
)
