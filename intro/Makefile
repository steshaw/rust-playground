CC := clang
CXX := clang++
CFLAGS := -Wall

clang_format_style = chromium

vpath %.c c-src
vpath %.cpp cpp-src

all: format intro-c intro-cpp

.PHONY: format
format:
	clang-format --style=${clang_format_style} -i c-src/*.c cpp-src/*.cpp
