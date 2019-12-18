CC := clang

#VPATH := src
vpath %.c src

all: format intro

#intro-c: src/main.c

.PHONY: format
format:
	clang-format -i src/intro.c
