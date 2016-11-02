 -- simple.adb:  simple example of array declarations and access

procedure Simple is

-- Array type declarations:
--   * index range can be any discrete type
--   * lower and upper bound can be arbitrary
--   * components can have any type

type AT1 is array (1..50)  of Integer;
type AT2 is array (4..457) of Integer;
type AT3 is array (0..9)   of Boolean;
-- type AT4 is array (0..9)   of String(1..5);

type Complex is
record
X, Y: Float;
end record;
type AT5 is array (0..9)   of Complex;

type AT6 is array (1..8)   of AT4;

type AT7 is array (Character range 'A'..'Z') of Float;

type Color is (Red, Orange, Yellow, Green, Blue, Violet);
type AT8 is array (Orange..Blue) of Boolean;
type AT9 is array (Color'Range)  of Character;

A:AT1; B:AT2; C:AT3; D:AT4; E:AT5; F:AT6; G:AT7; H:AT8; I:AT9;

N : constant Integer := 1;

begin

A(2*N+5) := 4_567;
B(N+4)   := 4_567;
C(N)     := True;
D(3*N)   := "ABCDE";
E(0)     := Complex' (X=>6.7, Y=>5.6);
F(3)     := AT4' (AT4'Range => "XXXXX");
F(3)(1)(5) := 'E';
G(Character'Succ('E')) := 2.9;
H(Color'Pred(Yellow))  := True;
I(Red)   := 'Q';

end Simple;
