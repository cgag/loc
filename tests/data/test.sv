// SystemVerilog test file
/* Multi-line comment
   for SystemVerilog */
module test_sv (
    input logic clk,
    input logic [7:0] data_in,
    output logic [7:0] data_out
);

    always_ff @(posedge clk) begin
        data_out <= data_in;
    end

endmodule
