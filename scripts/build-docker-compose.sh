#!/bin/bash

docker compose build && docker image prune -f && docker compose up --remove-orphans
