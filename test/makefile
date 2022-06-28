

build:
	verilator -Wall --cc --exe --build --trace -Wno-DECLFILENAME ./main.cpp ./top.v

run: build
	./obj_dir/Vtop

clean:
	rm -rf ./obj_dir