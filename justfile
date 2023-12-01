run day part:
    cargo watch -x "check -p {{day}}"
create day:
    cargo generate --path ./daily-template --name {{day}}
