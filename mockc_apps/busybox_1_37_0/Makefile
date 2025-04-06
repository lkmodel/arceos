APP_CHOICE?=

all: default_build

default_build: dynamic

static:
	$(error "Error: Unsupport type static")

dynamic:
	cp $(APP_CHOICE) apps.bin

.PHONY: all default_build static dynamic
