# parker
helps park things

## deploy

    flyctl deploy --remote-only

### staging

Create if needed:

    flyctl apps create parker-staging-houseofmoran --builder dockerfile --no-config

Deploy to staging:

    flyctl deploy --remote-only --config fly.staging.toml
    flyctl open --config fly.staging.toml

## certs

    flyctl certs create speculaas.houseofmoran.io
    flyctl certs create speculaas.houseofmoran.com