// `include "add.v"

module top (
	input [32-1:0]	a,
	input [32-1:0]	b,
	output [32-1:0]	c
	);




add a1 (.b (b),.a (a),.c (c));

endmodule;

module add (
	input [32-1:0]	a,
	input [32-1:0]	b,
	output [32-1:0]	c
	);


	assign c = a + b;



endmodule;
