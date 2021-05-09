initSidebarItems({"fn":[["hash_to_fr","Deterministically create a BLS12-381 field element from a hash"],["hash_to_g1","Hash onto BLS12-381 G1 (random oracle variant) returning zkgroup/pairing object"],["hash_to_miracl_g1","Hash onto BLS12-381 G1 (random oracle variant) returning MIRACL object"],["scalar_multiply","Multiply an element of a group by a scalar. This can be applied to any group over Fr; in particular G1 and G2. The factor can be &FrRepr or &Fr.  The difference is that FrRepr may be larger than the modulus. This is a pure wrapper around the impure function provided by the pairing library."],["sum","Sum elements of a group. This can be applied to elements of any group over Fr; in particular G1 and G2."]],"mod":[["serde","Serde utilities"],["test_utils","Test utilities"]],"type":[["MiraclG1","Type alias for a Miracl BLS12-381 G1 elliptic curve point."]]});