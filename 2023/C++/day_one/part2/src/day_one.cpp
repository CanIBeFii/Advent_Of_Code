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

	std::vector<std::string> matches{"zero", "one", "two", "three", "four", "five", "six" \
		, "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"};
	
	int sum = 0;
	int count = 1;
	for (auto line : lines) {
		size_t first_pos = line.size();
		int	first_num = 0;
		size_t last_pos = 0;
		int last_num = 0;
		int num_count = 0;
		for (auto number : matches) {
			size_t temp = 0;
			temp = line.find(number, 0);
			if (temp != std::string::npos && temp < first_pos) {
				first_pos = temp;
				first_num = num_count % 10;
			}
 			temp = 0;
			temp = line.rfind(number, std::string::npos);
			if (temp != std::string::npos && temp > last_pos) {
				last_pos = temp;
				last_num = num_count % 10;
			}
			num_count++;
		}
		if ( last_num == 0)
			last_num = first_num; //  Some how if there's only a single number, it's not found by rfind don't know why yet so last = first
		std::cout << "First: " << first_num << " Last: " << last_num << std::endl;
		sum += 10 * first_num + last_num;
		count++;
	}

	std::cout << "Result: " << sum << std::endl;

	return 0;
}