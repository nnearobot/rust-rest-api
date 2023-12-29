start:
	docker-compose up -d --build

stop:
	docker-compose stop

run:
	cargo run

doc:
	cardo doc --open

test:
	cargo test -- --test-threads=1

help:
	@echo "Restaurant API\n\
\n\
            make run   - run the application on the developer environment\n\
            make test  - run the tests\n\
            make doc   - open this crate's documentation\n\
\n\
            make start - build and start Docker containers for production\n\
            make stop  - stop Docker containers\n\
"