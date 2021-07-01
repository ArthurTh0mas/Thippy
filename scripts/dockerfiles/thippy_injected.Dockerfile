# Dockerfile to build thippy docker image. Uses binary that was build externally.
FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG VERSION

LABEL io.parity.image.title="thippy" \
    io.parity.image.description="Thippy - A Smart Contracts Parachain" \
    io.parity.image.source="https://github.com/ng8eke/thippy/blob/${VCS_REF}/scripts/dockerfiles/thippy_injected.Dockerfile" \
    io.parity.image.url="https://github.com/ng8eke/thippy/blob/${VCS_REF}/scripts/dockerfiles/thippy_injected.Dockerfile" \
    io.parity.image.documentation="https://github.com/ng8eke/thippy/blob/${VCS_REF}/README.md" \
    io.parity.image.created="${BUILD_DATE}" \
    io.parity.image.version="${VERSION}" \
    io.parity.image.revision="${VCS_REF}" \
    io.parity.image.authors="devops-team@parity.io" \
    io.parity.image.vendor="Annie Lai" \
    io.parity.image.licenses="GPL-3.0 License"

# show backtraces
ENV RUST_BACKTRACE 1
ENV DEBIAN_FRONTEND=noninteractive

# add non-root user
RUN groupadd -g 1000 user && \
	useradd -u 1000 -g user -s /bin/sh -m user

# switch to non-root user
USER user

COPY --chown=root:root ./thippy /usr/local/bin/

# check if executable works in this container
RUN /usr/local/bin/thippy --version

EXPOSE 30333 9933 9944
ENTRYPOINT ["/usr/local/bin/thippy"]
