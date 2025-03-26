#ifndef PROBLEMMANAGER_H
#define PROBLEMMANAGER_H
#include <string>
#include <iostream>

enum class ProblemAction {
	GET, 
	TEST, 
	ERR
};



class ProblemManager{
private:
	ProblemAction action;
	std::string problemId;

	void handleGet();
	void handleTest();

public:
	ProblemManager(ProblemAction action, const std::string& problemId);

	void run();


};





#endif
