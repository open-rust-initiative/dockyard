FROM mysql:8
ENV AUTO_RUN_DIR /docker-entrypoint-initdb.d
ENV INIT_SQL up.sql
COPY up.sql $AUTO_RUN_DIR
RUN chmod a+x $AUTO_RUN_DIR/$INIT_SQL