# [Choice] Debian OS version (use bookworm, or bullseye on local arm64/Apple Silicon): bookworm, buster, bullseye
ARG VARIANT="bookworm"
FROM mcr.microsoft.com/devcontainers/rust:1-1-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends cmake gnupg lsb-release software-properties-common wget

RUN cd /tmp \
    && wget https://apt.llvm.org/llvm.sh \
    && chmod a+x llvm.sh \
    && ./llvm.sh 15
