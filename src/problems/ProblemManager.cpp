#include "ProblemManager.h"
#include "json.hpp"
#include <fstream>

ProblemManager::ProblemManager(ProblemAction action, const std::string& problemId, const std::string& problemDirPath) :
	action(action), problemId(problemId) , problemDirPath(problemDirPath) {
}

void ProblemManager::handleGet() {
	
	using json = nlohmann::json;
	
	/* STEP 1 -> Get Problem Params*/
	std::ifstream file("problems.json");
	if (!file) {
		std::cerr << "Could not open problems.json. There is a configuration error." << std::endl;
		return;
	}
	
	// Parse json
	json prob;
	file >> prob;
	//std::string title = prob["title"];

	// Print values to test
	int idx = std::stoi(problemId);
	std::cout << prob["problems"][idx - 1]["title"] << std::endl;

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
	if (argc < 4) {
		std::cerr << "Usage: ./manager <get|test> <problem_id> <path_to_problem_folder>" << std::endl;
		return 1;
	}

	std::string command = argv[1];
	std::string problemId = argv[2];
	std::string problemDirPath = argv[3];
	ProblemAction action;	


	if (command == "get") action = ProblemAction::GET;
	else if (command == "test") action = ProblemAction::TEST;
	else action = ProblemAction::ERR;

	ProblemManager manager(action, problemId, problemDirPath);
	manager.run();







}
