CXXFLAGS = -std=c++14 -Wall

default: run_all
all: all_tests
run_all: run_all_tests
test: run_all_tests
all_tests: build/state_tests build/ast_tests build/eval_tests
run_all_tests: all_tests
	build/state_tests && \
	build/ast_tests && \
	build/eval_tests

build/state_tests: tests/state_tests.cpp build
	$(CXX) $(CXXFLAGS) -o $@ $<
build/state_tests.s: tests/state_tests.cpp build
	$(CXX) $(CXXFLAGS) -o $@ $< -S

build/ast_tests: tests/ast_tests.cpp build
	$(CXX) $(CXXFLAGS) -o $@ $<
build/ast_tests.s: tests/ast_tests.cpp build
	$(CXX) $(CXXFLAGS) -o $@ $< -S

build/eval_tests: tests/eval_tests.cpp build
	$(CXX) $(CXXFLAGS) -o $@ $<
build/eval_tests.s: tests/eval_tests.cpp build
	$(CXX) $(CXXFLAGS) -o $@ $< -S

interactive:
	when-changed Makefile src/*.cpp -c "clear; make run_all"

build:
	mkdir build

clean:
	rm -rf build
