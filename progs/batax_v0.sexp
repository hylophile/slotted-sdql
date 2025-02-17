(lambda $B
  (lambda $C
    (lambda $D
      (sum (var $C)
           $_i_Ai_key
           $_i_Ai_val
           (sum (var $_i_Ai_val)
                $_j_a_val_key
                $_j_a_val_val
                (sum (var $_i_Ai_val)
                     $_k_a2_val_key
                     $_k_a2_val_val
                     (sing (var $_j_a_val_key)
                           (* (* (* (var $B) (var $_j_a_val_val)) (var $_k_a2_val_val))
                              (get (var $D) (var $_k_a2_val_key))))))))))
