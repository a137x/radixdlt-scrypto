(module
  (import "env" "radix_engine" (func $radix_engine (param i32) (result i32)))

  ;; scrypto_encode(&RadixEngineInput::EmitLog(Level::Debug, "Hello".to_string()));
  (data (i32.const 1024) "\11\07\00\00\00EmitLog\02\00\00\00\11\05\00\00\00Debug\00\00\00\00\0c\05\00\00\00Hello")

  ;; Simple function that always returns `()`
  (func $Test_f (param $0 i32) (result i32)
    (local $buffer i32)
    (local.set 
      $buffer
      (call $scrypto_alloc
        (i32.const 40)
      )
    )
    (drop
      (call $memcpy
        (i32.add
          (local.get $buffer)
          (i32.const 4)
        )
        (i32.const 1024)
        (i32.const 40)
      )
    )
    (drop
      (call $radix_engine
        (local.get $buffer)
      )
    )

    ;; TO RETURN:
    ;; Now we need to allocate the return SBOR buffer: We need 3 bytes to respond with ()
    ;; $scrypto_alloc returns a pointer - the first 4 bytes aren't relevant to use, we start writing our response after that
    (local.set 
      $buffer
      (call $scrypto_alloc
        (i32.const 3)
      )
    )

    ;; PART 1: Encode our Scrypto payload prefix (92) as a byte (8 bits), at offset 4 from the pointer
    (i32.add
      (local.get $buffer)
      (i32.const 4)
    )
    (i32.const 92)
    (i32.store8)

    ;; PART 2: We need to write two more 0 bytes to encode our unit, at offset 4 + 1 from the pointer
    (i32.add
      (local.get $buffer)
      (i32.const 5)
    )
    (i32.const 0)
    (i32.store16)

    ;; We're finished! Return the pointer
    (local.get $buffer)
  )

  (memory $0 1)
  (export "memory" (memory $0))
  (export "scrypto_alloc" (func $scrypto_alloc))
  (export "scrypto_free" (func $scrypto_free))
  (export "Test_f" (func $Test_f))

  ${memcpy}
  ${buffer}
)