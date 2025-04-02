#include "ProblemManager.h"
#include "json.hpp"
#include <fstream>
#include <filesystem>

using namespace std;


ProblemManager::ProblemManager(ProblemAction action, const string& problemId, const std::string& problemDirPath) :
	action(action), problemId(problemId) , problemDirPath(problemDirPath) {
}

void ProblemManager::handleGet() {
	
	using json = nlohmann::json;
	
	/* STEP 1 -> Get Problem Params*/
	std::ifstream file(problemDirPath + "/problems.json");
	if (!file) {
		cout << problemDirPath << std::endl;
		cerr << "Could not open problems.json. There is a configuration error." << std::endl;
		return;
	}
	
	// Parse json
	json problems_json;
	file >> problems_json;
	//string title = problems_json["title"];
	
	// Check that problems json is valid
	// If it is not, something in the dev config is wrong
	if (!problems_json.contains("problems")) {
		cerr << "Problems list does not exist in problems.json. There is a configuration error." << std::endl;
		return;
	}
	
	json problems = problems_json["problems"];

	// Get index and ensure it is valid
	int problem_idx = stoi(problemId);
	if (problem_idx > problems_json["problems"].size()) {
		cerr << "Invalid index: " << problem_idx << std::endl;
		return;
	}
	
	// problem holds the problem
	json problem = problems[problem_idx - 1];
	
	// retrieve the relevant parameters

	/*
	if (!problem.contains("functionName") || !problem.contains("outputType") || !problems.contains("inputParams")) {
		std::cerr << "Problem is non populated correctly in problems.json. There is a configuration error." << endl;
		return;
	}
	*/

	string fName = problem["functionName"]; 
	string outputType = problem["outputType"];
	json inputParams = problem["inputParams"];
	
	// Write a function string like:     funcName(Type param1, Type param2)      to be reused
	string funcString = fName + "(";
	
	// populate parameters
	string parameters;
	for (string param : inputParams) {
		parameters += param;
		parameters += ", ";
	}
	if (parameters.size() != 0) { 
		parameters.pop_back();
		parameters.pop_back();
	}
	funcString += parameters;
	funcString += ")";



	/* STEP 2 -> Set up a Solution file*/
	ofstream solutioncpp("Solution.cpp");

	if (!solutioncpp.is_open()) {
		cerr << "Could not create Solution.cpp file. Closing." << std::endl;
		return;
	}

	solutioncpp << "#include \"solution.h\"\n\n\n\n" << outputType << " Solution::" << funcString << " {\n\t\n\t\n\t\n\t\n\t\n}\0";
	solutioncpp.close();

	/* STEP 3 -> Write and link Solution.h*/

	ofstream solutionh("Solution.h");

	if (!solutionh.is_open()) {
		cerr << "Could not create Solution.h file. Closing." << std::endl;
		return;
	}


	solutionh << "class Solution {\n\t\n\t\npublic:\n\t\n\t" << outputType << " " << funcString << ";\n\n\n\n};\n\0";

	solutionh.close();

}


void ProblemManager::handleTest() {
	cout << "Testing" << std::endl;
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
			cerr << "Unknown action" << std::endl;
			break;
		}
	
}




int main(int argc, char** argv) {
	if (argc < 4) {
		cerr << "Usage: ./manager <get|test> <problem_id> <path_to_problem_folder>" << std::endl;
		return 1;
	}

	string command = argv[1];
	string problemId = argv[2];
	string problemDirPath = argv[3];
	ProblemAction action;	


	if (command == "get") action = ProblemAction::GET;
	else if (command == "test") action = ProblemAction::TEST;
	else action = ProblemAction::ERR;

	ProblemManager manager(action, problemId, problemDirPath);
	manager.run();







}
