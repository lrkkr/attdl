set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias p := push
alias c := check
alias b := build

default:
  just --list

# build
build:
  cargo build --release -j 14

# git push with commit
push message:
  git add .
  git commit -m "{{ message }}"
  git push

# pre-commit check all files
check:
  pre-commit run --all-files
