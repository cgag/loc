(***************************************

 Binary Search Trees

***************************************)

(* insert delete inorder search min max *)

type 'a binary_tree =
  (* A root node and a left and right sub tree *)
  Node of 'a * 'a binary_tree * 'a binary_tree
  (* Empty leaf *)
  | Leaf
