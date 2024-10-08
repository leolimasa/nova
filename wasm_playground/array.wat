(module
  (type $arr32 (array i32))
  (func $main (result (ref $arr32))
    (local $a (ref $arr32))
    (local.set $a (array.new $arr32
      (i32.const 69)
      (i32.const 42)
      (i32.const 2)
    ))
    local.get $a
  )
)
