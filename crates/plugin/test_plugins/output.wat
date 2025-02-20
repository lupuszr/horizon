(module $rust_example.wasm
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func))
  (type (;6;) (func (param i32 i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32)))
  (type (;8;) (func (result i32)))
  (type (;9;) (func (param i32 i32 i32) (result i32)))
  (import "__wbindgen_externref_xform__" "__wbindgen_externref_table_grow" (func $_ZN12wasm_bindgen9externref31__wbindgen_externref_table_grow17he19b8e5bd557ee78E (;0;) (type 2)))
  (import "__wbindgen_externref_xform__" "__wbindgen_externref_table_set_null" (func $_ZN12wasm_bindgen9externref35__wbindgen_externref_table_set_null17hf10811798d139b72E (;1;) (type 1)))
  (table (;0;) 4 4 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1049442)
  (global (;2;) i32 i32.const 1049456)
  (export "memory" (memory 0))
  (export "__wbindgen_malloc" (func $__wbindgen_malloc))
  (export "__wbindgen_realloc" (func $__wbindgen_realloc))
  (export "__wbindgen_free" (func $__wbindgen_free))
  (export "__wbindgen_exn_store" (func $__wbindgen_exn_store))
  (export "__externref_table_alloc" (func $__externref_table_alloc))
  (export "__externref_table_dealloc" (func $__externref_table_dealloc))
  (export "__externref_drop_slice" (func $__externref_drop_slice))
  (export "__externref_heap_live_count" (func $__externref_heap_live_count))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN4core5panic12PanicPayload6as_str17h201ff5ad12170af0E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$6as_str17hc567c4039abaef48E $_ZN4core3ops8function6FnOnce9call_once17he41aae0b83180893E)
  (func $__rust_realloc (;2;) (type 3) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 0
              i32.const -4
              i32.add
              local.tee 4
              i32.load
              local.tee 5
              i32.const -8
              i32.and
              local.tee 6
              i32.const 4
              i32.const 8
              local.get 5
              i32.const 3
              i32.and
              local.tee 7
              select
              local.get 1
              i32.add
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 39
              i32.add
              local.set 8
              block ;; label = @6
                local.get 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                local.get 8
                i32.gt_u
                br_if 2 (;@4;)
              end
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 2
                    i32.const 9
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 2
                    local.get 3
                    call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17h0c9b7ed4b8acf74dE
                    local.tee 2
                    br_if 1 (;@7;)
                    i32.const 0
                    return
                  end
                  i32.const 0
                  local.set 2
                  local.get 3
                  i32.const -65588
                  i32.gt_u
                  br_if 1 (;@6;)
                  i32.const 16
                  local.get 3
                  i32.const 11
                  i32.add
                  i32.const -8
                  i32.and
                  local.get 3
                  i32.const 11
                  i32.lt_u
                  select
                  local.set 1
                  block ;; label = @8
                    block ;; label = @9
                      local.get 7
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 256
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 6
                      local.get 1
                      i32.const 4
                      i32.or
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 6
                      local.get 1
                      i32.sub
                      i32.const 131073
                      i32.ge_u
                      br_if 1 (;@8;)
                      local.get 0
                      return
                    end
                    local.get 0
                    i32.const -8
                    i32.add
                    local.tee 8
                    local.get 6
                    i32.add
                    local.set 7
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              local.get 6
                              local.get 1
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 7
                              i32.const 0
                              i32.load offset=1049412
                              i32.eq
                              br_if 4 (;@9;)
                              local.get 7
                              i32.const 0
                              i32.load offset=1049408
                              i32.eq
                              br_if 2 (;@11;)
                              local.get 7
                              i32.load offset=4
                              local.tee 5
                              i32.const 2
                              i32.and
                              br_if 5 (;@8;)
                              local.get 5
                              i32.const -8
                              i32.and
                              local.tee 9
                              local.get 6
                              i32.add
                              local.tee 5
                              local.get 1
                              i32.lt_u
                              br_if 5 (;@8;)
                              local.get 7
                              local.get 9
                              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h7dc40cdafd0f02adE
                              local.get 5
                              local.get 1
                              i32.sub
                              local.tee 3
                              i32.const 16
                              i32.lt_u
                              br_if 1 (;@12;)
                              local.get 4
                              local.get 1
                              local.get 4
                              i32.load
                              i32.const 1
                              i32.and
                              i32.or
                              i32.const 2
                              i32.or
                              i32.store
                              local.get 8
                              local.get 1
                              i32.add
                              local.tee 1
                              local.get 3
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 8
                              local.get 5
                              i32.add
                              local.tee 2
                              local.get 2
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              local.get 1
                              local.get 3
                              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17he6d9c654dc1b75fdE
                              local.get 0
                              return
                            end
                            local.get 6
                            local.get 1
                            i32.sub
                            local.tee 3
                            i32.const 15
                            i32.gt_u
                            br_if 2 (;@10;)
                            local.get 0
                            return
                          end
                          local.get 4
                          local.get 5
                          local.get 4
                          i32.load
                          i32.const 1
                          i32.and
                          i32.or
                          i32.const 2
                          i32.or
                          i32.store
                          local.get 8
                          local.get 5
                          i32.add
                          local.tee 1
                          local.get 1
                          i32.load offset=4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 0
                          return
                        end
                        i32.const 0
                        i32.load offset=1049400
                        local.get 6
                        i32.add
                        local.tee 7
                        local.get 1
                        i32.lt_u
                        br_if 2 (;@8;)
                        block ;; label = @11
                          block ;; label = @12
                            local.get 7
                            local.get 1
                            i32.sub
                            local.tee 3
                            i32.const 15
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 4
                            local.get 5
                            i32.const 1
                            i32.and
                            local.get 7
                            i32.or
                            i32.const 2
                            i32.or
                            i32.store
                            local.get 8
                            local.get 7
                            i32.add
                            local.tee 1
                            local.get 1
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.set 3
                            i32.const 0
                            local.set 1
                            br 1 (;@11;)
                          end
                          local.get 4
                          local.get 1
                          local.get 5
                          i32.const 1
                          i32.and
                          i32.or
                          i32.const 2
                          i32.or
                          i32.store
                          local.get 8
                          local.get 1
                          i32.add
                          local.tee 1
                          local.get 3
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 8
                          local.get 7
                          i32.add
                          local.tee 2
                          local.get 3
                          i32.store
                          local.get 2
                          local.get 2
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                        end
                        i32.const 0
                        local.get 1
                        i32.store offset=1049408
                        i32.const 0
                        local.get 3
                        i32.store offset=1049400
                        local.get 0
                        return
                      end
                      local.get 4
                      local.get 1
                      local.get 5
                      i32.const 1
                      i32.and
                      i32.or
                      i32.const 2
                      i32.or
                      i32.store
                      local.get 8
                      local.get 1
                      i32.add
                      local.tee 1
                      local.get 3
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 7
                      local.get 7
                      i32.load offset=4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 3
                      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17he6d9c654dc1b75fdE
                      local.get 0
                      return
                    end
                    i32.const 0
                    i32.load offset=1049404
                    local.get 6
                    i32.add
                    local.tee 7
                    local.get 1
                    i32.gt_u
                    br_if 7 (;@1;)
                  end
                  local.get 3
                  call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hed8b206a09582210E
                  local.tee 1
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 1
                  local.get 0
                  i32.const -4
                  i32.const -8
                  local.get 4
                  i32.load
                  local.tee 2
                  i32.const 3
                  i32.and
                  select
                  local.get 2
                  i32.const -8
                  i32.and
                  i32.add
                  local.tee 2
                  local.get 3
                  local.get 2
                  local.get 3
                  i32.lt_u
                  select
                  call $memcpy
                  local.set 1
                  local.get 0
                  call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17hd371afab6b2bc9caE
                  local.get 1
                  return
                end
                local.get 2
                local.get 0
                local.get 1
                local.get 3
                local.get 1
                local.get 3
                i32.lt_u
                select
                call $memcpy
                drop
                local.get 4
                i32.load
                local.tee 3
                i32.const -8
                i32.and
                local.tee 7
                i32.const 4
                i32.const 8
                local.get 3
                i32.const 3
                i32.and
                local.tee 3
                select
                local.get 1
                i32.add
                i32.lt_u
                br_if 3 (;@3;)
                block ;; label = @7
                  local.get 3
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  local.get 8
                  i32.gt_u
                  br_if 5 (;@2;)
                end
                local.get 0
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17hd371afab6b2bc9caE
              end
              local.get 2
              return
            end
            i32.const 1048617
            i32.const 1048664
            call $_ZN4core9panicking5panic17hfef8090705073b49E
            unreachable
          end
          i32.const 1048680
          i32.const 1048728
          call $_ZN4core9panicking5panic17hfef8090705073b49E
          unreachable
        end
        i32.const 1048617
        i32.const 1048664
        call $_ZN4core9panicking5panic17hfef8090705073b49E
        unreachable
      end
      i32.const 1048680
      i32.const 1048728
      call $_ZN4core9panicking5panic17hfef8090705073b49E
      unreachable
    end
    local.get 4
    local.get 1
    local.get 5
    i32.const 1
    i32.and
    i32.or
    i32.const 2
    i32.or
    i32.store
    local.get 8
    local.get 1
    i32.add
    local.tee 3
    local.get 7
    local.get 1
    i32.sub
    local.tee 1
    i32.const 1
    i32.or
    i32.store offset=4
    i32.const 0
    local.get 1
    i32.store offset=1049404
    i32.const 0
    local.get 3
    i32.store offset=1049412
    local.get 0
  )
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17h0c9b7ed4b8acf74dE (;3;) (type 4) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 2
    block ;; label = @1
      i32.const -65587
      local.get 0
      i32.const 16
      local.get 0
      i32.const 16
      i32.gt_u
      select
      local.tee 0
      i32.sub
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      local.get 1
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 3
      i32.add
      i32.const 12
      i32.add
      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hed8b206a09582210E
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -8
      i32.add
      local.set 2
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.const -1
          i32.add
          local.tee 4
          local.get 1
          i32.and
          br_if 0 (;@3;)
          local.get 2
          local.set 0
          br 1 (;@2;)
        end
        local.get 1
        i32.const -4
        i32.add
        local.tee 5
        i32.load
        local.tee 6
        i32.const -8
        i32.and
        local.get 4
        local.get 1
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 1
        i32.const 0
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i32.const 16
        i32.gt_u
        select
        i32.add
        local.tee 0
        local.get 2
        i32.sub
        local.tee 1
        i32.sub
        local.set 4
        block ;; label = @3
          local.get 6
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 0
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 1
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 2
          local.get 1
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17he6d9c654dc1b75fdE
          br 1 (;@2;)
        end
        local.get 2
        i32.load
        local.set 2
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 0
        local.get 2
        local.get 1
        i32.add
        i32.store
      end
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        local.tee 2
        local.get 3
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 3
        i32.add
        local.tee 1
        local.get 2
        local.get 3
        i32.sub
        local.tee 3
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 3
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17he6d9c654dc1b75fdE
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 2
    end
    local.get 2
  )
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h7dc40cdafd0f02adE (;4;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.load offset=12
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 256
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=24
          local.set 3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 2
                local.get 0
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 20
                i32.const 16
                local.get 0
                i32.load offset=20
                local.tee 2
                select
                i32.add
                i32.load
                local.tee 1
                br_if 1 (;@5;)
                i32.const 0
                local.set 2
                br 2 (;@4;)
              end
              local.get 0
              i32.load offset=8
              local.tee 1
              local.get 2
              i32.store offset=12
              local.get 2
              local.get 1
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 0
            i32.const 20
            i32.add
            local.get 0
            i32.const 16
            i32.add
            local.get 2
            select
            local.set 4
            loop ;; label = @5
              local.get 4
              local.set 5
              local.get 1
              local.tee 2
              i32.const 20
              i32.add
              local.get 2
              i32.const 16
              i32.add
              local.get 2
              i32.load offset=20
              local.tee 1
              select
              local.set 4
              local.get 2
              i32.const 20
              i32.const 16
              local.get 1
              select
              i32.add
              i32.load
              local.tee 1
              br_if 0 (;@5;)
            end
            local.get 5
            i32.const 0
            i32.store
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          block ;; label = @4
            local.get 0
            i32.load offset=28
            i32.const 2
            i32.shl
            i32.const 1048984
            i32.add
            local.tee 1
            i32.load
            local.get 0
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            i32.const 16
            i32.const 20
            local.get 3
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 2
            i32.store
            local.get 2
            i32.eqz
            br_if 3 (;@1;)
            br 2 (;@2;)
          end
          local.get 1
          local.get 2
          i32.store
          local.get 2
          br_if 1 (;@2;)
          i32.const 0
          i32.const 0
          i32.load offset=1049396
          i32.const -2
          local.get 0
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store offset=1049396
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 2
          local.get 0
          i32.load offset=8
          local.tee 4
          i32.eq
          br_if 0 (;@3;)
          local.get 4
          local.get 2
          i32.store offset=12
          local.get 2
          local.get 4
          i32.store offset=8
          return
        end
        i32.const 0
        i32.const 0
        i32.load offset=1049392
        i32.const -2
        local.get 1
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store offset=1049392
        return
      end
      local.get 2
      local.get 3
      i32.store offset=24
      block ;; label = @2
        local.get 0
        i32.load offset=16
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.store offset=16
        local.get 1
        local.get 2
        i32.store offset=24
      end
      local.get 0
      i32.load offset=20
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.store offset=20
      local.get 1
      local.get 2
      i32.store offset=24
      return
    end
  )
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17he6d9c654dc1b75fdE (;5;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 3
        local.get 1
        i32.add
        local.set 1
        block ;; label = @3
          local.get 0
          local.get 3
          i32.sub
          local.tee 0
          i32.const 0
          i32.load offset=1049408
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 1
          i32.store offset=1049400
          local.get 2
          local.get 2
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h7dc40cdafd0f02adE
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 3
              i32.const 2
              i32.and
              br_if 0 (;@5;)
              local.get 2
              i32.const 0
              i32.load offset=1049412
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 0
              i32.load offset=1049408
              i32.eq
              br_if 3 (;@2;)
              local.get 2
              local.get 3
              i32.const -8
              i32.and
              local.tee 3
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h7dc40cdafd0f02adE
              local.get 0
              local.get 3
              local.get 1
              i32.add
              local.tee 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              local.get 0
              i32.const 0
              i32.load offset=1049408
              i32.ne
              br_if 1 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1049400
              return
            end
            local.get 2
            local.get 3
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
          end
          block ;; label = @4
            local.get 1
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            i32.const 31
            local.set 2
            block ;; label = @5
              local.get 1
              i32.const 16777215
              i32.gt_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 6
              local.get 1
              i32.const 8
              i32.shr_u
              i32.clz
              local.tee 2
              i32.sub
              i32.shr_u
              i32.const 1
              i32.and
              local.get 2
              i32.const 1
              i32.shl
              i32.sub
              i32.const 62
              i32.add
              local.set 2
            end
            local.get 0
            i64.const 0
            i64.store offset=16 align=4
            local.get 0
            local.get 2
            i32.store offset=28
            local.get 2
            i32.const 2
            i32.shl
            i32.const 1048984
            i32.add
            local.set 3
            block ;; label = @5
              i32.const 0
              i32.load offset=1049396
              i32.const 1
              local.get 2
              i32.shl
              local.tee 4
              i32.and
              br_if 0 (;@5;)
              local.get 3
              local.get 0
              i32.store
              local.get 0
              local.get 3
              i32.store offset=24
              local.get 0
              local.get 0
              i32.store offset=12
              local.get 0
              local.get 0
              i32.store offset=8
              i32.const 0
              i32.const 0
              i32.load offset=1049396
              local.get 4
              i32.or
              i32.store offset=1049396
              return
            end
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.load
                  local.tee 4
                  i32.load offset=4
                  i32.const -8
                  i32.and
                  local.get 1
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 4
                  local.set 2
                  br 1 (;@6;)
                end
                local.get 1
                i32.const 0
                i32.const 25
                local.get 2
                i32.const 1
                i32.shr_u
                i32.sub
                local.get 2
                i32.const 31
                i32.eq
                select
                i32.shl
                local.set 3
                loop ;; label = @7
                  local.get 4
                  local.get 3
                  i32.const 29
                  i32.shr_u
                  i32.const 4
                  i32.and
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 5
                  i32.load
                  local.tee 2
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 3
                  i32.const 1
                  i32.shl
                  local.set 3
                  local.get 2
                  local.set 4
                  local.get 2
                  i32.load offset=4
                  i32.const -8
                  i32.and
                  local.get 1
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 2
              i32.load offset=8
              local.tee 1
              local.get 0
              i32.store offset=12
              local.get 2
              local.get 0
              i32.store offset=8
              local.get 0
              i32.const 0
              i32.store offset=24
              local.get 0
              local.get 2
              i32.store offset=12
              local.get 0
              local.get 1
              i32.store offset=8
              return
            end
            local.get 5
            local.get 0
            i32.store
            local.get 0
            local.get 4
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 0
            i32.store offset=8
            return
          end
          local.get 1
          i32.const 248
          i32.and
          i32.const 1049128
          i32.add
          local.set 2
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load offset=1049392
              local.tee 3
              i32.const 1
              local.get 1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 1
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 3
              local.get 1
              i32.or
              i32.store offset=1049392
              local.get 2
              local.set 1
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=8
            local.set 1
          end
          local.get 2
          local.get 0
          i32.store offset=8
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 0
        local.get 0
        i32.store offset=1049412
        i32.const 0
        i32.const 0
        i32.load offset=1049404
        local.get 1
        i32.add
        local.tee 1
        i32.store offset=1049404
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.load offset=1049408
        i32.ne
        br_if 1 (;@1;)
        i32.const 0
        i32.const 0
        i32.store offset=1049400
        i32.const 0
        i32.const 0
        i32.store offset=1049408
        return
      end
      i32.const 0
      local.get 0
      i32.store offset=1049408
      i32.const 0
      i32.const 0
      i32.load offset=1049400
      local.get 1
      i32.add
      local.tee 1
      i32.store offset=1049400
      local.get 0
      local.get 1
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 0
      local.get 1
      i32.add
      local.get 1
      i32.store
      return
    end
  )
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hed8b206a09582210E (;6;) (type 2) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    i32.const 245
                    i32.lt_u
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 1
                    local.get 0
                    i32.const -65587
                    i32.ge_u
                    br_if 5 (;@3;)
                    local.get 0
                    i32.const 11
                    i32.add
                    local.tee 1
                    i32.const -8
                    i32.and
                    local.set 2
                    i32.const 0
                    i32.load offset=1049396
                    local.tee 3
                    i32.eqz
                    br_if 4 (;@4;)
                    i32.const 31
                    local.set 4
                    block ;; label = @9
                      local.get 0
                      i32.const 16777204
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 6
                      local.get 1
                      i32.const 8
                      i32.shr_u
                      i32.clz
                      local.tee 0
                      i32.sub
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 0
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                      local.set 4
                    end
                    i32.const 0
                    local.get 2
                    i32.sub
                    local.set 1
                    block ;; label = @9
                      local.get 4
                      i32.const 2
                      i32.shl
                      i32.const 1048984
                      i32.add
                      i32.load
                      local.tee 5
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 0
                      i32.const 0
                      local.set 6
                      br 2 (;@7;)
                    end
                    i32.const 0
                    local.set 0
                    local.get 2
                    i32.const 0
                    i32.const 25
                    local.get 4
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    local.get 4
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 7
                    i32.const 0
                    local.set 6
                    loop ;; label = @9
                      block ;; label = @10
                        local.get 5
                        local.tee 5
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.tee 8
                        local.get 2
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 8
                        local.get 2
                        i32.sub
                        local.tee 8
                        local.get 1
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 8
                        local.set 1
                        local.get 5
                        local.set 6
                        local.get 8
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 1
                        local.get 5
                        local.set 6
                        local.get 5
                        local.set 0
                        br 4 (;@6;)
                      end
                      local.get 5
                      i32.load offset=20
                      local.tee 8
                      local.get 0
                      local.get 8
                      local.get 5
                      local.get 7
                      i32.const 29
                      i32.shr_u
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      i32.load
                      local.tee 5
                      i32.ne
                      select
                      local.get 0
                      local.get 8
                      select
                      local.set 0
                      local.get 7
                      i32.const 1
                      i32.shl
                      local.set 7
                      local.get 5
                      i32.eqz
                      br_if 2 (;@7;)
                      br 0 (;@9;)
                    end
                  end
                  block ;; label = @8
                    i32.const 0
                    i32.load offset=1049392
                    local.tee 5
                    i32.const 16
                    local.get 0
                    i32.const 11
                    i32.add
                    i32.const 504
                    i32.and
                    local.get 0
                    i32.const 11
                    i32.lt_u
                    select
                    local.tee 2
                    i32.const 3
                    i32.shr_u
                    local.tee 1
                    i32.shr_u
                    local.tee 0
                    i32.const 3
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        i32.const -1
                        i32.xor
                        i32.const 1
                        i32.and
                        local.get 1
                        i32.add
                        local.tee 7
                        i32.const 3
                        i32.shl
                        local.tee 0
                        i32.const 1049128
                        i32.add
                        local.tee 1
                        local.get 0
                        i32.const 1049136
                        i32.add
                        i32.load
                        local.tee 2
                        i32.load offset=8
                        local.tee 6
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 1
                        i32.store offset=12
                        local.get 1
                        local.get 6
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 5
                      i32.const -2
                      local.get 7
                      i32.rotl
                      i32.and
                      i32.store offset=1049392
                    end
                    local.get 2
                    local.get 0
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 2
                    local.get 0
                    i32.add
                    local.tee 0
                    local.get 0
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 2
                    i32.const 8
                    i32.add
                    return
                  end
                  local.get 2
                  i32.const 0
                  i32.load offset=1049400
                  i32.le_u
                  br_if 3 (;@4;)
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.load offset=1049396
                        local.tee 0
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 0
                        i32.ctz
                        i32.const 2
                        i32.shl
                        i32.const 1048984
                        i32.add
                        i32.load
                        local.tee 6
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 2
                        i32.sub
                        local.set 1
                        local.get 6
                        local.set 5
                        loop ;; label = @11
                          block ;; label = @12
                            local.get 6
                            i32.load offset=16
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 6
                            i32.load offset=20
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 5
                            i32.load offset=24
                            local.set 4
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 5
                                  i32.load offset=12
                                  local.tee 0
                                  local.get 5
                                  i32.ne
                                  br_if 0 (;@15;)
                                  local.get 5
                                  i32.const 20
                                  i32.const 16
                                  local.get 5
                                  i32.load offset=20
                                  local.tee 0
                                  select
                                  i32.add
                                  i32.load
                                  local.tee 6
                                  br_if 1 (;@14;)
                                  i32.const 0
                                  local.set 0
                                  br 2 (;@13;)
                                end
                                local.get 5
                                i32.load offset=8
                                local.tee 6
                                local.get 0
                                i32.store offset=12
                                local.get 0
                                local.get 6
                                i32.store offset=8
                                br 1 (;@13;)
                              end
                              local.get 5
                              i32.const 20
                              i32.add
                              local.get 5
                              i32.const 16
                              i32.add
                              local.get 0
                              select
                              local.set 7
                              loop ;; label = @14
                                local.get 7
                                local.set 8
                                local.get 6
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.get 0
                                i32.const 16
                                i32.add
                                local.get 0
                                i32.load offset=20
                                local.tee 6
                                select
                                local.set 7
                                local.get 0
                                i32.const 20
                                i32.const 16
                                local.get 6
                                select
                                i32.add
                                i32.load
                                local.tee 6
                                br_if 0 (;@14;)
                              end
                              local.get 8
                              i32.const 0
                              i32.store
                            end
                            local.get 4
                            i32.eqz
                            br_if 4 (;@8;)
                            block ;; label = @13
                              local.get 5
                              i32.load offset=28
                              i32.const 2
                              i32.shl
                              i32.const 1048984
                              i32.add
                              local.tee 6
                              i32.load
                              local.get 5
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 4
                              i32.const 16
                              i32.const 20
                              local.get 4
                              i32.load offset=16
                              local.get 5
                              i32.eq
                              select
                              i32.add
                              local.get 0
                              i32.store
                              local.get 0
                              i32.eqz
                              br_if 5 (;@8;)
                              br 4 (;@9;)
                            end
                            local.get 6
                            local.get 0
                            i32.store
                            local.get 0
                            br_if 3 (;@9;)
                            i32.const 0
                            i32.const 0
                            i32.load offset=1049396
                            i32.const -2
                            local.get 5
                            i32.load offset=28
                            i32.rotl
                            i32.and
                            i32.store offset=1049396
                            br 4 (;@8;)
                          end
                          local.get 0
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 2
                          i32.sub
                          local.tee 6
                          local.get 1
                          local.get 6
                          local.get 1
                          i32.lt_u
                          local.tee 6
                          select
                          local.set 1
                          local.get 0
                          local.get 5
                          local.get 6
                          select
                          local.set 5
                          local.get 0
                          local.set 6
                          br 0 (;@11;)
                        end
                      end
                      block ;; label = @10
                        block ;; label = @11
                          local.get 0
                          local.get 1
                          i32.shl
                          i32.const 2
                          local.get 1
                          i32.shl
                          local.tee 0
                          i32.const 0
                          local.get 0
                          i32.sub
                          i32.or
                          i32.and
                          i32.ctz
                          local.tee 8
                          i32.const 3
                          i32.shl
                          local.tee 1
                          i32.const 1049128
                          i32.add
                          local.tee 6
                          local.get 1
                          i32.const 1049136
                          i32.add
                          i32.load
                          local.tee 0
                          i32.load offset=8
                          local.tee 7
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 7
                          local.get 6
                          i32.store offset=12
                          local.get 6
                          local.get 7
                          i32.store offset=8
                          br 1 (;@10;)
                        end
                        i32.const 0
                        local.get 5
                        i32.const -2
                        local.get 8
                        i32.rotl
                        i32.and
                        i32.store offset=1049392
                      end
                      local.get 0
                      local.get 2
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 2
                      i32.add
                      local.tee 7
                      local.get 1
                      local.get 2
                      i32.sub
                      local.tee 6
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 1
                      i32.add
                      local.get 6
                      i32.store
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1049400
                        local.tee 5
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 5
                        i32.const -8
                        i32.and
                        i32.const 1049128
                        i32.add
                        local.set 1
                        i32.const 0
                        i32.load offset=1049408
                        local.set 2
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1049392
                            local.tee 8
                            i32.const 1
                            local.get 5
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 5
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 8
                            local.get 5
                            i32.or
                            i32.store offset=1049392
                            local.get 1
                            local.set 5
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.load offset=8
                          local.set 5
                        end
                        local.get 1
                        local.get 2
                        i32.store offset=8
                        local.get 5
                        local.get 2
                        i32.store offset=12
                        local.get 2
                        local.get 1
                        i32.store offset=12
                        local.get 2
                        local.get 5
                        i32.store offset=8
                      end
                      i32.const 0
                      local.get 7
                      i32.store offset=1049408
                      i32.const 0
                      local.get 6
                      i32.store offset=1049400
                      local.get 0
                      i32.const 8
                      i32.add
                      return
                    end
                    local.get 0
                    local.get 4
                    i32.store offset=24
                    block ;; label = @9
                      local.get 5
                      i32.load offset=16
                      local.tee 6
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 6
                      i32.store offset=16
                      local.get 6
                      local.get 0
                      i32.store offset=24
                    end
                    local.get 5
                    i32.load offset=20
                    local.tee 6
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 6
                    i32.store offset=20
                    local.get 6
                    local.get 0
                    i32.store offset=24
                  end
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 1
                        i32.const 16
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 5
                        local.get 2
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 5
                        local.get 2
                        i32.add
                        local.tee 2
                        local.get 1
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 2
                        local.get 1
                        i32.add
                        local.get 1
                        i32.store
                        i32.const 0
                        i32.load offset=1049400
                        local.tee 7
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 7
                        i32.const -8
                        i32.and
                        i32.const 1049128
                        i32.add
                        local.set 6
                        i32.const 0
                        i32.load offset=1049408
                        local.set 0
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1049392
                            local.tee 8
                            i32.const 1
                            local.get 7
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 7
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 8
                            local.get 7
                            i32.or
                            i32.store offset=1049392
                            local.get 6
                            local.set 7
                            br 1 (;@11;)
                          end
                          local.get 6
                          i32.load offset=8
                          local.set 7
                        end
                        local.get 6
                        local.get 0
                        i32.store offset=8
                        local.get 7
                        local.get 0
                        i32.store offset=12
                        local.get 0
                        local.get 6
                        i32.store offset=12
                        local.get 0
                        local.get 7
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      local.get 5
                      local.get 1
                      local.get 2
                      i32.add
                      local.tee 0
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 5
                      local.get 0
                      i32.add
                      local.tee 0
                      local.get 0
                      i32.load offset=4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      br 1 (;@8;)
                    end
                    i32.const 0
                    local.get 2
                    i32.store offset=1049408
                    i32.const 0
                    local.get 1
                    i32.store offset=1049400
                  end
                  local.get 5
                  i32.const 8
                  i32.add
                  return
                end
                block ;; label = @7
                  local.get 0
                  local.get 6
                  i32.or
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 6
                  i32.const 2
                  local.get 4
                  i32.shl
                  local.tee 0
                  i32.const 0
                  local.get 0
                  i32.sub
                  i32.or
                  local.get 3
                  i32.and
                  local.tee 0
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.const 1048984
                  i32.add
                  i32.load
                  local.set 0
                end
                local.get 0
                i32.eqz
                br_if 1 (;@5;)
              end
              loop ;; label = @6
                local.get 0
                local.get 6
                local.get 0
                i32.load offset=4
                i32.const -8
                i32.and
                local.tee 5
                local.get 2
                i32.sub
                local.tee 8
                local.get 1
                i32.lt_u
                local.tee 4
                select
                local.set 3
                local.get 5
                local.get 2
                i32.lt_u
                local.set 7
                local.get 8
                local.get 1
                local.get 4
                select
                local.set 8
                block ;; label = @7
                  local.get 0
                  i32.load offset=16
                  local.tee 5
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=20
                  local.set 5
                end
                local.get 6
                local.get 3
                local.get 7
                select
                local.set 6
                local.get 1
                local.get 8
                local.get 7
                select
                local.set 1
                local.get 5
                local.set 0
                local.get 5
                br_if 0 (;@6;)
              end
            end
            local.get 6
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              i32.const 0
              i32.load offset=1049400
              local.tee 0
              local.get 2
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              local.get 0
              local.get 2
              i32.sub
              i32.ge_u
              br_if 1 (;@4;)
            end
            local.get 6
            i32.load offset=24
            local.set 4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 6
                  i32.load offset=12
                  local.tee 0
                  local.get 6
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 20
                  i32.const 16
                  local.get 6
                  i32.load offset=20
                  local.tee 0
                  select
                  i32.add
                  i32.load
                  local.tee 5
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 0
                  br 2 (;@5;)
                end
                local.get 6
                i32.load offset=8
                local.tee 5
                local.get 0
                i32.store offset=12
                local.get 0
                local.get 5
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 6
              i32.const 20
              i32.add
              local.get 6
              i32.const 16
              i32.add
              local.get 0
              select
              local.set 7
              loop ;; label = @6
                local.get 7
                local.set 8
                local.get 5
                local.tee 0
                i32.const 20
                i32.add
                local.get 0
                i32.const 16
                i32.add
                local.get 0
                i32.load offset=20
                local.tee 5
                select
                local.set 7
                local.get 0
                i32.const 20
                i32.const 16
                local.get 5
                select
                i32.add
                i32.load
                local.tee 5
                br_if 0 (;@6;)
              end
              local.get 8
              i32.const 0
              i32.store
            end
            local.get 4
            i32.eqz
            br_if 3 (;@1;)
            block ;; label = @5
              local.get 6
              i32.load offset=28
              i32.const 2
              i32.shl
              i32.const 1048984
              i32.add
              local.tee 5
              i32.load
              local.get 6
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              i32.const 16
              i32.const 20
              local.get 4
              i32.load offset=16
              local.get 6
              i32.eq
              select
              i32.add
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 4 (;@1;)
              br 3 (;@2;)
            end
            local.get 5
            local.get 0
            i32.store
            local.get 0
            br_if 2 (;@2;)
            i32.const 0
            i32.const 0
            i32.load offset=1049396
            i32.const -2
            local.get 6
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store offset=1049396
            br 3 (;@1;)
          end
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 0
                      i32.load offset=1049400
                      local.tee 0
                      local.get 2
                      i32.ge_u
                      br_if 0 (;@9;)
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1049404
                        local.tee 0
                        local.get 2
                        i32.gt_u
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 1
                        local.get 2
                        i32.const 65583
                        i32.add
                        local.tee 6
                        i32.const 16
                        i32.shr_u
                        memory.grow
                        local.tee 0
                        i32.const -1
                        i32.eq
                        local.tee 7
                        br_if 7 (;@3;)
                        local.get 0
                        i32.const 16
                        i32.shl
                        local.tee 5
                        i32.eqz
                        br_if 7 (;@3;)
                        i32.const 0
                        i32.const 0
                        i32.load offset=1049416
                        i32.const 0
                        local.get 6
                        i32.const -65536
                        i32.and
                        local.get 7
                        select
                        local.tee 8
                        i32.add
                        local.tee 0
                        i32.store offset=1049416
                        i32.const 0
                        i32.const 0
                        i32.load offset=1049420
                        local.tee 1
                        local.get 0
                        local.get 1
                        local.get 0
                        i32.gt_u
                        select
                        i32.store offset=1049420
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1049412
                              local.tee 1
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 1049112
                              local.set 0
                              loop ;; label = @14
                                local.get 0
                                i32.load
                                local.tee 6
                                local.get 0
                                i32.load offset=4
                                local.tee 7
                                i32.add
                                local.get 5
                                i32.eq
                                br_if 2 (;@12;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                                br 3 (;@11;)
                              end
                            end
                            block ;; label = @13
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=1049428
                                local.tee 0
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 0
                                local.get 5
                                i32.le_u
                                br_if 1 (;@13;)
                              end
                              i32.const 0
                              local.get 5
                              i32.store offset=1049428
                            end
                            i32.const 0
                            i32.const 4095
                            i32.store offset=1049432
                            i32.const 0
                            local.get 8
                            i32.store offset=1049116
                            i32.const 0
                            local.get 5
                            i32.store offset=1049112
                            i32.const 0
                            i32.const 1049128
                            i32.store offset=1049140
                            i32.const 0
                            i32.const 1049136
                            i32.store offset=1049148
                            i32.const 0
                            i32.const 1049128
                            i32.store offset=1049136
                            i32.const 0
                            i32.const 1049144
                            i32.store offset=1049156
                            i32.const 0
                            i32.const 1049136
                            i32.store offset=1049144
                            i32.const 0
                            i32.const 1049152
                            i32.store offset=1049164
                            i32.const 0
                            i32.const 1049144
                            i32.store offset=1049152
                            i32.const 0
                            i32.const 1049160
                            i32.store offset=1049172
                            i32.const 0
                            i32.const 1049152
                            i32.store offset=1049160
                            i32.const 0
                            i32.const 1049168
                            i32.store offset=1049180
                            i32.const 0
                            i32.const 1049160
                            i32.store offset=1049168
                            i32.const 0
                            i32.const 1049176
                            i32.store offset=1049188
                            i32.const 0
                            i32.const 1049168
                            i32.store offset=1049176
                            i32.const 0
                            i32.const 1049184
                            i32.store offset=1049196
                            i32.const 0
                            i32.const 1049176
                            i32.store offset=1049184
                            i32.const 0
                            i32.const 0
                            i32.store offset=1049124
                            i32.const 0
                            i32.const 1049192
                            i32.store offset=1049204
                            i32.const 0
                            i32.const 1049184
                            i32.store offset=1049192
                            i32.const 0
                            i32.const 1049192
                            i32.store offset=1049200
                            i32.const 0
                            i32.const 1049200
                            i32.store offset=1049212
                            i32.const 0
                            i32.const 1049200
                            i32.store offset=1049208
                            i32.const 0
                            i32.const 1049208
                            i32.store offset=1049220
                            i32.const 0
                            i32.const 1049208
                            i32.store offset=1049216
                            i32.const 0
                            i32.const 1049216
                            i32.store offset=1049228
                            i32.const 0
                            i32.const 1049216
                            i32.store offset=1049224
                            i32.const 0
                            i32.const 1049224
                            i32.store offset=1049236
                            i32.const 0
                            i32.const 1049224
                            i32.store offset=1049232
                            i32.const 0
                            i32.const 1049232
                            i32.store offset=1049244
                            i32.const 0
                            i32.const 1049232
                            i32.store offset=1049240
                            i32.const 0
                            i32.const 1049240
                            i32.store offset=1049252
                            i32.const 0
                            i32.const 1049240
                            i32.store offset=1049248
                            i32.const 0
                            i32.const 1049248
                            i32.store offset=1049260
                            i32.const 0
                            i32.const 1049248
                            i32.store offset=1049256
                            i32.const 0
                            i32.const 1049256
                            i32.store offset=1049268
                            i32.const 0
                            i32.const 1049264
                            i32.store offset=1049276
                            i32.const 0
                            i32.const 1049256
                            i32.store offset=1049264
                            i32.const 0
                            i32.const 1049272
                            i32.store offset=1049284
                            i32.const 0
                            i32.const 1049264
                            i32.store offset=1049272
                            i32.const 0
                            i32.const 1049280
                            i32.store offset=1049292
                            i32.const 0
                            i32.const 1049272
                            i32.store offset=1049280
                            i32.const 0
                            i32.const 1049288
                            i32.store offset=1049300
                            i32.const 0
                            i32.const 1049280
                            i32.store offset=1049288
                            i32.const 0
                            i32.const 1049296
                            i32.store offset=1049308
                            i32.const 0
                            i32.const 1049288
                            i32.store offset=1049296
                            i32.const 0
                            i32.const 1049304
                            i32.store offset=1049316
                            i32.const 0
                            i32.const 1049296
                            i32.store offset=1049304
                            i32.const 0
                            i32.const 1049312
                            i32.store offset=1049324
                            i32.const 0
                            i32.const 1049304
                            i32.store offset=1049312
                            i32.const 0
                            i32.const 1049320
                            i32.store offset=1049332
                            i32.const 0
                            i32.const 1049312
                            i32.store offset=1049320
                            i32.const 0
                            i32.const 1049328
                            i32.store offset=1049340
                            i32.const 0
                            i32.const 1049320
                            i32.store offset=1049328
                            i32.const 0
                            i32.const 1049336
                            i32.store offset=1049348
                            i32.const 0
                            i32.const 1049328
                            i32.store offset=1049336
                            i32.const 0
                            i32.const 1049344
                            i32.store offset=1049356
                            i32.const 0
                            i32.const 1049336
                            i32.store offset=1049344
                            i32.const 0
                            i32.const 1049352
                            i32.store offset=1049364
                            i32.const 0
                            i32.const 1049344
                            i32.store offset=1049352
                            i32.const 0
                            i32.const 1049360
                            i32.store offset=1049372
                            i32.const 0
                            i32.const 1049352
                            i32.store offset=1049360
                            i32.const 0
                            i32.const 1049368
                            i32.store offset=1049380
                            i32.const 0
                            i32.const 1049360
                            i32.store offset=1049368
                            i32.const 0
                            i32.const 1049376
                            i32.store offset=1049388
                            i32.const 0
                            i32.const 1049368
                            i32.store offset=1049376
                            i32.const 0
                            local.get 5
                            i32.store offset=1049412
                            i32.const 0
                            i32.const 1049376
                            i32.store offset=1049384
                            i32.const 0
                            local.get 8
                            i32.const -40
                            i32.add
                            local.tee 0
                            i32.store offset=1049404
                            local.get 5
                            local.get 0
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            local.get 5
                            local.get 0
                            i32.add
                            i32.const 40
                            i32.store offset=4
                            i32.const 0
                            i32.const 2097152
                            i32.store offset=1049424
                            br 8 (;@4;)
                          end
                          local.get 1
                          local.get 5
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 6
                          local.get 1
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 0
                          i32.load offset=12
                          i32.eqz
                          br_if 3 (;@8;)
                        end
                        i32.const 0
                        i32.const 0
                        i32.load offset=1049428
                        local.tee 0
                        local.get 5
                        local.get 0
                        local.get 5
                        i32.lt_u
                        select
                        i32.store offset=1049428
                        local.get 5
                        local.get 8
                        i32.add
                        local.set 6
                        i32.const 1049112
                        local.set 0
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              loop ;; label = @14
                                local.get 0
                                i32.load
                                local.tee 7
                                local.get 6
                                i32.eq
                                br_if 1 (;@13;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                                br 2 (;@12;)
                              end
                            end
                            local.get 0
                            i32.load offset=12
                            i32.eqz
                            br_if 1 (;@11;)
                          end
                          i32.const 1049112
                          local.set 0
                          block ;; label = @12
                            loop ;; label = @13
                              block ;; label = @14
                                local.get 0
                                i32.load
                                local.tee 6
                                local.get 1
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 1
                                local.get 6
                                local.get 0
                                i32.load offset=4
                                i32.add
                                local.tee 6
                                i32.lt_u
                                br_if 2 (;@12;)
                              end
                              local.get 0
                              i32.load offset=8
                              local.set 0
                              br 0 (;@13;)
                            end
                          end
                          i32.const 0
                          local.get 5
                          i32.store offset=1049412
                          i32.const 0
                          local.get 8
                          i32.const -40
                          i32.add
                          local.tee 0
                          i32.store offset=1049404
                          local.get 5
                          local.get 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 5
                          local.get 0
                          i32.add
                          i32.const 40
                          i32.store offset=4
                          i32.const 0
                          i32.const 2097152
                          i32.store offset=1049424
                          local.get 1
                          local.get 6
                          i32.const -32
                          i32.add
                          i32.const -8
                          i32.and
                          i32.const -8
                          i32.add
                          local.tee 0
                          local.get 0
                          local.get 1
                          i32.const 16
                          i32.add
                          i32.lt_u
                          select
                          local.tee 7
                          i32.const 27
                          i32.store offset=4
                          i32.const 0
                          i64.load offset=1049112 align=4
                          local.set 9
                          local.get 7
                          i32.const 16
                          i32.add
                          i32.const 0
                          i64.load offset=1049120 align=4
                          i64.store align=4
                          local.get 7
                          local.get 9
                          i64.store offset=8 align=4
                          i32.const 0
                          local.get 8
                          i32.store offset=1049116
                          i32.const 0
                          local.get 5
                          i32.store offset=1049112
                          i32.const 0
                          local.get 7
                          i32.const 8
                          i32.add
                          i32.store offset=1049120
                          i32.const 0
                          i32.const 0
                          i32.store offset=1049124
                          local.get 7
                          i32.const 28
                          i32.add
                          local.set 0
                          loop ;; label = @12
                            local.get 0
                            i32.const 7
                            i32.store
                            local.get 0
                            i32.const 4
                            i32.add
                            local.tee 0
                            local.get 6
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                          local.get 7
                          local.get 1
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 7
                          local.get 7
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                          local.get 1
                          local.get 7
                          local.get 1
                          i32.sub
                          local.tee 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 7
                          local.get 0
                          i32.store
                          block ;; label = @12
                            local.get 0
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 1
                            local.get 0
                            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h552ba975f7708a56E
                            br 8 (;@4;)
                          end
                          local.get 0
                          i32.const 248
                          i32.and
                          i32.const 1049128
                          i32.add
                          local.set 6
                          block ;; label = @12
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1049392
                              local.tee 5
                              i32.const 1
                              local.get 0
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 0
                              i32.and
                              br_if 0 (;@13;)
                              i32.const 0
                              local.get 5
                              local.get 0
                              i32.or
                              i32.store offset=1049392
                              local.get 6
                              local.set 0
                              br 1 (;@12;)
                            end
                            local.get 6
                            i32.load offset=8
                            local.set 0
                          end
                          local.get 6
                          local.get 1
                          i32.store offset=8
                          local.get 0
                          local.get 1
                          i32.store offset=12
                          local.get 1
                          local.get 6
                          i32.store offset=12
                          local.get 1
                          local.get 0
                          i32.store offset=8
                          br 7 (;@4;)
                        end
                        local.get 0
                        local.get 5
                        i32.store
                        local.get 0
                        local.get 0
                        i32.load offset=4
                        local.get 8
                        i32.add
                        i32.store offset=4
                        local.get 5
                        local.get 2
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 7
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        i32.const -8
                        i32.add
                        local.tee 1
                        local.get 5
                        local.get 2
                        i32.add
                        local.tee 0
                        i32.sub
                        local.set 2
                        local.get 1
                        i32.const 0
                        i32.load offset=1049412
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 1
                        i32.const 0
                        i32.load offset=1049408
                        i32.eq
                        br_if 4 (;@6;)
                        block ;; label = @11
                          local.get 1
                          i32.load offset=4
                          local.tee 6
                          i32.const 3
                          i32.and
                          i32.const 1
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 1
                          local.get 6
                          i32.const -8
                          i32.and
                          local.tee 6
                          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h7dc40cdafd0f02adE
                          local.get 6
                          local.get 2
                          i32.add
                          local.set 2
                          local.get 1
                          local.get 6
                          i32.add
                          local.tee 1
                          i32.load offset=4
                          local.set 6
                        end
                        local.get 1
                        local.get 6
                        i32.const -2
                        i32.and
                        i32.store offset=4
                        local.get 0
                        local.get 2
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 0
                        local.get 2
                        i32.add
                        local.get 2
                        i32.store
                        block ;; label = @11
                          local.get 2
                          i32.const 256
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 2
                          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h552ba975f7708a56E
                          br 6 (;@5;)
                        end
                        local.get 2
                        i32.const 248
                        i32.and
                        i32.const 1049128
                        i32.add
                        local.set 1
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1049392
                            local.tee 6
                            i32.const 1
                            local.get 2
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 2
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 6
                            local.get 2
                            i32.or
                            i32.store offset=1049392
                            local.get 1
                            local.set 2
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.load offset=8
                          local.set 2
                        end
                        local.get 1
                        local.get 0
                        i32.store offset=8
                        local.get 2
                        local.get 0
                        i32.store offset=12
                        local.get 0
                        local.get 1
                        i32.store offset=12
                        local.get 0
                        local.get 2
                        i32.store offset=8
                        br 5 (;@5;)
                      end
                      i32.const 0
                      local.get 0
                      local.get 2
                      i32.sub
                      local.tee 1
                      i32.store offset=1049404
                      i32.const 0
                      i32.const 0
                      i32.load offset=1049412
                      local.tee 0
                      local.get 2
                      i32.add
                      local.tee 6
                      i32.store offset=1049412
                      local.get 6
                      local.get 1
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 2
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 1
                      br 6 (;@3;)
                    end
                    i32.const 0
                    i32.load offset=1049408
                    local.set 1
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        local.get 2
                        i32.sub
                        local.tee 6
                        i32.const 15
                        i32.gt_u
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.const 0
                        i32.store offset=1049408
                        i32.const 0
                        i32.const 0
                        i32.store offset=1049400
                        local.get 1
                        local.get 0
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 1
                        local.get 0
                        i32.add
                        local.tee 0
                        local.get 0
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 6
                      i32.store offset=1049400
                      i32.const 0
                      local.get 1
                      local.get 2
                      i32.add
                      local.tee 5
                      i32.store offset=1049408
                      local.get 5
                      local.get 6
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 0
                      i32.add
                      local.get 6
                      i32.store
                      local.get 1
                      local.get 2
                      i32.const 3
                      i32.or
                      i32.store offset=4
                    end
                    local.get 1
                    i32.const 8
                    i32.add
                    return
                  end
                  local.get 0
                  local.get 7
                  local.get 8
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049412
                  local.tee 0
                  i32.const 15
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 1
                  i32.const -8
                  i32.add
                  local.tee 6
                  i32.store offset=1049412
                  i32.const 0
                  local.get 0
                  local.get 1
                  i32.sub
                  i32.const 0
                  i32.load offset=1049404
                  local.get 8
                  i32.add
                  local.tee 1
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 5
                  i32.store offset=1049404
                  local.get 6
                  local.get 5
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 1
                  i32.add
                  i32.const 40
                  i32.store offset=4
                  i32.const 0
                  i32.const 2097152
                  i32.store offset=1049424
                  br 3 (;@4;)
                end
                i32.const 0
                local.get 0
                i32.store offset=1049412
                i32.const 0
                i32.const 0
                i32.load offset=1049404
                local.get 2
                i32.add
                local.tee 2
                i32.store offset=1049404
                local.get 0
                local.get 2
                i32.const 1
                i32.or
                i32.store offset=4
                br 1 (;@5;)
              end
              i32.const 0
              local.get 0
              i32.store offset=1049408
              i32.const 0
              i32.const 0
              i32.load offset=1049400
              local.get 2
              i32.add
              local.tee 2
              i32.store offset=1049400
              local.get 0
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 2
              i32.add
              local.get 2
              i32.store
            end
            local.get 5
            i32.const 8
            i32.add
            return
          end
          i32.const 0
          local.set 1
          i32.const 0
          i32.load offset=1049404
          local.tee 0
          local.get 2
          i32.le_u
          br_if 0 (;@3;)
          i32.const 0
          local.get 0
          local.get 2
          i32.sub
          local.tee 1
          i32.store offset=1049404
          i32.const 0
          i32.const 0
          i32.load offset=1049412
          local.tee 0
          local.get 2
          i32.add
          local.tee 6
          i32.store offset=1049412
          local.get 6
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 2
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          return
        end
        local.get 1
        return
      end
      local.get 0
      local.get 4
      i32.store offset=24
      block ;; label = @2
        local.get 6
        i32.load offset=16
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 5
        i32.store offset=16
        local.get 5
        local.get 0
        i32.store offset=24
      end
      local.get 6
      i32.load offset=20
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 5
      i32.store offset=20
      local.get 5
      local.get 0
      i32.store offset=24
    end
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 16
        i32.lt_u
        br_if 0 (;@2;)
        local.get 6
        local.get 2
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 6
        local.get 2
        i32.add
        local.tee 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 1
        i32.add
        local.get 1
        i32.store
        block ;; label = @3
          local.get 1
          i32.const 256
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h552ba975f7708a56E
          br 2 (;@1;)
        end
        local.get 1
        i32.const 248
        i32.and
        i32.const 1049128
        i32.add
        local.set 2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1049392
            local.tee 5
            i32.const 1
            local.get 1
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 1
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 5
            local.get 1
            i32.or
            i32.store offset=1049392
            local.get 2
            local.set 1
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 1
        end
        local.get 2
        local.get 0
        i32.store offset=8
        local.get 1
        local.get 0
        i32.store offset=12
        local.get 0
        local.get 2
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 6
      local.get 1
      local.get 2
      i32.add
      local.tee 0
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 6
      local.get 0
      i32.add
      local.tee 0
      local.get 0
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
    end
    local.get 6
    i32.const 8
    i32.add
  )
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17hd371afab6b2bc9caE (;7;) (type 1) (param i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.const -8
    i32.add
    local.tee 1
    local.get 0
    i32.const -4
    i32.add
    i32.load
    local.tee 2
    i32.const -8
    i32.and
    local.tee 0
    i32.add
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.load
        local.tee 2
        local.get 0
        i32.add
        local.set 0
        block ;; label = @3
          local.get 1
          local.get 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.load offset=1049408
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1049400
          local.get 3
          local.get 3
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 3
          local.get 0
          i32.store
          return
        end
        local.get 1
        local.get 2
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h7dc40cdafd0f02adE
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 3
                      i32.load offset=4
                      local.tee 2
                      i32.const 2
                      i32.and
                      br_if 0 (;@9;)
                      local.get 3
                      i32.const 0
                      i32.load offset=1049412
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 3
                      i32.const 0
                      i32.load offset=1049408
                      i32.eq
                      br_if 3 (;@6;)
                      local.get 3
                      local.get 2
                      i32.const -8
                      i32.and
                      local.tee 2
                      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h7dc40cdafd0f02adE
                      local.get 1
                      local.get 2
                      local.get 0
                      i32.add
                      local.tee 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 0
                      i32.add
                      local.get 0
                      i32.store
                      local.get 1
                      i32.const 0
                      i32.load offset=1049408
                      i32.ne
                      br_if 1 (;@8;)
                      i32.const 0
                      local.get 0
                      i32.store offset=1049400
                      return
                    end
                    local.get 3
                    local.get 2
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 1
                    local.get 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 1
                    local.get 0
                    i32.add
                    local.get 0
                    i32.store
                  end
                  local.get 0
                  i32.const 256
                  i32.lt_u
                  br_if 2 (;@5;)
                  i32.const 31
                  local.set 3
                  block ;; label = @8
                    local.get 0
                    i32.const 16777215
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 6
                    local.get 0
                    i32.const 8
                    i32.shr_u
                    i32.clz
                    local.tee 3
                    i32.sub
                    i32.shr_u
                    i32.const 1
                    i32.and
                    local.get 3
                    i32.const 1
                    i32.shl
                    i32.sub
                    i32.const 62
                    i32.add
                    local.set 3
                  end
                  local.get 1
                  i64.const 0
                  i64.store offset=16 align=4
                  local.get 1
                  local.get 3
                  i32.store offset=28
                  local.get 3
                  i32.const 2
                  i32.shl
                  i32.const 1048984
                  i32.add
                  local.set 2
                  i32.const 0
                  i32.load offset=1049396
                  i32.const 1
                  local.get 3
                  i32.shl
                  local.tee 4
                  i32.and
                  br_if 3 (;@4;)
                  local.get 2
                  local.get 1
                  i32.store
                  local.get 1
                  local.get 2
                  i32.store offset=24
                  local.get 1
                  local.get 1
                  i32.store offset=12
                  local.get 1
                  local.get 1
                  i32.store offset=8
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049396
                  local.get 4
                  i32.or
                  i32.store offset=1049396
                  br 4 (;@3;)
                end
                i32.const 0
                local.get 1
                i32.store offset=1049412
                i32.const 0
                i32.const 0
                i32.load offset=1049404
                local.get 0
                i32.add
                local.tee 0
                i32.store offset=1049404
                local.get 1
                local.get 0
                i32.const 1
                i32.or
                i32.store offset=4
                block ;; label = @7
                  local.get 1
                  i32.const 0
                  i32.load offset=1049408
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 0
                  i32.store offset=1049400
                  i32.const 0
                  i32.const 0
                  i32.store offset=1049408
                end
                local.get 0
                i32.const 0
                i32.load offset=1049424
                local.tee 4
                i32.le_u
                br_if 5 (;@1;)
                i32.const 0
                i32.load offset=1049412
                local.tee 0
                i32.eqz
                br_if 5 (;@1;)
                i32.const 0
                local.set 2
                i32.const 0
                i32.load offset=1049404
                local.tee 5
                i32.const 41
                i32.lt_u
                br_if 4 (;@2;)
                i32.const 1049112
                local.set 1
                loop ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load
                    local.tee 3
                    local.get 0
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 3
                    local.get 1
                    i32.load offset=4
                    i32.add
                    i32.lt_u
                    br_if 6 (;@2;)
                  end
                  local.get 1
                  i32.load offset=8
                  local.set 1
                  br 0 (;@7;)
                end
              end
              i32.const 0
              local.get 1
              i32.store offset=1049408
              i32.const 0
              i32.const 0
              i32.load offset=1049400
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1049400
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 0
              i32.add
              local.get 0
              i32.store
              return
            end
            local.get 0
            i32.const 248
            i32.and
            i32.const 1049128
            i32.add
            local.set 3
            block ;; label = @5
              block ;; label = @6
                i32.const 0
                i32.load offset=1049392
                local.tee 2
                i32.const 1
                local.get 0
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 0
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 2
                local.get 0
                i32.or
                i32.store offset=1049392
                local.get 3
                local.set 0
                br 1 (;@5;)
              end
              local.get 3
              i32.load offset=8
              local.set 0
            end
            local.get 3
            local.get 1
            i32.store offset=8
            local.get 0
            local.get 1
            i32.store offset=12
            local.get 1
            local.get 3
            i32.store offset=12
            local.get 1
            local.get 0
            i32.store offset=8
            return
          end
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 2
                i32.load
                local.tee 4
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 0
                i32.ne
                br_if 0 (;@6;)
                local.get 4
                local.set 3
                br 1 (;@5;)
              end
              local.get 0
              i32.const 0
              i32.const 25
              local.get 3
              i32.const 1
              i32.shr_u
              i32.sub
              local.get 3
              i32.const 31
              i32.eq
              select
              i32.shl
              local.set 2
              loop ;; label = @6
                local.get 4
                local.get 2
                i32.const 29
                i32.shr_u
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee 5
                i32.load
                local.tee 3
                i32.eqz
                br_if 2 (;@4;)
                local.get 2
                i32.const 1
                i32.shl
                local.set 2
                local.get 3
                local.set 4
                local.get 3
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 0
                i32.ne
                br_if 0 (;@6;)
              end
            end
            local.get 3
            i32.load offset=8
            local.tee 0
            local.get 1
            i32.store offset=12
            local.get 3
            local.get 1
            i32.store offset=8
            local.get 1
            i32.const 0
            i32.store offset=24
            local.get 1
            local.get 3
            i32.store offset=12
            local.get 1
            local.get 0
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 5
          local.get 1
          i32.store
          local.get 1
          local.get 4
          i32.store offset=24
          local.get 1
          local.get 1
          i32.store offset=12
          local.get 1
          local.get 1
          i32.store offset=8
        end
        i32.const 0
        local.set 1
        i32.const 0
        i32.const 0
        i32.load offset=1049432
        i32.const -1
        i32.add
        local.tee 0
        i32.store offset=1049432
        local.get 0
        br_if 1 (;@1;)
        block ;; label = @3
          i32.const 0
          i32.load offset=1049120
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          loop ;; label = @4
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 0
            i32.load offset=8
            local.tee 0
            br_if 0 (;@4;)
          end
        end
        i32.const 0
        local.get 1
        i32.const 4095
        local.get 1
        i32.const 4095
        i32.gt_u
        select
        i32.store offset=1049432
        return
      end
      block ;; label = @2
        i32.const 0
        i32.load offset=1049120
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        loop ;; label = @3
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 1
          i32.load offset=8
          local.tee 1
          br_if 0 (;@3;)
        end
      end
      i32.const 0
      local.get 2
      i32.const 4095
      local.get 2
      i32.const 4095
      i32.gt_u
      select
      i32.store offset=1049432
      local.get 5
      local.get 4
      i32.le_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const -1
      i32.store offset=1049424
    end
  )
  (func $_ZN4core9panicking5panic17hfef8090705073b49E (;8;) (type 0) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=16
    local.get 2
    i32.const 1
    i32.store offset=4
    local.get 2
    i64.const 4
    i64.store offset=8 align=4
    local.get 2
    i32.const 46
    i32.store offset=28
    local.get 2
    local.get 0
    i32.store offset=24
    local.get 2
    local.get 2
    i32.const 24
    i32.add
    i32.store
    local.get 2
    local.get 1
    call $_ZN4core9panicking9panic_fmt17hf09e831ea9f1651aE
    unreachable
  )
  (func $_ZN4core9panicking9panic_fmt17hf09e831ea9f1651aE (;9;) (type 0) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1
    i32.store16 offset=12
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 4
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $rust_begin_unwind (;10;) (type 1) (param i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i64.load align=4
    local.set 2
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    local.get 2
    i64.store offset=4 align=4
    local.get 1
    i32.const 4
    i32.add
    call $_ZN3std3sys9backtrace26__rust_end_short_backtrace17h9dcf082627c76851E
    unreachable
  )
  (func $rust_panic (;11;) (type 5)
    unreachable
  )
  (func $_ZN4core5panic12PanicPayload6as_str17h201ff5ad12170af0E (;12;) (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
  )
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h552ba975f7708a56E (;13;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    i32.const 31
    local.set 2
    block ;; label = @1
      local.get 1
      i32.const 16777215
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 6
      local.get 1
      i32.const 8
      i32.shr_u
      i32.clz
      local.tee 2
      i32.sub
      i32.shr_u
      i32.const 1
      i32.and
      local.get 2
      i32.const 1
      i32.shl
      i32.sub
      i32.const 62
      i32.add
      local.set 2
    end
    local.get 0
    i64.const 0
    i64.store offset=16 align=4
    local.get 0
    local.get 2
    i32.store offset=28
    local.get 2
    i32.const 2
    i32.shl
    i32.const 1048984
    i32.add
    local.set 3
    block ;; label = @1
      i32.const 0
      i32.load offset=1049396
      i32.const 1
      local.get 2
      i32.shl
      local.tee 4
      i32.and
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      i32.store
      local.get 0
      local.get 3
      i32.store offset=24
      local.get 0
      local.get 0
      i32.store offset=12
      local.get 0
      local.get 0
      i32.store offset=8
      i32.const 0
      i32.const 0
      i32.load offset=1049396
      local.get 4
      i32.or
      i32.store offset=1049396
      return
    end
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.load
          local.tee 4
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 4
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 3
        loop ;; label = @3
          local.get 4
          local.get 3
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 5
          i32.load
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 2
          local.set 4
          local.get 2
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.load offset=8
      local.tee 3
      local.get 0
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 0
      local.get 3
      i32.store offset=8
      return
    end
    local.get 5
    local.get 0
    i32.store
    local.get 0
    local.get 4
    i32.store offset=24
    local.get 0
    local.get 0
    i32.store offset=12
    local.get 0
    local.get 0
    i32.store offset=8
  )
  (func $_ZN3std3sys9backtrace26__rust_end_short_backtrace17h9dcf082627c76851E (;14;) (type 1) (param i32)
    local.get 0
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h9b4576c502f796bbE
    unreachable
  )
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h9b4576c502f796bbE (;15;) (type 1) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.tee 2
    i32.load offset=12
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.load offset=4
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;)
          end
          local.get 3
          br_if 1 (;@2;)
          i32.const 1
          local.set 2
          i32.const 0
          local.set 3
          br 2 (;@1;)
        end
        local.get 3
        br_if 0 (;@2;)
        local.get 2
        i32.load
        local.tee 2
        i32.load offset=4
        local.set 3
        local.get 2
        i32.load
        local.set 2
        br 1 (;@1;)
      end
      local.get 1
      i32.const -2147483648
      i32.store
      local.get 1
      local.get 0
      i32.store offset=12
      local.get 1
      i32.const 1
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load8_u offset=8
      local.get 0
      i32.load8_u offset=9
      call $_ZN3std9panicking20rust_panic_with_hook17he163a328f096b027E
      unreachable
    end
    local.get 1
    local.get 3
    i32.store offset=4
    local.get 1
    local.get 2
    i32.store
    local.get 1
    i32.const 2
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.load8_u offset=8
    local.get 0
    i32.load8_u offset=9
    call $_ZN3std9panicking20rust_panic_with_hook17he163a328f096b027E
    unreachable
  )
  (func $_ZN3std9panicking20rust_panic_with_hook17he163a328f096b027E (;16;) (type 6) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    i32.const 0
    i32.const 0
    i32.load offset=1048980
    local.tee 5
    i32.const 1
    i32.add
    i32.store offset=1048980
    block ;; label = @1
      local.get 5
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1049440
          br_if 0 (;@3;)
          i32.const 0
          i32.const 0
          i32.load offset=1049436
          i32.const 1
          i32.add
          i32.store offset=1049436
          i32.const 0
          i32.load offset=1048976
          i32.const -1
          i32.gt_s
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 4
        i32.const 8
        i32.add
        local.get 0
        local.get 1
        call_indirect (type 0)
        unreachable
      end
      i32.const 0
      i32.const 0
      i32.store8 offset=1049440
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      call $rust_panic
      unreachable
    end
    unreachable
  )
  (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$6as_str17hc567c4039abaef48E (;17;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store
  )
  (func $_ZN12wasm_bindgen4__rt21LazyCell$LT$T$C$F$GT$8try_with17ha146a081a81b8b1bE (;18;) (type 1) (param i32)
    (local i32)
    call $_ZN9once_cell6unsync17OnceCell$LT$T$GT$15get_or_try_init17h32f120d39a5cc659E
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 0
        i32.load offset=1048968
        local.tee 1
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.sub
        local.tee 0
        i32.const 0
        i32.load offset=1048960
        i32.lt_u
        br_if 1 (;@1;)
      end
      unreachable
    end
    i32.const 0
    i32.load offset=1048956
    local.get 0
    i32.const 2
    i32.shl
    i32.add
    i32.const 0
    i32.load offset=1048964
    i32.store
    i32.const 0
    local.get 0
    i32.store offset=1048964
  )
  (func $_ZN9once_cell6unsync17OnceCell$LT$T$GT$15get_or_try_init17h32f120d39a5cc659E (;19;) (type 5)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load offset=1048948
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1048972
              local.set 1
              i32.const 0
              i32.const 0
              i32.store offset=1048972
              local.get 1
              i32.eqz
              br_if 1 (;@4;)
              local.get 0
              i32.const 24
              i32.add
              local.get 1
              call_indirect (type 1)
              local.get 0
              i32.const 16
              i32.add
              local.tee 2
              local.get 0
              i32.const 36
              i32.add
              i64.load align=4
              i64.store
              local.get 0
              local.get 0
              i64.load offset=28 align=4
              i64.store offset=8
              local.get 0
              i32.load offset=24
              local.set 3
              i32.const 0
              i32.load offset=1048948
              local.tee 1
              br_if 4 (;@1;)
              block ;; label = @6
                local.get 1
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                i32.load offset=1048952
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                i32.load offset=1048956
                local.tee 4
                i32.const -4
                i32.add
                i32.load
                local.tee 2
                i32.const -8
                i32.and
                local.tee 5
                i32.const 4
                i32.const 8
                local.get 2
                i32.const 3
                i32.and
                local.tee 2
                select
                local.get 1
                i32.const 2
                i32.shl
                local.tee 1
                i32.add
                i32.lt_u
                br_if 3 (;@3;)
                block ;; label = @7
                  local.get 2
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  local.get 1
                  i32.const 39
                  i32.add
                  i32.gt_u
                  br_if 5 (;@2;)
                end
                local.get 4
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17hd371afab6b2bc9caE
              end
              i32.const 0
              local.get 3
              i32.store offset=1048952
              i32.const 0
              i32.const 1
              i32.store offset=1048948
              i32.const 0
              local.get 0
              i64.load offset=8
              i64.store offset=1048956 align=4
              i32.const 0
              local.get 0
              i32.const 16
              i32.add
              i64.load
              i64.store offset=1048964 align=4
            end
            local.get 0
            i32.const 48
            i32.add
            global.set $__stack_pointer
            return
          end
          local.get 0
          i32.const 0
          i32.store offset=40
          local.get 0
          i32.const 1
          i32.store offset=28
          local.get 0
          i32.const 1048788
          i32.store offset=24
          local.get 0
          i64.const 4
          i64.store offset=32 align=4
          local.get 0
          i32.const 24
          i32.add
          i32.const 1048892
          call $_ZN4core9panicking9panic_fmt17hf09e831ea9f1651aE
          unreachable
        end
        i32.const 1048617
        i32.const 1048664
        call $_ZN4core9panicking5panic17hfef8090705073b49E
        unreachable
      end
      i32.const 1048680
      i32.const 1048728
      call $_ZN4core9panicking5panic17hfef8090705073b49E
      unreachable
    end
    local.get 0
    i32.const 40
    i32.add
    local.get 2
    i64.load
    i64.store align=4
    local.get 0
    local.get 0
    i64.load offset=8
    i64.store offset=32 align=4
    local.get 0
    local.get 3
    i32.store offset=28
    local.get 0
    i32.const 1
    i32.store offset=24
    local.get 0
    i32.const 24
    i32.add
    call $_ZN4core3ptr113drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$core..cell..Cell$LT$wasm_bindgen..externref..Slab$GT$$GT$$GT$17hc90deb49e2bf84c8E
    local.get 0
    i32.const 0
    i32.store offset=40
    local.get 0
    i32.const 1
    i32.store offset=28
    local.get 0
    i32.const 1048924
    i32.store offset=24
    local.get 0
    i64.const 4
    i64.store offset=32 align=4
    local.get 0
    i32.const 24
    i32.add
    i32.const 1048932
    call $_ZN4core9panicking9panic_fmt17hf09e831ea9f1651aE
    unreachable
  )
  (func $__wbindgen_malloc (;20;) (type 4) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 1
      i32.popcnt
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      i32.const -2147483648
      local.get 1
      i32.sub
      local.get 0
      i32.lt_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load8_u offset=1049441
        drop
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 9
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.get 0
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17h0c9b7ed4b8acf74dE
            local.set 1
            br 1 (;@3;)
          end
          local.get 0
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hed8b206a09582210E
          local.set 1
        end
        local.get 1
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 1
      return
    end
    unreachable
  )
  (func $__wbindgen_realloc (;21;) (type 3) (param i32 i32 i32 i32) (result i32)
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.popcnt
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        i32.const -2147483648
        local.get 3
        i32.sub
        local.get 1
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        local.get 3
        local.get 2
        call $__rust_realloc
        local.tee 3
        br_if 1 (;@1;)
      end
      unreachable
    end
    local.get 3
  )
  (func $__wbindgen_free (;22;) (type 7) (param i32 i32 i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const -4
          i32.add
          i32.load
          local.tee 3
          i32.const -8
          i32.and
          local.tee 4
          i32.const 4
          i32.const 8
          local.get 3
          i32.const 3
          i32.and
          local.tee 3
          select
          local.get 1
          i32.add
          i32.lt_u
          br_if 1 (;@2;)
          block ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 1
            i32.const 39
            i32.add
            i32.gt_u
            br_if 3 (;@1;)
          end
          local.get 0
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17hd371afab6b2bc9caE
        end
        return
      end
      i32.const 1048617
      i32.const 1048664
      call $_ZN4core9panicking5panic17hfef8090705073b49E
      unreachable
    end
    i32.const 1048680
    i32.const 1048728
    call $_ZN4core9panicking5panic17hfef8090705073b49E
    unreachable
  )
  (func $__wbindgen_exn_store (;23;) (type 1) (param i32))
  (func $_ZN5alloc7raw_vec11finish_grow17h108e4e2a3be76ba7E (;24;) (type 7) (param i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.load offset=4
              i32.eqz
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 2
                i32.load offset=8
                local.tee 3
                br_if 0 (;@6;)
                local.get 1
                i32.eqz
                br_if 4 (;@2;)
                i32.const 0
                i32.load8_u offset=1049441
                drop
                br 2 (;@4;)
              end
              local.get 2
              i32.load
              local.get 3
              i32.const 4
              local.get 1
              call $__rust_realloc
              local.set 2
              br 2 (;@3;)
            end
            local.get 1
            i32.eqz
            br_if 2 (;@2;)
            i32.const 0
            i32.load8_u offset=1049441
            drop
          end
          local.get 1
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hed8b206a09582210E
          local.set 2
        end
        local.get 2
        i32.const 4
        local.get 2
        select
        local.set 3
        local.get 2
        i32.eqz
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 2
      i32.const 4
      local.set 3
    end
    local.get 0
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
  )
  (func $_ZN4core3ptr113drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$core..cell..Cell$LT$wasm_bindgen..externref..Slab$GT$$GT$$GT$17hc90deb49e2bf84c8E (;25;) (type 1) (param i32)
    (local i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=4
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=8
          local.tee 2
          i32.const -4
          i32.add
          i32.load
          local.tee 0
          i32.const -8
          i32.and
          local.tee 3
          i32.const 4
          i32.const 8
          local.get 0
          i32.const 3
          i32.and
          local.tee 0
          select
          local.get 1
          i32.const 2
          i32.shl
          local.tee 1
          i32.add
          i32.lt_u
          br_if 1 (;@2;)
          block ;; label = @4
            local.get 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 1
            i32.const 39
            i32.add
            i32.gt_u
            br_if 3 (;@1;)
          end
          local.get 2
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17hd371afab6b2bc9caE
        end
        return
      end
      i32.const 1048617
      i32.const 1048664
      call $_ZN4core9panicking5panic17hfef8090705073b49E
      unreachable
    end
    i32.const 1048680
    i32.const 1048728
    call $_ZN4core9panicking5panic17hfef8090705073b49E
    unreachable
  )
  (func $__externref_table_alloc (;26;) (type 8) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    call $_ZN9once_cell6unsync17OnceCell$LT$T$GT$15get_or_try_init17h32f120d39a5cc659E
    i32.const 0
    i32.load offset=1048968
    local.set 1
    i32.const 0
    i32.load offset=1048964
    local.set 2
    i32.const 0
    i64.const 0
    i64.store offset=1048964 align=4
    i32.const 0
    i32.load offset=1048956
    local.set 3
    i32.const 0
    i32.load offset=1048960
    local.set 4
    i32.const 0
    i64.const 4
    i64.store offset=1048956 align=4
    i32.const 0
    i32.load offset=1048952
    local.set 5
    i32.const 0
    i32.const 0
    i32.store offset=1048952
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            local.get 4
            i32.ne
            br_if 0 (;@4;)
            block ;; label = @5
              block ;; label = @6
                local.get 4
                local.get 5
                i32.ne
                br_if 0 (;@6;)
                local.get 5
                i32.const 128
                local.get 5
                i32.const 128
                i32.gt_u
                select
                local.tee 6
                call $_ZN12wasm_bindgen9externref31__wbindgen_externref_table_grow17he19b8e5bd557ee78E
                local.tee 7
                i32.const -1
                i32.eq
                br_if 3 (;@3;)
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    br_if 0 (;@8;)
                    local.get 7
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 5
                  local.get 1
                  i32.add
                  local.get 7
                  i32.ne
                  br_if 4 (;@3;)
                end
                local.get 5
                local.get 6
                i32.add
                local.tee 6
                local.get 5
                i32.lt_u
                br_if 3 (;@3;)
                local.get 6
                i32.const 1073741823
                i32.gt_u
                br_if 3 (;@3;)
                local.get 6
                i32.const 2
                i32.shl
                local.tee 7
                i32.const 2147483644
                i32.gt_u
                br_if 3 (;@3;)
                block ;; label = @7
                  block ;; label = @8
                    local.get 5
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 3
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 3
                  i32.store offset=20
                  local.get 0
                  local.get 5
                  i32.const 2
                  i32.shl
                  i32.store offset=28
                  i32.const 4
                  local.set 3
                end
                local.get 0
                local.get 3
                i32.store offset=24
                local.get 0
                i32.const 8
                i32.add
                local.get 7
                local.get 0
                i32.const 20
                i32.add
                call $_ZN5alloc7raw_vec11finish_grow17h108e4e2a3be76ba7E
                local.get 0
                i32.load offset=8
                i32.const 1
                i32.eq
                br_if 3 (;@3;)
                local.get 0
                i32.load offset=12
                local.set 3
                local.get 5
                local.set 7
                local.get 6
                local.set 5
                br 1 (;@5;)
              end
              local.get 4
              local.set 7
              local.get 4
              local.get 5
              i32.ge_u
              br_if 2 (;@3;)
            end
            local.get 3
            local.get 7
            i32.const 2
            i32.shl
            i32.add
            local.get 4
            i32.const 1
            i32.add
            i32.store
            local.get 7
            i32.const 1
            i32.add
            local.set 4
          end
          local.get 2
          local.get 4
          i32.ge_u
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          i32.const 2
          i32.shl
          i32.add
          i32.load
          local.set 7
          i32.const 0
          local.get 1
          i32.store offset=1048968
          i32.const 0
          local.get 7
          i32.store offset=1048964
          i32.const 0
          local.get 4
          i32.store offset=1048960
          i32.const 0
          i32.load offset=1048956
          local.set 7
          i32.const 0
          local.get 3
          i32.store offset=1048956
          i32.const 0
          i32.load offset=1048952
          local.set 4
          i32.const 0
          local.get 5
          i32.store offset=1048952
          block ;; label = @4
            local.get 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 7
            i32.const -4
            i32.add
            i32.load
            local.tee 5
            i32.const -8
            i32.and
            local.tee 3
            i32.const 4
            i32.const 8
            local.get 5
            i32.const 3
            i32.and
            local.tee 5
            select
            local.get 4
            i32.const 2
            i32.shl
            local.tee 4
            i32.add
            i32.lt_u
            br_if 2 (;@2;)
            block ;; label = @5
              local.get 5
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              local.get 4
              i32.const 39
              i32.add
              i32.gt_u
              br_if 4 (;@1;)
            end
            local.get 7
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17hd371afab6b2bc9caE
          end
          local.get 0
          i32.const 32
          i32.add
          global.set $__stack_pointer
          local.get 1
          local.get 2
          i32.add
          return
        end
        unreachable
      end
      i32.const 1048617
      i32.const 1048664
      call $_ZN4core9panicking5panic17hfef8090705073b49E
      unreachable
    end
    i32.const 1048680
    i32.const 1048728
    call $_ZN4core9panicking5panic17hfef8090705073b49E
    unreachable
  )
  (func $_ZN4core3ops8function6FnOnce9call_once17he41aae0b83180893E (;27;) (type 1) (param i32)
    local.get 0
    i32.const 0
    i32.store offset=16
    local.get 0
    i64.const 0
    i64.store offset=8 align=4
    local.get 0
    i64.const 17179869184
    i64.store align=4
  )
  (func $__externref_table_dealloc (;28;) (type 1) (param i32)
    block ;; label = @1
      local.get 0
      i32.const 132
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      call $_ZN12wasm_bindgen9externref35__wbindgen_externref_table_set_null17hf10811798d139b72E
      local.get 0
      call $_ZN12wasm_bindgen4__rt21LazyCell$LT$T$C$F$GT$8try_with17ha146a081a81b8b1bE
    end
  )
  (func $__externref_drop_slice (;29;) (type 0) (param i32 i32)
    (local i32)
    block ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 2
      i32.shl
      local.set 1
      loop ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load
          local.tee 2
          i32.const 132
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          call $_ZN12wasm_bindgen9externref35__wbindgen_externref_table_set_null17hf10811798d139b72E
          local.get 2
          call $_ZN12wasm_bindgen4__rt21LazyCell$LT$T$C$F$GT$8try_with17ha146a081a81b8b1bE
        end
        local.get 0
        i32.const 4
        i32.add
        local.set 0
        local.get 1
        i32.const -4
        i32.add
        local.tee 1
        br_if 0 (;@2;)
      end
    end
  )
  (func $__externref_heap_live_count (;30;) (type 8) (result i32)
    (local i32 i32 i32 i32)
    call $_ZN9once_cell6unsync17OnceCell$LT$T$GT$15get_or_try_init17h32f120d39a5cc659E
    i32.const 0
    local.set 0
    block ;; label = @1
      i32.const 0
      i32.load offset=1048964
      local.tee 1
      i32.const 0
      i32.load offset=1048960
      local.tee 2
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1048956
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @2
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 3
        local.get 1
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.tee 1
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 2
    local.get 0
    i32.sub
  )
  (func $memcpy (;31;) (type 9) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 4
        i32.add
        local.tee 5
        local.get 0
        i32.le_u
        br_if 0 (;@2;)
        local.get 4
        i32.const -1
        i32.add
        local.set 6
        local.get 0
        local.set 3
        local.get 1
        local.set 7
        block ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.set 8
          local.get 0
          local.set 3
          local.get 1
          local.set 7
          loop ;; label = @4
            local.get 3
            local.get 7
            i32.load8_u
            i32.store8
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            local.get 8
            i32.const -1
            i32.add
            local.tee 8
            br_if 0 (;@4;)
          end
        end
        local.get 6
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
        loop ;; label = @3
          local.get 3
          local.get 7
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.get 7
          i32.const 1
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 2
          i32.add
          local.get 7
          i32.const 2
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 3
          i32.add
          local.get 7
          i32.const 3
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 4
          i32.add
          local.get 7
          i32.const 4
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 5
          i32.add
          local.get 7
          i32.const 5
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 6
          i32.add
          local.get 7
          i32.const 6
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 7
          i32.add
          local.get 7
          i32.const 7
          i32.add
          i32.load8_u
          i32.store8
          local.get 7
          i32.const 8
          i32.add
          local.set 7
          local.get 3
          i32.const 8
          i32.add
          local.tee 3
          local.get 5
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 8
      i32.const -4
      i32.and
      local.tee 6
      i32.add
      local.set 3
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 7
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 5
          local.get 3
          i32.ge_u
          br_if 1 (;@2;)
          local.get 7
          local.set 1
          loop ;; label = @4
            local.get 5
            local.get 1
            i32.load
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 5
        local.get 3
        i32.ge_u
        br_if 0 (;@2;)
        local.get 7
        i32.const 3
        i32.shl
        local.tee 2
        i32.const 24
        i32.and
        local.set 4
        local.get 7
        i32.const -4
        i32.and
        local.tee 9
        i32.const 4
        i32.add
        local.set 1
        i32.const 0
        local.get 2
        i32.sub
        i32.const 24
        i32.and
        local.set 10
        local.get 9
        i32.load
        local.set 2
        loop ;; label = @3
          local.get 5
          local.get 2
          local.get 4
          i32.shr_u
          local.get 1
          i32.load
          local.tee 2
          local.get 10
          i32.shl
          i32.or
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 8
      i32.const 3
      i32.and
      local.set 2
      local.get 7
      local.get 6
      i32.add
      local.set 1
    end
    block ;; label = @1
      local.get 3
      local.get 3
      local.get 2
      i32.add
      local.tee 5
      i32.ge_u
      br_if 0 (;@1;)
      local.get 2
      i32.const -1
      i32.add
      local.set 8
      block ;; label = @2
        local.get 2
        i32.const 7
        i32.and
        local.tee 7
        i32.eqz
        br_if 0 (;@2;)
        loop ;; label = @3
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 7
          i32.const -1
          i32.add
          local.tee 7
          br_if 0 (;@3;)
        end
      end
      local.get 8
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      loop ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.get 1
        i32.const 1
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 2
        i32.add
        local.get 1
        i32.const 2
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 3
        i32.add
        local.get 1
        i32.const 3
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 4
        i32.add
        local.get 1
        i32.const 4
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 5
        i32.add
        local.get 1
        i32.const 5
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 6
        i32.add
        local.get 1
        i32.const 6
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 7
        i32.add
        local.get 1
        i32.const 7
        i32.add
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 8
        i32.add
        local.set 1
        local.get 3
        i32.const 8
        i32.add
        local.tee 3
        local.get 5
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (data $.rodata (;0;) (i32.const 1048576) "/rust/deps/dlmalloc-0.2.7/src/dlmalloc.rsassertion failed: psize >= size + min_overhead\00\00\00\10\00)\00\00\00\a8\04\00\00\09\00\00\00assertion failed: psize <= size + max_overhead\00\00\00\00\10\00)\00\00\00\ae\04\00\00\0d\00\00\00Lazy instance has previously been poisoned\00\00\a8\00\10\00*\00\00\00/Users/lupus/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.20.2/src/lib.rs\00\00\00\dc\00\10\00]\00\00\00\08\03\00\00\19\00\00\00reentrant init\00\00L\01\10\00\0e\00\00\00\dc\00\10\00]\00\00\00z\02\00\00\0d\00\00\00")
  (data $.data (;1;) (i32.const 1048948) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.84.0 (9fc6b4312 2025-01-07)")
  )
  (@custom "target_features" (after data) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)
