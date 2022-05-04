GENERAL_ARGS = --release
BACKEND_ARGS = $(GENERAL_ARGS) -p jokehub

.PHONY: \
		build-backend \
		run-backend \

run-backend:
		cd backend && \
			cargo run $(BACKEND_ARGS)

build-backend: 
		cd backend && \
				cargo build $(BACKEND_ARGS)


.DEFAULT_GOAL := run-backend