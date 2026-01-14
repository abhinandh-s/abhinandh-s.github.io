dev:
  nix develop

serve:
  trunk serve --poll --poll-interval 10s

ship:
   git add -A && git commit -m "migration" && git push
