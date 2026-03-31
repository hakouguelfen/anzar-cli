.PHONY: publish

install:
	cargo install --path .
publish:
	cargo release $(bump) --execute
	git push gitlab && git push gitlab --tags
