@pgrx *args:
    cargo pgrx {{args}} pg15

@build:
    cargo pgrx package

@docs:
    cargo watch -s 'cargo doc && http target/doc'
