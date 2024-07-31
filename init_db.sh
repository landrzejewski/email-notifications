#!/usr/bin/env bash
# cargo install sqlx-cli --no-default-features --features rustls,postgres

DATABASE_URL=postgres://admin:admin@localhost:5432/notifications
export DATABASE_URL

# sqlx database create
# sqlx migrate add create_subscriptions_table
sqlx migrate run