/***************************************

 Binary Search Trees

***************************************/

/* insert delete inorder search min max */

type binary_tree('a) =
  /* A root node and a left and right sub tree */
  | Node('a, binary_tree('a), binary_tree('a))
  /* Empty leaf */
  | Leaf;
