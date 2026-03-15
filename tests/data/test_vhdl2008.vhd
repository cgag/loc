-- VHDL 2008 test file with multi-line comments
-- Single line comment
/* This is a VHDL 2008
   multi-line comment
   supported since VHDL-2008 standard */

library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

entity counter_vhdl2008 is
    Port ( 
        clk : in  STD_LOGIC;
        reset : in  STD_LOGIC;
        enable : in  STD_LOGIC;
        count_out : out  STD_LOGIC_VECTOR (15 downto 0)
    );
end counter_vhdl2008;

architecture RTL of counter_vhdl2008 is
    signal count_reg : unsigned(15 downto 0) := (others => '0');
begin
    /* Counter process with
       synchronous reset */
    counter_proc: process(clk)
    begin
        if rising_edge(clk) then
            if reset = '1' then
                count_reg <= (others => '0');
            elsif enable = '1' then
                count_reg <= count_reg + 1;
            end if;
        end if;
    end process counter_proc;
    
    -- Output assignment
    count_out <= std_logic_vector(count_reg);
end RTL;
