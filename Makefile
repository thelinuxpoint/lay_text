
config:
	@cargo build

push:
	@cargo build
	@git add .
	@git commit -m "Lay Text Editor on Update"

