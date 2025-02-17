(lambda $A
  (lambda $A1
    (lambda $A2
      (lambda $B
        (lambda $B1
          (lambda $B2
            (sum
             (sum (var $A)
                  $_i_p_key
                  $_i_p_val
                  (let (get
                        (var $A)
                        (+
                         (var $_i_p_key)
                         1))
                    $q
                    (sing (var $_i_p_key)
                          (sum (subarray (var $A1) (var $_i_p_val) (- (var $q) 1))
                               $_c_j_key
                               $_c_j_val
                               (sing (unique (var $_c_j_val)) (get (var $A2) (var $_c_j_key)))))))
             $_k_Bk_key
             $_k_Bk_val
             (sum (var $_k_Bk_val)
                  $_i_Bki_key
                  $_i_Bki_val
                  (* (var $_i_Bki_val)
                     (sum (get (sum (var $B)
                                    $_i_p_key
                                    $_i_p_val
                                    (let (get
                                          (var $B)
                                          (+
                                           (var $_i_p_key)
                                           1))
                                      $q
                                      (sing (var $_i_p_key)
                                            (sum (subarray (var $B1) (var $_i_p_val) (- (var $q) 1))
                                                 $_c_j_key
                                                 $_c_j_val
                                                 (sing (unique (var $_c_j_val))
                                                       (get (var $B2) (var $_c_j_key)))))))
                               (var $_k_Bk_key))
                          $_j_Ckj_key
                          $_j_Ckj_val
                          (var $_j_Ckj_val)))))))))))
