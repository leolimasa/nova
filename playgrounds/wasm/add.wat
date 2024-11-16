(module 
  (import "js" "mem" (memory 1))
  (import "js" "global" (global $g (mut i32)))
  (import "console" "log" (func $log (param i32 i32))) ;; takes in offset and length
  (import "console" "logRaw" (func $logRaw (param i32 i32))) ;; takes in offset and length
  (table 2 funcref)
  (elem (i32.const 0) $add $global_plus_1)
  ;; (data (i32.const 0) "Hi")
  (func $global_plus_1 (result i32)
    global.get $g
    i32.const 1
    i32.add
  )
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add
  )
  (func $printHi 
    i32.const 0
    i32.const 2
    call $log
  )
  (func $main (result i32)
    (i32.store (i32.const 0) (i32.const 42)) ;; store 42 at offset 0
    i32.const 0 ;; offset
    i32.const 4 ;; length
    call $logRaw

    i32.const 0 ;; offset
    i32.const 4 ;; length
    call $logRaw

    (i32.store (i32.const 1) (i32.const 69)) ;; store 42 at offset 0
    i32.const 0 ;; offset
    i32.const 4 ;; length
    call $logRaw
    
    i32.const 1 ;; return success
  )
  (export "add" (func $add))
  (export "global_plus_1" (func $global_plus_1))
  (export "main" (func $main))
)
