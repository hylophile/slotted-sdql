(lambda $B
  (lambda $C
    (lambda $D
      (sum (var $B)
           $_i_Bi_key
           $_i_Bi_val
           (sum (var $_i_Bi_val)
                $_k_Bik_key
                $_k_Bik_val
                (sum (get (var $C) (var $_k_Bik_key))
                     $_j_C_v_key
                     $_j_C_v_val
                     (sum (var $_k_Bik_val)
                          $_l_B_v_key
                          $_l_B_v_val
                          (sing (var $_i_Bi_key)
                                (sing (var $_j_C_v_key)
                                      (* (* (var $_j_C_v_val) (var $_l_B_v_val))
                                         (get (get (var $D) (var $_j_C_v_key))
                                              (var $_l_B_v_key))))))))))))
