(lambda $var_01
  (lambda $var_02
    (lambda $var_03
      (lambda $var_04
        (lambda $var_05
          (lambda $var_06
            (lambda $var_07
              (lambda $var_08
                (lambda $var_09
                  (lambda $var_10
                    (lambda $var_11
                      (lambda $var_12
                        (sum
                         (sum (var $var_01)
                              $var_13
                              $var_14
                              (let (get
                                    (var $var_02)
                                    (var $var_13))
                                $var_15
                                (let (get
                                      (var $var_02)
                                      (+
                                       (var $var_13)
                                       1))
                                  $var_16
                                  (sing
                                   (unique (var $var_14))
                                   (sum (subarray (var $var_03) (var $var_15) (- (var $var_16) 1))
                                        $var_17
                                        $var_18
                                        (let (get
                                              (var $var_04)
                                              (var $var_17))
                                          $var_19
                                          (let (get
                                                (var $var_04)
                                                (+
                                                 (var $var_17)
                                                 1))
                                            $var_20
                                            (sing (unique (var $var_18))
                                                  (sum (subarray (var $var_05)
                                                                 (var $var_19)
                                                                 (- (var $var_20) 1))
                                                       $var_21
                                                       $var_22
                                                       (let (get
                                                             (var $var_06)
                                                             (var $var_21))
                                                         $var_23
                                                         (sing (unique (var $var_22))
                                                               (var $var_23))))))))))))
                         $var_13
                         $var_14
                         (sing
                          (var $var_13)
                          (sum
                           (var $var_14)
                           $var_15
                           $var_16
                           (sum (get (sum (var $var_07)
                                          $var_17
                                          $var_18
                                          (let (get
                                                (var $var_07)
                                                (+
                                                 (var $var_17)
                                                 1))
                                            $var_19
                                            (sing (var $var_17)
                                                  (sum (subarray (var $var_08)
                                                                 (var $var_18)
                                                                 (- (var $var_19) 1))
                                                       $var_20
                                                       $var_21
                                                       (sing (unique (var $var_21))
                                                             (get (var $var_09) (var $var_20)))))))
                                     (var $var_15))
                                $var_17
                                $var_18
                                (sing
                                 (var $var_17)
                                 (* (var $var_18)
                                    (sum (var $var_16)
                                         $var_19
                                         $var_20
                                         (* (var $var_20)
                                            (get (get (sum (var $var_10)
                                                           $var_21
                                                           $var_22
                                                           (let (get
                                                                 (var $var_10)
                                                                 (+
                                                                  (var $var_21)
                                                                  1))
                                                             $var_23
                                                             (sing (var $var_21)
                                                                   (sum (subarray (var $var_11)
                                                                                  (var $var_22)
                                                                                  (- (var $var_23) 1))
                                                                        $var_24
                                                                        $var_25
                                                                        (sing (unique (var $var_25))
                                                                              (get (var $var_12)
                                                                                   (var $var_24)))))))
                                                      (var $var_17))
                                                 (var $var_19))))))))))))))))))))))
