ARG TAG=latest
ARG IMAGEPATH=tensorchord/pgvecto-rs-binary:$TAG
FROM $IMAGEPATH as binary
FROM postgres:15

COPY --from=binary /pgvecto-rs-binary-release.deb /tmp/vectors.deb
RUN apt-get install -y /tmp/vectors.deb && \
    rm -f /tmp/vectors.deb && \
    echo "echo 'shared_preload_libraries = '\"'\"'vectors.so'\"'\"'' >> /var/lib/postgresql/data/postgresql.auto.conf" > /docker-entrypoint-initdb.d/vectors.sh
