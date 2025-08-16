build:
	cargo build --release
	docker build -t actixnew .
	docker image prune -f
run:
	docker run -dp 8080:8080 --restart=always actixnew:latest
