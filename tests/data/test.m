% MATLAB test file
function y = square_plus_one(x)
% Compute x^2 + 1
y = x.^2 + 1;
%{
Block comment
still comment
%}
if x > 0
    y = y + 1;
end
end
