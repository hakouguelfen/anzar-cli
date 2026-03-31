.PHONY: publish

install:
	cargo install --path .
publish:
	cargo release $(bump) --execute
	git push gitlab && git push gitlab --tags
publish_github:
	cargo release $(bump) --execute
	git push github && git push github --tags
