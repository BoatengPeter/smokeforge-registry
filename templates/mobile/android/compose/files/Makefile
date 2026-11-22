.PHONY: help build test lint check ci clean

help:
	@echo "make build   Build the debug APK"
	@echo "make test    Run JVM unit tests"
	@echo "make lint    Run Android lint"
	@echo "make check   Run tests and lint"
	@echo "make ci      Run tests, lint, and assemble"
	@echo "make clean   Remove generated output"

build:
	./gradlew assembleDebug

test:
	./gradlew test

lint:
	./gradlew lint

check: test lint

ci: test lint assembleDebug

clean:
	./gradlew clean
