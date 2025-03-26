#include "ProblemManager.h"

ProblemManager::ProblemManager(ProblemAction action, const std::string& problemId) :
	action(action), problemId(problemId) {
}

void ProblemManager::handleGet() {
	std::cout << "Getting" << std::endl;
}

void ProblemManager::handleTest() {
	std::cout << "Testing" << std::endl;
}



void ProblemManager::run() {
	switch(action) {
		case ProblemAction::GET:
			handleGet();
			break;
		case ProblemAction::TEST:
			handleTest();
			break;
		default:
			std::cerr << "Unknown action" << std::endl;
			break;
		}
	
}




int main(int argc, char** argv) {
	if (argc < 3) {
		std::cerr << "Usage: ./manager <get|test> <problem_id>" << std::endl;
		return 1;
	}

	std::string command = argv[1];
	std::string problemId = argv[2];
	ProblemAction action;	


	if (command == "get") action = ProblemAction::GET;
	else if (command == "test") action = ProblemAction::TEST;
	else action = ProblemAction::ERR;

	ProblemManager manager(action, problemId);
	manager.run();







}
