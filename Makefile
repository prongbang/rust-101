# make create_project name=my-project
create_project:
	cargo new $(name)

run:
	cargo run src/main.rs