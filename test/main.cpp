#include <iostream>

#include "Vtop.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

using namespace std;

int main(int argc, char** argv, char** env) {
    printf("running\n");
    VerilatedContext* contextp = new VerilatedContext;
    contextp->commandArgs(argc, argv);
    Vtop* top = new Vtop{contextp};

    Verilated::traceEverOn(true);
    // VerilatedVcdC* tfp = new VerilatedVcdC;
    // top->trace(tfp, 99); // Trace 99 levels of hierarchy
    // tfp->open("./obj_dir/simx.vcd");

    const auto sim_time = 99;
    while (contextp->time() < sim_time && !contextp->gotFinish()) {
        contextp->timeInc(1);
        int a = rand() & 5;
        int b = rand() & 5;
        top->a = a;
        top->b = b;
        top->eval();

        cout << "a: " << top->a << ", b: " << top->b << ", c: " << top->c << endl;
        // tfp->dump(contextp->time());
    }
    // tfp->close();
    // delete top;
    // delete contextp;
    return 0;
}