-- VHDL test file
-- This is a single line comment
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity test_entity is
    Port ( 
        clk : in  STD_LOGIC;
        reset : in  STD_LOGIC;
        data_out : out  STD_LOGIC_VECTOR (7 downto 0)
    );
end test_entity;

architecture Behavioral of test_entity is
    signal counter : integer := 0;
begin
    -- Counter process
    process(clk, reset)
    begin
        if reset = '1' then
            counter <= 0;
        elsif rising_edge(clk) then
            counter <= counter + 1;
        end if;
    end process;
    
    data_out <= std_logic_vector(to_unsigned(counter, 8));
end Behavioral;
