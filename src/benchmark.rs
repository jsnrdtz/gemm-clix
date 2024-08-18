extern "C" {
    fn runBenchmark(challenge: *const u8, total_nonces: *mut u64, num_threads: i32, duration: i32);
}

impl Miner {
    pub async fn benchmark(&self, args: BenchmarkArgs) {
        // Проверка количества потоков
        self.check_num_cores(args.cores);

        // Запуск задачи на GPU
        let challenge = [0; 32];
        let mut total_nonces = vec![0u64; args.cores as usize];

        // Вызов функции CUDA
        unsafe {
            runBenchmark(
                challenge.as_ptr(),
                total_nonces.as_mut_ptr(),
                args.cores as i32,
                TEST_DURATION as i32,
            );
        }

        // Расчет и вывод результатов
        let total_nonces: u64 = total_nonces.iter().sum();
        let hashpower = total_nonces.saturating_div(TEST_DURATION as u64);
        println!("Hashpower: {} H/sec", hashpower);
    }
}
