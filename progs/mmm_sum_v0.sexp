(lambda $A
  (lambda $B
    (sum (var $A)
         $_k_Bk_key
         $_k_Bk_val
         (sum (var $_k_Bk_val)
              $_i_Bki_key
              $_i_Bki_val
              (sum (get (var $B) (var $_k_Bk_key))
                   $_j_Ckj_key
                   $_j_Ckj_val
                   (* (var $_i_Bki_val) (var $_j_Ckj_val)))))))
