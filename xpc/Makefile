all: client server

client: client.cpp
	clang++ -O2 -std=c++17 -o client -lbenchmark client.cpp

server: server.c
	clang -o server -O2 server.c

clean:
	rm client
	rm server

install:
	cp server /Library/PrivilegedHelperTools/com.example.benchmark
	launchctl unload /Library/LaunchDaemons/com.example.benchmark.plist
	cp com.example.benchmark.plist /Library/LaunchDaemons/
	launchctl load /Library/LaunchDaemons/com.example.benchmark.plist

uninstall:
	launchctl unload /Library/LaunchDaemons/com.example.benchmark.plist
	rm /Library/PrivilegedHelperTools/com.example.benchmark
