fn main() -> Result<(), Box<dyn std::error::Error>> {
    use codec::{Decode, Encode};
    use sp_core::traits::RuntimeCode;
    use sp_core::{traits::CodeExecutor, NativeOrEncoded};
    use sp_externalities::Externalities;
    use sp_runtime::traits::BlakeTwo256;
    use sp_state_machine::Backend;
    use sp_state_machine::OverlayedChanges;
    use sp_state_machine::TrieBackend; // only needed for jupyter
    //use memory_db::MemoryDB;

    const STATE_VERSION: sp_core::storage::StateVersion = sp_core::storage::StateVersion::V1;

    /// We use this trait so we just need to define a new instance in the following code snippets
    pub trait SimpleRun: Sized + Send + Sync + Clone + 'static {
        // return block number
        fn run_runtime(&self, method: &str, ext: &mut dyn Externalities) -> (u32, Vec<u8>);
    }

    #[derive(Clone)]
    struct FakeCodeExecutor<SR>(SR);

    impl<SR: SimpleRun> sp_core::traits::ReadRuntimeVersion for FakeCodeExecutor<SR> {
        fn read_runtime_version(
            &self,
            _wasm_code: &[u8],
            _ext: &mut dyn Externalities,
        ) -> Result<Vec<u8>, String> {
            panic!("Fake implementation.")
        }
    }

    impl<SR: SimpleRun> CodeExecutor for FakeCodeExecutor<SR> {
        /// just some static string.
        type Error = &'static str;

        fn call<
            R: codec::Codec + PartialEq,
            NC: FnOnce() -> Result<R, Box<dyn std::error::Error + Send + Sync>>
                + std::panic::UnwindSafe,
        >(
            &self,
            ext: &mut dyn Externalities,
            _runtime_code: &RuntimeCode,
            method: &str,
            _data: &[u8],
            _use_native: bool,
            _native_call: Option<NC>,
        ) -> (Result<NativeOrEncoded<R>, Self::Error>, bool) {
            let result = self.0.run_runtime(method, ext);
            (Ok(NativeOrEncoded::Encoded(result.encode())), true)
        }
    }

    // in memory persistence
    let mut backend = sp_state_machine::new_in_mem::<sp_core::Blake2Hasher>();
    //let mut backend = sp_state_machine::new_in_mem::<sp_core::Blake2Hasher, sp_trie::HashKey<_>>();
    let mut change_overlay = OverlayedChanges::default();

    #[derive(Clone)]
    struct Example1;

    impl SimpleRun for Example1 {
        fn run_runtime(&self, _method: &str, ext: &mut dyn Externalities) -> (u32, Vec<u8>) {
            // block number usually is in init_block of system
            let previous_block: u32 = ext
                .storage(b":block_number")
                .and_then(|encoded| Decode::decode(&mut encoded.as_slice()).ok())
                .unwrap_or(0);
            let current_block = previous_block + 1;
            ext.set_storage(b":block_number".to_vec(), current_block.encode());

            // finalize block
            let final_root = ext.storage_root(STATE_VERSION);
            (current_block, final_root)
        }
    }

    let executor = FakeCodeExecutor(Example1);

    for i in 1u32..5 {
        // simulate executing a block (collator)
        let remote_root = backend.storage_root(std::iter::empty(), STATE_VERSION).0;
        let (remote_result, remote_proof) = sp_state_machine::prove_execution(
            &mut backend,
            &mut change_overlay,
            &executor,
            sp_core::testing::TaskExecutor::new(),
            "test",
            &[],
            &RuntimeCode::empty(),
        )
        .unwrap();

        // simulate verifying a block execution (pvf)
        let checked_result = sp_state_machine::execution_proof_check::<BlakeTwo256, _, _>(
            remote_root,
            remote_proof,
            &mut Default::default(),
            &executor,
            sp_core::testing::TaskExecutor::new(),
            "test",
            &[],
            &RuntimeCode::empty(),
        )
        .unwrap();

        let (block_number, _root): (u32, Vec<u8>) =
            Decode::decode(&mut remote_result.as_slice()).unwrap();
        assert_eq!(block_number, i);
        assert_eq!(remote_result, checked_result);

        // flush to backend TODO consider done live
        // Note that when running try using assimilate: more correct
        let changes = change_overlay.drain_storage_changes(
            &backend,
            Default::default(), // ignore block hash
            &mut Default::default(),
            STATE_VERSION,
        )?;
        backend.apply_transaction(changes.transaction_storage_root, changes.transaction);
    }

    // 1. Write a function run_block for running n blocks with any SimpleRuntime logic. The function should return the given struct and use as input parameter a given number of blocks.

    /// stats for a given block execution.
    /// Can be displayed and compared.
    #[derive(Debug, PartialEq, Eq)]
    pub struct BlockExecutionStats {
			/// height
			block_number: u32,
			/// Size of proof
			proof_size: usize,
			/// Size of compact proof (lookup in susbstrate for `encode_compact`).
			compact_proof_size: usize,
			/// Size of compressed proof (from compact with zsh).
			compressed_proof_size: usize,
			/// Size of the full db. Facultative.
			full_db_size: usize,
			/// Size of the last db state (reflect more the state of a parachain with pruning). Facultative.
			last_state_db_size: usize,
    }

    Ok(())
}
