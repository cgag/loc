--[[ This
     is
     a
     multi-line
     comment,
     not
     code. ]]

-- build table
statetab = {}
local w1, w2 = NOWORD, NOWORD
for w in allwords() do
  insert(prefix(w1, w2), w)
  w1 = w2; w2 = w;
end
insert(prefix(w1, w2), NOWORD)
