#ifndef PROBLEM_H
#define PROBLEM_H

#include <vector>
#include <iostream>
#include <functional>
#include <memory>
#include <unordered_map>
#include <any>


template <typename T>
struct TestSuite {
	std::vector<std::function<T()>> testCases;
	std::unordered_map<int, std::string> testName;
	std::vector<T> solutions;
};

template <typename T>
class Problem {
public:
	Problem(const std::string& statement) : problemStatement(statement) {};
	
	int runTests(const std::string& executablePath) {
		return -1; //placeholder
	}
		/*
		Returns the index of the test that failed; otherwise returns -1
		*/




private:

	std::string problemStatement;
	
	TestSuite<T> testSuite;
	
	





};




#endif
