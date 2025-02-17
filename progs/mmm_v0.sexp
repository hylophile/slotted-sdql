(lambda $A
  (lambda $B
    (sum (var $A)
         $_i_Bi_key
         $_i_Bi_val
         (sum (var $_i_Bi_val)
              $_k_Bik_key
              $_k_Bik_val
              (sum (get (var $B) (var $_k_Bik_key))
                   $_j_Ckj_key
                   $_j_Ckj_val
                   (sing (var $_i_Bi_key)
                         (sing (var $_j_Ckj_key) (* (var $_k_Bik_val) (var $_j_Ckj_val)))))))))
