#include <iostream>
#include <fstream>
#include <vector>

int main(int argc, char** argv) {
	if (argc < 2) {
		std::cout << "Usage: " << argv[0] << " <input_file>" << std::endl;
		return 1;
	}

	std::ifstream input_file;

	input_file.open(argv[1], std::ifstream::in);
	if (!input_file.is_open()) {
		std::cout << "Could not open file: " << argv[1] << std::endl;
		return 1;
	}

	std::vector<std::string> lines;
	std::string line;
	while (std::getline(input_file, line)) {
		lines.push_back(line);
	}
	input_file.close();

	int sum = 0;
	for (auto line : lines) {
		sum += 10 * (line[line.find_first_of("0123456789", 0)] - '0');
		sum += line[line.find_last_of("0123456789", std::string::npos)] - '0';
	}

	std::cout << "Result: " << sum << std::endl;

	return 0;
}