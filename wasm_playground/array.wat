(module
  (type $arr32 (array i32))
  (func $main (result i32)
    (local $a (ref $arr32))

    ;; create array
    ;; type, initial value, length
    (array.new $arr32 (i32.const 0) (i32.const 10))
    local.set $a

    ;; Put two elements in array
    (array.set $arr32 (local.get $a) (i32.const 0) (i32.const 42))

    ;; return second element
    ;;(array.get $arr32 (local.get $a) (i32.const 2))

    ;; return the array length
    ;;(array.len (local.get $a))
  )
  (export "main" (func $main))
)
