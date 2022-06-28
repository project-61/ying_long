

build:
	verilator -Wall --cc --exe --build --trace ./main.cpp ./top.v ./add.v

run: build
	./obj_dir/Vtop

clean:
	rm -rf ./obj_dir