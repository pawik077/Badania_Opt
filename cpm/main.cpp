#include <iostream>
#include <fstream>

int main(int argc, char* argv[]) {
	std::ifstream file;
	if (argc == 2) {
		file.open(argv[1]);
	} else {
		file.open("input.txt");
	}
	// *** file read ***
	int n;
	file >> n;
	int** data = new int* [n];
	int* times = new int[n];;
	for (int i = 0; i < n; i++) {
		data[i] = new int[n];
		for (int j = 0; j < n; j++) {
			file >> data[i][j];
		}
	}
	for (int i = 0; i < n; i++) {
		file >> times[i];
	}
	file.close();
#ifdef _DEBUG
	std::cout << n << std::endl;
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n; j++) {
			std::cout << data[i][j] << " ";
		}
		std::cout << "\n";
	}
	std::cout << std::endl;
	for (int i = 0; i < n; i++) {
		std::cout << times[i] << " ";
	}
	std::cout << std::endl;
#endif
	int* ESs = new int[n];
	int* EFs = new int[n];
	int* LSs = new int[n];
	int* LFs = new int[n];
	// *** forward operations (early start, early finish) ***
	ESs[0] = 0;
	EFs[0] = 0;
	for (int i = 0; i < n; i++) {
		int max = 0;
		for (int j = 0; j < i; j++) {
			if(i == j) continue;
			if (data[j][i] == 1) {
				if (EFs[j] > max) {
					max = EFs[j];
				}
			}
		}
		ESs[i] = max;
		EFs[i] = max + times[i];
	}
#ifdef _DEBUG
	for (int i = 0; i < n; i++) {
		std::cout << ESs[i] << " ";
	}
	std::cout << std::endl;
	for (int i = 0; i < n; i++) {
		std::cout << EFs[i] << " ";
	}
	std::cout << std::endl;
#endif
	// *** backward operations (late start, late finish) ***
	LSs[n - 1] = EFs[n - 1];
	LFs[n - 1] = EFs[n - 1];
	for (int i = n - 1; i >= 0; i--) {
		int min = EFs[n - 1];
		for (int j = n - 1; j > i; j--) {
			if (i == j) continue;
			if (data[i][j] == 1) {
				if (LSs[j] < min) {
					min = LSs[j];
				}
			}
		}
		LSs[i] = min - times[i];
		LFs[i] = min;
	}
#ifdef _DEBUG
	for (int i = 0; i < n; i++) {
		std::cout << LSs[i] << " ";
	}
	std::cout << std::endl;
	for (int i = 0; i < n; i++) {
		std::cout << LFs[i] << " ";
	}
	std::cout << std::endl;
#endif
	// *** critical path ***
	int* TFs = new int[n];
	int* criticalPath = new int[n];
	int criticalPathLength = 0;
	for (int i = 0; i < n; i++) {
		TFs[i] = LSs[i] - ESs[i];
		if (TFs[i] == 0) {
			criticalPath[criticalPathLength] = i;
			criticalPathLength++;
		}
	}
#ifdef _DEBUG
	for (int i = 0; i < n; i++) {
		std::cout << TFs[i] << " ";
	}
	std::cout << std::endl;
	for (int i = 0; i < criticalPathLength; i++) {
		std::cout << criticalPath[i] << " ";
	}
	std::cout << std::endl;
#endif
	// *** output ***
	std::cout << "Tasks:\n";
	for (int i = 0; i < n; i++) {
		std::cout << "Task " << i <<
			": Time: " << (times[i] < 10 ? " " : "") << times[i] <<
			", ES: " << (ESs[i] < 10 ? " " : "") << ESs[i] <<
			", EF: " << (EFs[i] < 10 ? " " : "") << EFs[i] <<
			", LS: " << (LSs[i] < 10 ? " " : "") << LSs[i] <<
			", LF: " << (LFs[i] < 10 ? " " : "") << LFs[i] <<
			", TF: " << (TFs[i] < 10 ? " " : "") << TFs[i] << "\n";
	}
	std::cout << std::endl;
	std::cout << "Critical path: ";
	for (int i = 0; i < criticalPathLength; i++) {
		std::cout << criticalPath[i] << " ";
	}
	std::cout << std::endl;
	return 0;
}