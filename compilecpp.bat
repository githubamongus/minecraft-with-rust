cd src/cpp
g++ -c -I include/ renderer.cpp
g++ -shared -o ../../renderer.dll renderer.o glad.o
cd ../..
powershell