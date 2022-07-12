(module
  (import "seal0" "seal_set_storage" (func $set_storage (param i32 i32 i32)))
  (import "seal0" "seal_get_storage" (func $get_storage (param i32 i32 i32) (result i32)))
  (import "seal0" "seal_input" (func $input (param i32 i32)))
  (import "seal0" "seal_return" (func $return (param i32 i32 i32)))
  (import "env" "memory" (memory 1 1)) ;; this gives us one page of memory (16KiB)

  ;; [0, 256)
  ;; we reserve this memory as buffer to receive data

	;; [256, 260)
  ;; initialize memory with length of our buffer (256 in hex)
	(data (i32.const 256) "\00\00\01\00")

  (func (export "deploy")
    ;; we initialize our storage value on contract deployment to '99'
    (i32.store (i32.const 0) (i32.const 99))
    (call $set_storage
      (i32.const 512)   ;; key_ptr
      (i32.const 0)     ;; data_ptr
      (i32.const 4)     ;; data_len
    )
  )

  (func (export "call")
    ;; copy data passed to contract to mem[0]-mem[256]
    ;; it overwrites the data_len_ptr with the actual size of the buffer
    (call $input
      (i32.const 0)   ;; data_ptr
      (i32.const 256) ;; data_len_ptr
    )

    ;; The first four bytes are a 'selector' which is passed by a conforming client
    ;; according to the metadata.json. When writing contracts with ink! this metadata
    ;; file is generated automatically from the source code. The number is inversed because
    ;; wasm uses little endian.
    ;; The rest are the arguments passed.
    (if (i32.eq (i32.const 0xf2017f13) (i32.load (i32.const 0)))
      (then
        ;; this is our 'double' function

        ;; The arguments start after the selector (mem[4])
        (i32.store
          (i32.const 4) ;; overwrite value at mem[4]
          (i32.mul
            (i32.load (i32.const 4))
            (i32.const 2)
          )
        )
        ;; we just pass '512' as key ptr which is zero initialized.
        ;; hence we store our value at a key with all zeros (key is 32byte)
        (call $set_storage
          (i32.const 512)   ;; key_ptr
          (i32.const 4)     ;; data_ptr
          (i32.const 4)     ;; data_len
        )
      )
      (else
        ;; this is our 'get' function

        ;; $get_storage returns a status code which we ignore (drop)
        (drop
          (call $get_storage
            (i32.const 512) ;; key_ptr
            (i32.const 0)   ;; data_ptr
            (i32.const 256) ;; data_len_ptr
          )
        )
        ;; return the value pulled from storage to the caller
        (call $return
          (i32.const 0)   ;; flags (none needed)
          (i32.const 0)   ;; data_ptr
          (i32.const 4)   ;; data_len
        )
      )
    )
  )
)
