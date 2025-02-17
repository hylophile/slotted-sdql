(lambda $A
  (lambda $B
    (sum (var $B)
         $_i_Bi_key
         $_i_Bi_val
         (sum (var $_i_Bi_val)
              $_j_Bij_key
              $_j_Bij_val
              (sum (var $A)
                   $_k_Ck_key
                   $_k_Ck_val
                   (sum (var $_j_Bij_val)
                        $_l_B_v_key
                        $_l_B_v_val
                        (sing (var $_i_Bi_key)
                              (sing (var $_j_Bij_key)
                                    (sing (var $_k_Ck_key)
                                          (* (var $_l_B_v_val)
                                             (get (var $_k_Ck_val) (var $_l_B_v_key))))))))))))
