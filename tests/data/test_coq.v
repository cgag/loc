(* Coq test file *)
(* Multi-line comment 
   in Coq style *)

Require Import Coq.Arith.Arith.

Theorem add_comm : forall n m : nat,
  n + m = m + n.
Proof.
  intros n m.
  induction n as [| n' IHn'].
  - simpl. rewrite <- plus_n_O. reflexivity.
  - simpl. rewrite -> IHn'. rewrite <- plus_n_Sm. reflexivity.
Qed.

Definition double (n : nat) : nat :=
  n + n.

Lemma double_even : forall n,
  exists k, double n = 2 * k.
Proof.
  intro n.
  exists n.
  unfold double.
  simpl.
  rewrite <- plus_n_O.
  reflexivity.
Qed.
