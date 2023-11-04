FROM python:3.11.6-slim-bullseye AS python-base

ENV PYTHONUNBUFFERED=1 \
	PYTHONDONTWRITEBYTECODE=1 \
	PIP_NO_CACHE_DIR=off \
	PIP_DISABLE_PIP_VERSION_CHECK=on \
	PIP_DEFAULT_TIMEOUT=100 \
	POETRY_HOME="/opt/poetry" \
	POETRY_VIRTUALENVS_IN_PROJECT=true \
	POETRY_NO_INTERACTION=1 \
	PYSETUP_PATH="/opt/pysetup" \
	VENV_PATH="/opt/pysetup/.venv"

ENV PATH="$POETRY_HOME/bin:$VENV_PATH/bin:$PATH"

FROM python-base AS builder-base
RUN buildDeps="build-essential" &&
	apt-get update &&
	apt-get install --no-install-recommends -y \
		curl \
		vim \
		netcat &&
	apt-get install -y --no-install-recommends $buildDeps &&
	rm -rf /var/lib/apt/lists/*

ENV POETRY_VERSION=1.6.1
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN curl -sSL https://install.python-poetry.org | POETRY_HOME=${POETRY_HOME} python3 - --version ${POETRY_VERSION} &&
	chmod a+x /opt/poetry/bin/poetry

WORKDIR $PYSETUP_PATH
COPY ./poetry.lock ./pyproject.toml ./
RUN poetry install --only main

FROM python-base as development
ENV FASTAPI_ENV=development

COPY --from=builder-base $POETRY_HOME $POETRY_HOME
COPY --from=builder-base $PYSETUP_PATH $PYSETUP_PATH

RUN ls
COPY scripts/activate-environment.sh /activate-environment.sh
RUN chmod +x /activate-environment.sh

WORKDIR $PYSETUP_PATH
RUN poetry install

WORKDIR /app
COPY . .

EXPOSE 8000
ENTRYPOINT /activate-environment.sh $0 $@
CMD ["uvicorn", "--reload", "--host=0.0.0.0", "--port=8000", "main:app"]

FROM python-base AS production
ENV FASTAPI_ENV=production

COPY --from=builder-base $VENV_PATH $VENV_PATH
COPY backend/gunicorn_config.py /gunicorn_config.py

COPY scripts/activate-environment.sh /activate-environment.sh
RUN chmod +x /activate-environment.sh

RUN groupadd -g 1500 poetry &&
	useradd -m -u 1500 -g poetry poetry

COPY --chown=poetry:poetry ./backend/src /app
USER poetry
WORKDIR /app

ENTRYPOINT /activate-environment.sh $0 $@
CMD [ "gunicorn", "--worker-class uvicorn.workers.UvicornWorker", "--config /gunicorn_config.py", "main:app"]
