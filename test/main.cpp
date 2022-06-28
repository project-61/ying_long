#include <iostream>

#include "./Vadd_demo.h"


#include "Vadd.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

using namespace std;

int main(int argc, char** argv, char** env) {
    printf("running\n");
    VerilatedContext* contextp = new VerilatedContext;
    contextp->commandArgs(argc, argv);
    Vadd* adder = new Vadd{contextp};

    Verilated::traceEverOn(true);
    // VerilatedVcdC* tfp = new VerilatedVcdC;
    // top->trace(tfp, 99); // Trace 99 levels of hierarchy
    // tfp->open("./obj_dir/simx.vcd");

    const auto sim_time = 99;
    while (contextp->time() < sim_time && !contextp->gotFinish()) {
        contextp->timeInc(1);
        int a = rand() & 5;
        int b = rand() & 5;
        adder->a = a;
        adder->b = b;
        adder->eval();

        cout << "a: " << adder->a << ", b: " << adder->b << ", c: " << adder->c << endl;
        // tfp->dump(contextp->time());
    }
    // tfp->close();
    // delete top;
    // delete contextp;
    return 0;
}