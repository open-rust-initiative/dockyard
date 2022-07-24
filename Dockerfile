FROM ubuntu:22.04
RUN apt-get update  \
    && apt-get install -y libmysqlclient-dev
ENV MYPATH /opt/dockyard
WORKDIR $MYPATH
VOLUME $MYPATH
COPY target/release/dockyard $MYPATH
COPY .env $MYPATH
COPY static/ $MYPATH/static
COPY cert.pem $MYPATH
COPY key.pem $MYPATH
#CMD $MYPATH/dockyard
#CMD /bin/bash
CMD /opt/dockyard/dockyard