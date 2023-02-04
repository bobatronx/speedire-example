FROM python:3.11 as build

RUN curl -sSL https://install.python-poetry.org | python3 -

ENV PATH="$PATH:/root/.local/bin"

RUN echo $PATH

ADD dist/speedireexample-0.1.0.tar.gz /opt
WORKDIR /opt/speedireexample-0.1.0
RUN poetry update
RUN poetry install
WORKDIR /opt/speedireexample-0.1.0/speedireexample
CMD [ "poetry", "run", "--", "uvicorn", "main:app" ]