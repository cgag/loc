// Verilog test file
// This is a single line comment
/* This is a 
   multi-line comment
   in Verilog */
module test_module (
    input wire clk,
    input wire reset,
    output reg [7:0] data_out
);

    reg [7:0] counter;
    
    // Counter logic
    always @(posedge clk or posedge reset) begin
        if (reset) begin
            counter <= 8'b0;
        end else begin
            counter <= counter + 1;
        end
    end
    
    assign data_out = counter;

endmodule
